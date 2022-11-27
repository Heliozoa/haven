use crate::{
    api::{PostSmall, Tag, TagView},
    error::HavenResult,
    HavenConn,
};

use diesel::prelude::*;
use rocket::serde::json::Json;

pub async fn new() {
    todo!()
}

pub async fn view(conn: HavenConn, tag_id: i32) -> HavenResult<Json<TagView>> {
    use crate::schema::{posts as p, tag_aliases as ta, tag_details as td, tags as t, users as u};

    let (tag, posts) = conn
        .run(move |c| {
            let (id, alias, slug, description, differentiator, image) = t::table
                .inner_join(td::table.on(td::tag_id.eq(tag_id)))
                .inner_join(ta::table.on(ta::tag_id.eq(tag_id)))
                .select((
                    t::id,
                    ta::alias,
                    t::url_slug,
                    td::description,
                    td::differentiator,
                    t::image_id,
                ))
                .filter(t::id.eq(tag_id))
                .get_result(c)?;
            let tag = Tag {
                id,
                alias,
                slug,
                description,
                differentiator,
                image,
            };
            let posts = p::table
                .inner_join(u::table.on(u::id.eq(p::user_id)))
                .select((
                    p::id,
                    p::thumbnail_filename,
                    p::title,
                    u::id,
                    u::display_name,
                ))
                .get_results(c)?;
            let posts = posts
                .into_iter()
                .map(|(id, thumbnail, title, user_id, username)| PostSmall {
                    id,
                    thumbnail,
                    title,
                    user_id,
                    username,
                })
                .collect();
            Result::<_, anyhow::Error>::Ok((tag, posts))
        })
        .await?;

    let tag_view = TagView { tag, posts };
    Ok(Json(tag_view))
}

pub async fn delete(tag_id: i32) {
    todo!()
}
