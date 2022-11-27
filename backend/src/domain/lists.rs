use super::eq;
use diesel::prelude::*;

pub fn create(conn: &mut PgConnection, user_id: i32, name: &str) -> QueryResult<i32> {
    use crate::schema::lists as l;

    diesel::insert_into(l::table)
        .values(eq!(l::user_id, l::name))
        .returning(l::id)
        .get_result(conn)
}

pub fn add_post(conn: &mut PgConnection, list_id: i32, post_id: i32) -> QueryResult<()> {
    use crate::schema::list_posts as lp;

    diesel::insert_into(lp::table)
        .values(eq!(lp::list_id, lp::post_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_post(conn: &mut PgConnection, list_id: i32, post_id: i32) -> QueryResult<()> {
    use crate::schema::list_posts as lp;

    diesel::delete(lp::table.filter(eq!(lp::list_id).and(eq!(lp::post_id)))).execute(conn)?;
    Ok(())
}

pub fn add_comic(conn: &mut PgConnection, list_id: i32, comic_id: i32) -> QueryResult<()> {
    use crate::schema::list_comics as lc;

    diesel::insert_into(lc::table)
        .values(eq!(lc::list_id, lc::comic_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_comic(conn: &mut PgConnection, list_id: i32, comic_id: i32) -> QueryResult<()> {
    use crate::schema::list_comics as lc;

    diesel::delete(lc::table.filter(eq!(lc::list_id).and(eq!(lc::comic_id)))).execute(conn)?;
    Ok(())
}

pub fn add_blog(conn: &mut PgConnection, list_id: i32, blog_id: i32) -> QueryResult<()> {
    use crate::schema::list_blogs as lb;

    diesel::insert_into(lb::table)
        .values(eq!(lb::list_id, lb::blog_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_blog(conn: &mut PgConnection, list_id: i32, blog_id: i32) -> QueryResult<()> {
    use crate::schema::list_blogs as lb;

    diesel::delete(lb::table.filter(eq!(lb::list_id).and(eq!(lb::blog_id)))).execute(conn)?;
    Ok(())
}

pub fn add_tag(conn: &mut PgConnection, list_id: i32, tag_id: i32) -> QueryResult<()> {
    use crate::schema::list_tags as lt;

    diesel::insert_into(lt::table)
        .values(eq!(lt::list_id, lt::tag_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_tag(conn: &mut PgConnection, list_id: i32, tag_id: i32) -> QueryResult<()> {
    use crate::schema::list_tags as lt;

    diesel::delete(lt::table.filter(eq!(lt::list_id).and(eq!(lt::tag_id)))).execute(conn)?;
    Ok(())
}

pub fn add_user(conn: &mut PgConnection, list_id: i32, user_id: i32) -> QueryResult<()> {
    use crate::schema::list_users as lu;

    diesel::insert_into(lu::table)
        .values(eq!(lu::list_id, lu::user_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_user(conn: &mut PgConnection, list_id: i32, user_id: i32) -> QueryResult<()> {
    use crate::schema::list_users as lu;

    diesel::delete(lu::table.filter(eq!(lu::list_id).and(eq!(lu::user_id)))).execute(conn)?;
    Ok(())
}

pub fn add_post_comment(
    conn: &mut PgConnection,
    list_id: i32,
    post_comment_id: i32,
) -> QueryResult<()> {
    use crate::schema::list_post_comments as pc;

    diesel::insert_into(pc::table)
        .values(eq!(pc::list_id, pc::post_comment_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_post_comment(
    conn: &mut PgConnection,
    list_id: i32,
    post_comment_id: i32,
) -> QueryResult<()> {
    use crate::schema::list_post_comments as pc;

    diesel::delete(pc::table.filter(eq!(pc::list_id).and(eq!(pc::post_comment_id))))
        .execute(conn)?;
    Ok(())
}

pub fn add_blog_comment(
    conn: &mut PgConnection,
    list_id: i32,
    blog_comment_id: i32,
) -> QueryResult<()> {
    use crate::schema::list_blog_comments as bc;

    diesel::insert_into(bc::table)
        .values(eq!(bc::list_id, bc::blog_comment_id))
        .execute(conn)?;
    Ok(())
}

pub fn remove_blog_comment(
    conn: &mut PgConnection,
    list_id: i32,
    blog_comment_id: i32,
) -> QueryResult<()> {
    use crate::schema::list_blog_comments as bc;

    diesel::delete(bc::table.filter(eq!(bc::list_id).and(eq!(bc::blog_comment_id))))
        .execute(conn)?;
    Ok(())
}
