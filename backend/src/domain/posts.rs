use super::eq;
use diesel::prelude::*;

pub struct NewPost<'a> {
    pub title: &'a str,
    pub text: &'a str,
    pub thumbnail_filename: &'a str,
    pub images: &'a [NewImage<'a>],
    pub tag_ids: &'a [i32],
    pub nsfw: bool,
}

pub struct NewImage<'a> {
    pub filename: &'a str,
    pub order_no: i32,
    pub width: i32,
    pub height: i32,
}

pub fn post(
    conn: &mut PgConnection,
    user_id: i32,
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
            .cloned()
            .map(|tag_id| eq!(pt::post_id, pt::tag_id))
            .collect::<Vec<_>>();
        diesel::insert_into(pt::table).values(tags).execute(conn)?;

        QueryResult::Ok(post_id)
    })
}

pub fn like(conn: &mut PgConnection, user_id: i32, post_id: i32) -> QueryResult<()> {
    use crate::schema::post_likes as pl;

    diesel::insert_into(pl::table)
        .values(eq!(pl::user_id, pl::post_id))
        .execute(conn)?;
    Ok(())
}

pub fn unlike(conn: &mut PgConnection, user_id: i32, post_id: i32) -> QueryResult<()> {
    use crate::schema::post_likes as pl;

    diesel::delete(pl::table.filter(eq!(pl::user_id).and(eq!(pl::post_id)))).execute(conn)?;
    Ok(())
}

pub fn tag(conn: &mut PgConnection, post_id: i32, tag_id: i32) -> QueryResult<()> {
    use crate::schema::post_tags as pt;

    diesel::insert_into(pt::table)
        .values(eq!(pt::post_id, pt::tag_id))
        .execute(conn)?;
    Ok(())
}

pub fn untag(conn: &mut PgConnection, post_id: i32, tag_id: i32) -> QueryResult<()> {
    use crate::schema::post_tags as pt;

    diesel::delete(pt::table.filter(eq!(pt::post_id).and(eq!(pt::tag_id)))).execute(conn)?;
    Ok(())
}

pub fn comment(conn: &mut PgConnection, user_id: i32, post_id: i32, text: &str) -> QueryResult<()> {
    use crate::schema::post_comments as pc;

    diesel::insert_into(pc::table)
        .values(eq!(pc::user_id, pc::post_id, pc::text))
        .execute(conn)?;
    Ok(())
}

pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<()> {
    use crate::schema::posts as p;

    diesel::delete(p::table.filter(eq!(p::id))).execute(conn)?;
    Ok(())
}

pub fn delete_comment(conn: &mut PgConnection, id: i32) -> QueryResult<()> {
    use crate::schema::post_comments as pc;

    diesel::delete(pc::table.filter(eq!(pc::id))).execute(conn)?;
    Ok(())
}
