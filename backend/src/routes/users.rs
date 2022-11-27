use crate::{
    api::{self, PostSmall, UserUpdate},
    authorization::Authentication,
    eq,
    error::{HavenError, HavenResult},
    query, static_files, Pool,
};
use axum::{
    extract::{Multipart, Path},
    Json,
};
use chrono::{DateTime, Utc};
use diesel::{dsl::*, prelude::*};
use image::{codecs::png::PngDecoder, DynamicImage, ImageDecoder, ImageFormat};
use std::io::{BufReader, Cursor};

query! {
    #[derive(Debug)]
    pub struct UserView {
        pub id: i32 = users::id,
        pub name: String = users::display_name,
        pub avatar: String = users::avatar,
        pub profile: String = users::profile_text,
        pub created: DateTime<Utc> = users::created,
    }
}

pub async fn view(
    pool: Pool,
    Path(id): Path<i32>,
    auth: Option<Authentication>,
) -> HavenResult<Json<api::UserView>> {
    use crate::schema::{posts as p, user_follows as uf, users as u};
    let mut conn = pool.get()?;

    let user = tokio::task::spawn_blocking(move || {
        let user: UserView = u::table
            .find(id)
            .select(UserView::as_select())
            .first(&mut conn)?;

        let following = if let Some(auth) = auth {
            select(exists(
                uf::table.filter(
                    uf::followed_user_id
                        .eq(id)
                        .and(uf::follower_user_id.eq(auth.user_id)),
                ),
            ))
            .get_result(&mut conn)?
        } else {
            false
        };

        let posts = p::table
            .inner_join(u::table)
            .filter(eq!(u::id))
            .select(PostSmall::as_select())
            .get_results::<PostSmall>(&mut conn)?;

        QueryResult::Ok(api::UserView {
            id: user.id,
            name: user.name,
            avatar: user.avatar,
            profile: user.profile,
            created: user.created,
            following,
            posts,
        })
    })
    .await??;

    Ok(Json(user))
}

pub async fn follow(
    pool: Pool,
    Path(followed_user_id): Path<i32>,
    auth: Authentication,
) -> HavenResult<()> {
    use crate::schema::{user_follows as uf, users as u};
    let mut conn = pool.get()?;

    let id = auth.user_id;
    let follower_user_id = auth.user_id;
    tokio::task::spawn_blocking(move || {
        select(exists(u::table.filter(eq!(u::id)))).execute(&mut conn)?;
        diesel::insert_into(uf::table)
            .values(eq!(uf::follower_user_id, uf::followed_user_id))
            .execute(&mut conn)?;
        QueryResult::Ok(())
    })
    .await??;

    Ok(())
}

pub async fn unfollow(
    pool: Pool,
    Path(followed_user_id): Path<i32>,
    auth: Authentication,
) -> HavenResult<()> {
    use crate::schema::user_follows as uf;
    let mut conn = pool.get()?;

    let follower_user_id = auth.user_id;
    tokio::task::spawn_blocking(move || {
        diesel::delete(uf::table.filter(eq!(uf::follower_user_id).and(eq!(uf::followed_user_id))))
            .execute(&mut conn)?;
        QueryResult::Ok(())
    })
    .await??;

    Ok(())
}

pub async fn update(
    pool: Pool,
    Path(id): Path<i32>,
    _auth: Authentication,
    update: Json<UserUpdate>,
) -> HavenResult<()> {
    use crate::schema::users as u;
    let mut conn = pool.get()?;

    tokio::task::spawn_blocking(move || {
        let display_name = &update.display_name;
        let profile_text = &update.profile_text;
        diesel::update(u::table)
            .filter(eq!(u::id))
            .set(eq!(u::display_name, u::profile_text))
            .execute(&mut conn)?;
        QueryResult::Ok(())
    })
    .await??;

    Ok(())
}

pub async fn upload_avatar(_auth: Authentication, mut avatar: Multipart) -> HavenResult<()> {
    let avatar = avatar
        .next_field()
        .await?
        .ok_or_else(|| HavenError::BadRequest("No file"))?;
    let ct = avatar
        .content_type()
        .ok_or_else(|| HavenError::BadRequest("Missing content type"))?;
    if ct != mime::IMAGE_PNG {
        return Err(HavenError::BadRequest("Invalid content type"));
    }
    let avatar = avatar.bytes().await?;

    tokio::task::spawn_blocking(move || {
        let cursor = Cursor::new(&avatar);
        let decoder = PngDecoder::new(BufReader::new(cursor))?;
        let (width, height) = decoder.dimensions();
        if width != 128 || height != 128 {
            return Err(HavenError::BadRequest("Invalid dimensions"));
        }
        let avatar = DynamicImage::from_decoder(decoder)?;
        let (path, _filename) = static_files::generate_avatar_path(".png");

        // ! delete file on error after this point
        avatar
            .save_with_format(&path, ImageFormat::Png)
            .map_err(|e| delete_file(e, &path))?;
        Ok(())
    })
    .await??;

    Ok(())
}

fn delete_file<E: Into<HavenError>>(e: E, path: &std::path::Path) -> E {
    if let Err(err) = std::fs::remove_file(path) {
        error!("Failed to delete file {} {}", path.display(), err);
    }
    e
}
