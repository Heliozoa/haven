use super::{eq, posts::NewImage};
use crate::domain::posts::NewPost;
use diesel::{prelude::*, PgConnection};

pub struct NewComic<'a> {
    title: &'a str,
    text: &'a str,
    thumbnail_filename: &'a str,
    tag_ids: &'a [i32],
    nsfw: bool,
    first_chapter: NewPost<'a>,
}

pub fn start(
    conn: &mut PgConnection,
    user_id: i32,
    new_comic: NewComic<'_>,
) -> QueryResult<(i32, i32)> {
    use crate::schema::{comic_tags as ct, comics as c};

    let (comic_id, chapter_id) = conn.transaction(|conn| {
        let NewComic {
            title,
            text,
            thumbnail_filename,
            tag_ids,
            nsfw,
            first_chapter,
        } = new_comic;
        let comic_id = diesel::insert_into(c::table)
            .values(eq!(
                c::user_id,
                c::title,
                c::text,
                c::thumbnail_filename,
                c::nsfw
            ))
            .returning(c::id)
            .get_result(conn)?;

        let values = tag_ids
            .iter()
            .map(|tag_id| eq!(ct::comic_id, ct::tag_id))
            .collect::<Vec<_>>();
        diesel::insert_into(ct::table)
            .values(values)
            .execute(conn)?;

        let chapter_id = post_chapter(conn, user_id, comic_id, 1, first_chapter)?;

        QueryResult::Ok((comic_id, chapter_id))
    })?;
    Ok((comic_id, chapter_id))
}

pub fn post_chapter(
    conn: &mut PgConnection,
    user_id: i32,
    comic_id: i32,
    chapter_no: i32,
    NewPost {
        title,
        text,
        thumbnail_filename,
        images,
        tag_ids,
        nsfw,
    }: NewPost<'_>,
) -> QueryResult<i32> {
    use crate::schema::{images as i, post_tags as pt, posts as p};

    conn.transaction(|conn| {
        let post_id = diesel::insert_into(p::table)
            .values(eq!(
                p::user_id,
                p::title,
                p::text,
                p::thumbnail_filename,
                p::nsfw,
                p::comic_id,
                p::chapter_no
            ))
            .returning(p::id)
            .get_result(conn)?;

        let images = images
            .iter()
            .map(
                |NewImage {
                     filename,
                     order_no,
                     width,
                     height,
                 }| eq!(i::post_id, i::filename, i::order_no, i::width, i::height),
            )
            .collect::<Vec<_>>();
        diesel::insert_into(i::table)
            .values(&images)
            .execute(conn)?;

        let tags = tag_ids
            .iter()
            .map(|tag_id| eq!(pt::post_id, pt::tag_id))
            .collect::<Vec<_>>();
        diesel::insert_into(pt::table).values(tags).execute(conn)?;

        QueryResult::Ok(post_id)
    })
}

pub fn like(conn: &mut PgConnection, user_id: i32, comic_id: i32) -> QueryResult<()> {
    use crate::schema::comic_likes as c;

    diesel::insert_into(c::table)
        .values(eq!(c::user_id, c::comic_id))
        .execute(conn)?;
    Ok(())
}

pub fn unlike(conn: &mut PgConnection, user_id: i32, comic_id: i32) -> QueryResult<()> {
    use crate::schema::comic_likes as c;

    diesel::delete(c::table.filter(eq!(c::user_id).and(eq!(c::comic_id)))).execute(conn)?;
    Ok(())
}

pub fn tag(conn: &mut PgConnection, tag_id: i32, comic_id: i32) -> QueryResult<()> {
    use crate::schema::comic_tags as c;

    diesel::insert_into(c::table)
        .values(eq!(c::tag_id, c::comic_id))
        .execute(conn)?;
    Ok(())
}

pub fn untag(conn: &mut PgConnection, tag_id: i32, comic_id: i32) -> QueryResult<()> {
    use crate::schema::comic_tags as c;

    diesel::delete(c::table.filter(eq!(c::tag_id).and(eq!(c::comic_id)))).execute(conn)?;
    Ok(())
}
