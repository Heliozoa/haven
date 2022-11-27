use super::eq;
use diesel::{prelude::*, PgConnection};

pub fn post(
    conn: &mut PgConnection,
    user_id: i32,
    title: &str,
    text: &str,
    nsfw: bool,
) -> QueryResult<i32> {
    use crate::schema::blogs as b;

    diesel::insert_into(b::table)
        .values(eq!(b, user_id, title, text, nsfw))
        .returning(b::id)
        .get_result(conn)
}

pub fn like(conn: &mut PgConnection, user_id: i32, blog_id: i32) -> QueryResult<()> {
    use crate::schema::blog_likes as bl;

    diesel::insert_into(bl::table)
        .values(eq!(bl, user_id, blog_id))
        .execute(conn)?;
    Ok(())
}

pub fn unlike(conn: &mut PgConnection, user_id: i32, blog_id: i32) -> QueryResult<()> {
    use crate::schema::blog_likes as bl;

    diesel::delete(bl::table.filter(eq!(bl::user_id).and(eq!(bl::blog_id)))).execute(conn)?;
    Ok(())
}

pub fn comment(conn: &mut PgConnection, user_id: i32, blog_id: i32, text: &str) -> QueryResult<()> {
    use crate::schema::blog_comments as bc;

    diesel::insert_into(bc::table)
        .values(eq!(bc, user_id, blog_id, text))
        .execute(conn)?;
    Ok(())
}

pub fn delete_comment(conn: &mut PgConnection, id: i32) -> QueryResult<()> {
    use crate::schema::blog_comments as bc;

    diesel::delete(bc::table.filter(eq!(bc::id))).execute(conn)?;
    Ok(())
}
