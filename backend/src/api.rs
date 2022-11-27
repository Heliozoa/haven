//! Contains types for interacting with the frontend.

use crate::query;
use chrono::{DateTime, Utc};
use elm_rs::{Elm, ElmDecode, ElmEncode};
use serde::{Deserialize, Serialize};
use std::fmt;

query! {
    #[derive(Debug, Serialize, Elm, ElmDecode)]
    pub struct PostSmall {
        pub id: i32 = posts::id,
        pub thumbnail: String = posts::thumbnail_filename,
        pub title: String = posts::title,
        pub user_id: i32 = users::id,
        pub username: String = users::display_name,
    }
}

#[derive(Debug, Serialize, Elm, ElmDecode)]
pub struct PostFull {
    pub id: i32,
    pub images: Vec<Image>,
    pub tags: Vec<TagEmbed>,
    pub title: String,
    pub text: String,
    pub user_id: i32,
    pub username: String,
    pub avatar: String,
    pub comments: Vec<Comment>,
}

query! {
    #[derive(Debug, Serialize, Elm, ElmDecode)]
    pub struct Image {
        pub id: i32 = images::id,
        pub filename: String = images::filename,
        pub height: i32 = images::height,
        pub width: i32 = images::width,
    }
}

query! {
    #[derive(Debug, Serialize, Elm, ElmDecode)]
    pub struct Comment {
        pub id: i32 = post_comments::id,
        pub user_id: i32 = users::id,
        pub username: String = users::display_name,
        pub text: String = post_comments::text,
        pub avatar: String = users::avatar,
    }
}

query! {
    #[derive(Debug, Serialize, Elm, ElmDecode)]
    pub struct TagEmbed {
        pub id: i32 = tags::id,
        pub alias_name: String = tag_aliases::alias_name,
        pub slug: String = tags::url_slug,
        pub differentiator: Option<String> = tag_details::differentiator,
        pub image: Option<i32> = tags::image_id,
    }
}

query! {
    #[derive(Debug, Serialize, Elm, ElmEncode, ElmDecode)]
    pub struct User {
        pub id: i32 = users::id,
        pub name: String = users::display_name,
    }
}

#[derive(Debug, Serialize, Elm, ElmDecode)]
pub struct UserView {
    pub id: i32,
    pub name: String,
    pub avatar: String,
    pub profile: String,
    pub created: DateTime<Utc>,
    pub following: bool,
    pub posts: Vec<PostSmall>,
}

#[derive(Debug, Deserialize, Elm, ElmEncode)]
pub struct UserUpdate {
    pub display_name: String,
    pub profile_text: String,
}

#[derive(Debug, Serialize, Elm, ElmDecode)]
pub struct Tag {
    pub id: i32,
    pub alias: String,
    pub slug: String,
    pub differentiator: Option<String>,
    pub description: Option<String>,
    pub image: Option<i32>,
}

#[derive(Debug, Serialize, Elm, ElmDecode)]
pub struct TagView {
    pub tag: Tag,
    pub posts: Vec<PostSmall>,
}

#[derive(Deserialize, Elm, ElmEncode)]
pub struct Login {
    pub email: String,
    pub password: String,
}

impl fmt::Debug for Login {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

#[derive(Deserialize, Elm, ElmEncode)]
pub struct Registration {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl fmt::Debug for Registration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

#[derive(Debug, Deserialize, Elm, ElmEncode)]
pub struct NewComment {
    pub text: String,
}

#[derive(Debug, Deserialize, Elm, ElmEncode)]
pub struct PostMetadata {
    pub title: String,
    pub post: String,
    pub tags: Vec<i32>,
    pub thumbnail: Thumbnail,
    pub nsfw: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Elm, ElmEncode)]
pub struct Thumbnail {
    pub idx: usize,
    pub x: u32,
    pub y: u32,
    pub size: u32,
}

#[derive(Debug, Serialize, Deserialize, Elm, ElmDecode)]
pub struct Error<'a> {
    pub message: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Elm, ElmDecode)]
pub struct Settings {
    pub dark_mode: bool,
}
