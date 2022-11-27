use crate::{
    api::{Comment, Image, NewComment, PostFull, PostMetadata, PostSmall, TagEmbed, Thumbnail},
    authorization::Authentication,
    domain::posts::{self, NewImage, NewPost},
    eq,
    error::{HavenError, HavenResult},
    query, static_files, Pool,
};
use axum::{
    body::Bytes,
    extract::{Multipart, Path},
    Json,
};
use diesel::{prelude::*, r2d2::ConnectionManager};
use image::{
    codecs::{jpeg::JpegDecoder, png::PngDecoder},
    imageops::FilterType,
    DynamicImage, ImageDecoder, ImageFormat,
};
use r2d2::PooledConnection;
use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
};

pub async fn list(pool: Pool) -> HavenResult<Json<Vec<PostSmall>>> {
    use crate::schema::{posts as p, users as u};
    let mut conn = pool.get()?;

    let posts = tokio::task::spawn_blocking(move || {
        p::table
            .inner_join(u::table)
            .select(PostSmall::as_select())
            .get_results(&mut conn)
    })
    .await??;
    Ok(Json(posts))
}

enum Format {
    Jpeg,
    Png,
}

impl Format {
    fn ext(&self) -> &'static str {
        match self {
            Self::Jpeg => ".jpg",
            Self::Png => ".png",
        }
    }
}

pub async fn new(pool: Pool, auth: Authentication, mut post: Multipart) -> HavenResult<String> {
    let conn = pool.get()?;

    let metadata = post
        .next_field()
        .await?
        .ok_or_else(|| HavenError::BadRequest("Empty upload"))?
        .bytes()
        .await?;
    let metadata: PostMetadata = serde_json::from_slice(&metadata)?;

    let id = match new_inner(conn, metadata, post, auth.user_id).await {
        Ok(id) => id,
        Err(err) => {
            // try to delete all written files
            for written_image in err.written_images {
                if written_image.exists() {
                    if let Err(err) = tokio::fs::remove_file(&written_image).await {
                        error!(
                            "Failed to remove image {}: {}",
                            written_image.display(),
                            err
                        );
                    }
                }
            }
            return Err(err.inner);
        }
    };
    Ok(id.to_string())
}

struct PostError {
    inner: HavenError,
    written_images: Vec<PathBuf>,
}

async fn new_inner(
    mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    mut metadata: PostMetadata,
    mut post: Multipart,
    user_id: i32,
) -> Result<i32, PostError> {
    let mut idx = 0;
    let mut new_images = vec![];
    let mut written_images = vec![];
    let mut thumbnail_filename = None;

    while let Some(image) = post.next_field().await.map_err(|e| PostError {
        inner: e.into(),
        written_images: vec![],
    })? {
        let ct = image.content_type().ok_or_else(|| PostError {
            inner: HavenError::BadRequest("Missing content type"),
            written_images: vec![],
        })?;
        let format = if ct == mime::IMAGE_JPEG {
            Format::Jpeg
        } else if ct == mime::IMAGE_PNG {
            Format::Png
        } else {
            return Err(PostError {
                inner: HavenError::InvalidFileUpload,
                written_images,
            });
        };
        let image = image.bytes().await.map_err(|e| PostError {
            inner: e.into(),
            written_images: vec![],
        })?;

        match tokio::task::spawn_blocking(move || {
            let handled = process_image(&metadata, idx, image, format)?;
            Result::<_, WriteError>::Ok((metadata, handled))
        })
        .await
        {
            Ok(Ok((returned, write))) => {
                new_images.push(write.image);
                thumbnail_filename = thumbnail_filename.or(write.thumbnail_filename);
                written_images.push(write.written_image);
                write.written_thumbnail.map(|wt| written_images.push(wt));
                metadata = returned
            }
            Ok(Err(err)) => {
                err.written_image.map(|wi| written_images.push(wi));
                err.written_thumbnail.map(|wt| written_images.push(wt));
                return Err(PostError {
                    inner: err.inner,
                    written_images: written_images.clone(),
                });
            }
            Err(e) => {
                return Err(PostError {
                    inner: e.into(),
                    written_images: vec![],
                })
            }
        }
        idx += 1;
    }

    let thumbnail_filename = thumbnail_filename.ok_or_else(|| PostError {
        inner: HavenError::BadRequest("Invalid thumbnail index"),
        written_images: written_images.clone(),
    })?;
    let post_id = tokio::task::spawn_blocking(move || {
        let images = new_images
            .iter()
            .map(
                |HandledImage {
                     idx,
                     filename,
                     width,
                     height,
                 }| NewImage {
                    filename,
                    order_no: *idx,
                    width: *width,
                    height: *height,
                },
            )
            .collect::<Vec<_>>();
        let new_post = NewPost {
            title: &metadata.title,
            text: &metadata.post,
            thumbnail_filename: &thumbnail_filename,
            images: &images,
            tag_ids: &metadata.tags,
            nsfw: metadata.nsfw,
        };
        let post_id = posts::post(&mut conn, user_id, new_post)?;
        QueryResult::Ok(post_id)
    })
    .await
    .map_err(|e| PostError {
        inner: e.into(),
        written_images: written_images.clone(),
    })?
    .map_err(|e| PostError {
        inner: e.into(),
        written_images,
    })?;

    Ok(post_id)
}

