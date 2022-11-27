use super::eq;
use diesel::prelude::*;

pub fn new(
    conn: &mut PgConnection,
    url_slug: &str,
    owner_id: Option<i32>,
    nsfw: bool,
    alias_name: &str,
    language: Option<&str>,
    description: Option<&str>,
) -> QueryResult<(i32, i32, i32)> {
    use crate::schema::{tag_aliases as ta, tag_details as td, tags as t};

    let main = true;
    let canonical = true;
    conn.transaction(|conn| {
        let tag_id = diesel::insert_into(t::table)
            .values(eq!(t::url_slug, t::owner_id, t::nsfw))
            .returning(t::id)
            .get_result(conn)?;

        let alias_id = diesel::insert_into(ta::table)
            .values(eq!(
                ta::tag_id,
                ta::alias_name,
                ta::language,
                ta::main,
                ta::canonical
            ))
            .returning(ta::id)
            .get_result(conn)?;

        let details_id = diesel::insert_into(td::table)
            .values(eq!(td::tag_id, td::description, td::language))
            .returning(td::id)
            .get_result(conn)?;

        QueryResult::<_>::Ok((tag_id, alias_id, details_id))
    })
}

pub fn set_image(conn: &mut PgConnection, id: i32, image_id: Option<i32>) -> QueryResult<()> {
    use crate::schema::tags as t;

    diesel::update(t::table)
        .set(eq!(t::image_id))
        .filter(eq!(t::id))
        .execute(conn)?;
    Ok(())
}

pub fn connect(conn: &mut PgConnection, tag_id: i32, auto_tag_id: i32) -> QueryResult<()> {
    use crate::schema::tag_auto_tags as tat;

    diesel::insert_into(tat::table)
        .values(eq!(tat::tag_id, tat::auto_tag_id))
        .execute(conn)?;
    Ok(())
}

pub fn add_alias(
    conn: &mut PgConnection,
    tag_id: i32,
    alias_name: &str,
    language: Option<&str>,
) -> QueryResult<()> {
    use crate::schema::tag_aliases as ta;

    diesel::insert_into(ta::table)
        .values(eq!(ta::tag_id, ta::alias_name, ta::language))
        .execute(conn)?;
    Ok(())
}

pub fn add_details(
    conn: &mut PgConnection,
    tag_id: i32,
    differentiator: Option<&str>,
    description: Option<&str>,
    language: &str,
) -> QueryResult<()> {
    use crate::schema::tag_details as td;

    diesel::insert_into(td::table)
        .values(eq!(
            td::tag_id,
            td::differentiator,
            td::description,
            td::language
        ))
        .execute(conn)?;
    Ok(())
}
