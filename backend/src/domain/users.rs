use super::eq;
use crate::{api::User, authorization};
use diesel::prelude::*;

pub fn register(
    conn: &mut PgConnection,
    display_name: &str,
    email_address: &str,
    password: &str,
) -> QueryResult<i32> {
    use crate::schema::users as u;

    let password_hash = authorization::hash_password(password.as_bytes());
    diesel::insert_into(u::table)
        .values(eq!(u::display_name, u::email_address, u::password_hash))
        .returning(u::id)
        .get_result(conn)
}

pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    use crate::schema::users as u;

    u::table.select(User::as_select()).load(conn)
}

pub fn find(conn: &mut PgConnection, id: i32) -> QueryResult<User> {
    use crate::schema::users as u;

    u::table
        .select(User::as_select())
        .filter(eq!(u::id))
        .get_result(conn)
}

pub fn follow(
    conn: &mut PgConnection,
    follower_user_id: i32,
    followed_user_id: i32,
) -> QueryResult<()> {
    use crate::schema::user_follows as uf;

    diesel::insert_into(uf::table)
        .values(eq!(uf::follower_user_id, uf::followed_user_id))
        .execute(conn)?;
    Ok(())
}

pub fn unfollow(
    conn: &mut PgConnection,
    follower_user_id: i32,
    followed_user_id: i32,
) -> QueryResult<()> {
    use crate::schema::user_follows as uf;

    diesel::delete(uf::table.filter(eq!(uf::follower_user_id).and(eq!(uf::followed_user_id))))
        .execute(conn)?;
    Ok(())
}

pub fn set_role(conn: &mut PgConnection, id: i32, role: &str) -> QueryResult<()> {
    use crate::schema::users as u;

    diesel::update(u::table)
        .set(eq!(u::role))
        .filter(eq!(u::id))
        .execute(conn)?;
    Ok(())
}

pub fn hide_tag(conn: &mut PgConnection, user_id: i32, tag_id: i32) -> QueryResult<()> {
    use crate::schema::user_hidden_tags as uht;

    diesel::insert_into(uht::table)
        .values(eq!(uht::user_id, uht::tag_id))
        .execute(conn)?;
    Ok(())
}

pub fn mute(conn: &mut PgConnection, muter_id: i32, mutee_id: i32) -> QueryResult<()> {
    use crate::schema::user_muted_users as umu;

    diesel::insert_into(umu::table)
        .values(eq!(umu::muter_id, umu::mutee_id))
        .execute(conn)?;
    Ok(())
}

pub fn block(conn: &mut PgConnection, blocker_id: i32, blockee_id: i32) -> QueryResult<()> {
    use crate::schema::user_blocked_users as ubu;

    diesel::insert_into(ubu::table)
        .values(eq!(ubu::blocker_id, ubu::blockee_id))
        .execute(conn)?;
    Ok(())
}

pub fn report(conn: &mut PgConnection, reporter_id: i32, user_id: i32) -> QueryResult<i32> {
    use crate::schema::reports as r;

    diesel::insert_into(r::table)
        .values(eq!(r::reporter_id, r::user_id))
        .returning(r::id)
        .get_result(conn)
}