struct Write {
    image: HandledImage,
    thumbnail_filename: Option<String>,
    written_image: PathBuf,
    written_thumbnail: Option<PathBuf>,
}

struct HandledImage {
    idx: i32,
    filename: String,
    width: i32,
    height: i32,
}

struct WriteError {
    inner: HavenError,
    written_image: Option<PathBuf>,
    written_thumbnail: Option<PathBuf>,
}

fn process_image(
    metadata: &PostMetadata,
    idx: usize,
    image: Bytes,
    format: Format,
) -> Result<Write, WriteError> {
    let thumbnail = if metadata.thumbnail.idx == idx {
        Some(metadata.thumbnail)
    } else {
        None
    };

    let cursor = Cursor::new(&image);
    let (thumbnail_write, (width, height)) = {
        let res = match format {
            Format::Jpeg => {
                let decoder = JpegDecoder::new(BufReader::new(cursor)).map_err(|e| WriteError {
                    inner: e.into(),
                    written_image: None,
                    written_thumbnail: None,
                })?;
                get_dimensions_and_write_thumbnail(decoder, thumbnail).map_err(|e| WriteError {
                    inner: e.inner,
                    written_image: None,
                    written_thumbnail: None,
                })?
            }
            Format::Png => {
                let decoder = PngDecoder::new(BufReader::new(cursor)).map_err(|e| WriteError {
                    inner: e.into(),
                    written_image: None,
                    written_thumbnail: None,
                })?;
                get_dimensions_and_write_thumbnail(decoder, thumbnail).map_err(|e| WriteError {
                    inner: e.inner,
                    written_image: None,
                    written_thumbnail: e.written_thumbnail,
                })?
            }
        };
        res
    };

    let (path, filename) = static_files::generate_image_path(format.ext());
    let mut thumbnail_filename = None;
    let mut thumbnail_path = None;
    if let Some(thumbnail_write) = thumbnail_write {
        thumbnail_filename = Some(thumbnail_write.filename);
        thumbnail_path = Some(thumbnail_write.path);
    }
    std::fs::write(&path, &image).map_err(|e| WriteError {
        inner: e.into(),
        written_image: Some(path.clone()),
        written_thumbnail: thumbnail_path.clone(),
    })?;
    let image = HandledImage {
        idx: i32::try_from(idx).map_err(|e| WriteError {
            inner: e.into(),
            written_image: Some(path.clone()),
            written_thumbnail: thumbnail_path.clone(),
        })?,
        width: i32::try_from(width).map_err(|e| WriteError {
            inner: e.into(),
            written_image: Some(path.clone()),
            written_thumbnail: thumbnail_path.clone(),
        })?,
        height: i32::try_from(height).map_err(|e| WriteError {
            inner: e.into(),
            written_image: Some(path.clone()),
            written_thumbnail: thumbnail_path.clone(),
        })?,
        filename,
    };
    Ok(Write {
        image,
        thumbnail_filename,
        written_image: path,
        written_thumbnail: thumbnail_path,
    })
}

struct ThumbnailWrite {
    filename: String,
    path: PathBuf,
}

struct ThumbnailWriteError {
    inner: HavenError,
    written_thumbnail: Option<PathBuf>,
}

// convenience function to avoid duplicating code for different decoders
fn get_dimensions_and_write_thumbnail<'a, D: ImageDecoder<'a>>(
    decoder: D,
    thumbnail: Option<Thumbnail>,
) -> Result<(Option<ThumbnailWrite>, (u32, u32)), ThumbnailWriteError> {
    let d = decoder.dimensions();

    // if this file has a thumbnail, write it and store the path
    let thumbnail_filename = if let Some(thumbnail) = thumbnail {
        // process image to thumbnail
        let img = DynamicImage::from_decoder(decoder)
            .map_err(|e| ThumbnailWriteError {
                inner: e.into(),
                written_thumbnail: None,
            })?
            .crop_imm(thumbnail.x, thumbnail.y, thumbnail.size, thumbnail.size)
            .resize(512, 512, FilterType::Lanczos3);

        // save thumbnail and store path
        let (path, filename) = static_files::generate_thumbnail_path(".png");
        img.save_with_format(&path, ImageFormat::Png)
            .map_err(|e| ThumbnailWriteError {
                inner: e.into(),
                written_thumbnail: Some(path.clone()),
            })?;
        Some(ThumbnailWrite { filename, path })
    } else {
        None
    };
    Ok((thumbnail_filename, d))
}

query! {
    struct PostData {
        title: String = posts::title,
        text: String = posts::text,
        user_id: i32 = posts::user_id,
        username: String = users::display_name,
        avatar: String = users::avatar,
    }
}

pub async fn view(pool: Pool, Path(post_id): Path<i32>) -> HavenResult<Json<PostFull>> {
    use crate::schema::{
        images as i, post_tags as pt, posts as p, tag_aliases as ta, tag_details as td, tags as t,
        users as u,
    };
    let mut conn = pool.get()?;

    let res = tokio::task::spawn_blocking(move || {
        let images = i::table
            .filter(eq!(i::post_id))
            .order_by(i::order_no.asc())
            .select(Image::as_select())
            .get_results(&mut conn)?;
        let comments = comments(&mut conn, post_id)?;
        let tags = t::table
            .inner_join(td::table.on(t::id.eq(td::tag_id)))
            .inner_join(ta::table.on(t::id.eq(ta::tag_id)))
            .inner_join(pt::table.on(t::id.eq(ta::tag_id)))
            .filter(eq!(pt::post_id).and(pt::tag_id.eq(t::id)))
            .select(TagEmbed::as_select())
            .get_results(&mut conn)?;

        let id = post_id;
        let PostData {
            title,
            text,
            user_id,
            username,
            avatar,
        } = p::table
            .inner_join(u::table.on(u::id.eq(p::user_id)))
            .filter(eq!(p::id))
            .select(PostData::as_select())
            .get_result(&mut conn)?;

        let post = PostFull {
            id,
            comments,
            images,
            text,
            title,
            user_id,
            username,
            tags,
            avatar,
        };

        Result::<_, anyhow::Error>::Ok(post)
    })
    .await??;
    Ok(Json(res))
}

fn comments(conn: &mut PgConnection, post_id: i32) -> QueryResult<Vec<Comment>> {
    use crate::schema::{post_comments as pc, users as u};

    pc::table
        .inner_join(u::table.on(u::id.eq(pc::user_id).and(eq!(pc::post_id))))
        .select(Comment::as_select())
        .get_results(conn)
}

pub async fn comment(
    pool: Pool,
    Path(post_id): Path<i32>,
    auth: Authentication,
    Json(comment): Json<NewComment>,
) -> HavenResult<Json<Vec<Comment>>> {
    use crate::schema::post_comments as pc;
    let mut conn = pool.get()?;

    let user_id = auth.user_id;
    let text = comment.text;
    let comments = tokio::task::spawn_blocking(move || {
        diesel::insert_into(pc::table)
            .values(eq!(pc::post_id, pc::user_id, pc::text))
            .execute(&mut conn)?;

        let comments = comments(&mut conn, post_id)?;
        QueryResult::Ok(comments)
    })
    .await??;

    Ok(Json(comments))
}

pub async fn tag(
    pool: Pool,
    Path(post_id): Path<i32>,
    Path(tag_id): Path<i32>,
    _auth: Authentication,
) -> HavenResult<()> {
    use crate::schema::post_tags as pt;
    let mut conn = pool.get()?;

    tokio::task::spawn_blocking(move || {
        diesel::insert_into(pt::table)
            .values(eq!(pt::post_id, pt::tag_id))
            .execute(&mut conn)?;
        QueryResult::Ok(())
    })
    .await??;

    Ok(())
}

pub async fn untag(
    pool: Pool,
    Path(post_id): Path<i32>,
    Path(tag_id): Path<i32>,
    _auth: Authentication,
) -> HavenResult<()> {
    use crate::schema::post_tags as pt;
    let mut conn = pool.get()?;

    tokio::task::spawn_blocking(move || {
        diesel::delete(pt::table)
            .filter(eq!(pt::post_id).and(eq!(pt::tag_id)))
            .execute(&mut conn)?;
        QueryResult::Ok(())
    })
    .await??;

    Ok(())
}
