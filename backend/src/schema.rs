// @generated automatically by Diesel CLI.

diesel::table! {
    /// Representation of the `bans` table.
    ///
    /// (Automatically generated by Diesel.)
    bans (id) {
        /// The `id` column of the `bans` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `bans` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `banned_id` column of the `bans` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        banned_id -> Int4,
        /// The `banned_by_id` column of the `bans` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        banned_by_id -> Int4,
        /// The `reason` column of the `bans` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        reason -> Text,
        /// The `expires` column of the `bans` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        expires -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    /// Representation of the `blog_comment_likes` table.
    ///
    /// (Automatically generated by Diesel.)
    blog_comment_likes (user_id, blog_comment_id) {
        /// The `user_id` column of the `blog_comment_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `blog_comment_id` column of the `blog_comment_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blog_comment_id -> Int4,
        /// The `created` column of the `blog_comment_likes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `blog_comments` table.
    ///
    /// (Automatically generated by Diesel.)
    blog_comments (id) {
        /// The `id` column of the `blog_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `blog_comments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `blog_id` column of the `blog_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blog_id -> Int4,
        /// The `user_id` column of the `blog_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `text` column of the `blog_comments` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        text -> Text,
    }
}

diesel::table! {
    /// Representation of the `blog_likes` table.
    ///
    /// (Automatically generated by Diesel.)
    blog_likes (user_id, blog_id) {
        /// The `user_id` column of the `blog_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `blog_id` column of the `blog_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blog_id -> Int4,
        /// The `created` column of the `blog_likes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `blogs` table.
    ///
    /// (Automatically generated by Diesel.)
    blogs (id) {
        /// The `id` column of the `blogs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `blogs` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `user_id` column of the `blogs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `title` column of the `blogs` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `text` column of the `blogs` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        text -> Text,
        /// The `nsfw` column of the `blogs` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        nsfw -> Bool,
    }
}

diesel::table! {
    /// Representation of the `comic_likes` table.
    ///
    /// (Automatically generated by Diesel.)
    comic_likes (user_id, comic_id) {
        /// The `user_id` column of the `comic_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `comic_id` column of the `comic_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        comic_id -> Int4,
        /// The `created` column of the `comic_likes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `comic_tags` table.
    ///
    /// (Automatically generated by Diesel.)
    comic_tags (comic_id, tag_id) {
        /// The `comic_id` column of the `comic_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        comic_id -> Int4,
        /// The `tag_id` column of the `comic_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `created` column of the `comic_tags` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `comics` table.
    ///
    /// (Automatically generated by Diesel.)
    comics (id) {
        /// The `id` column of the `comics` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `comics` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `user_id` column of the `comics` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `title` column of the `comics` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `text` column of the `comics` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        text -> Text,
        /// The `thumbnail_filename` column of the `comics` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        thumbnail_filename -> Text,
        /// The `nsfw` column of the `comics` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        nsfw -> Bool,
    }
}

diesel::table! {
    /// Representation of the `images` table.
    ///
    /// (Automatically generated by Diesel.)
    images (id) {
        /// The `id` column of the `images` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `post_id` column of the `images` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Int4,
        /// The `filename` column of the `images` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        filename -> Text,
        /// The `order_no` column of the `images` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        order_no -> Int4,
        /// The `width` column of the `images` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        width -> Int4,
        /// The `height` column of the `images` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        height -> Int4,
    }
}

diesel::table! {
    /// Representation of the `list_blog_comments` table.
    ///
    /// (Automatically generated by Diesel.)
    list_blog_comments (list_id, blog_comment_id) {
        /// The `list_id` column of the `list_blog_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `blog_comment_id` column of the `list_blog_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blog_comment_id -> Int4,
        /// The `created` column of the `list_blog_comments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `list_blogs` table.
    ///
    /// (Automatically generated by Diesel.)
    list_blogs (list_id, blog_id) {
        /// The `list_id` column of the `list_blogs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `blog_id` column of the `list_blogs` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blog_id -> Int4,
        /// The `created` column of the `list_blogs` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `list_comics` table.
    ///
    /// (Automatically generated by Diesel.)
    list_comics (list_id, comic_id) {
        /// The `list_id` column of the `list_comics` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `comic_id` column of the `list_comics` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        comic_id -> Int4,
        /// The `created` column of the `list_comics` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `list_post_comments` table.
    ///
    /// (Automatically generated by Diesel.)
    list_post_comments (list_id, post_comment_id) {
        /// The `list_id` column of the `list_post_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `post_comment_id` column of the `list_post_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_comment_id -> Int4,
        /// The `created` column of the `list_post_comments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `list_posts` table.
    ///
    /// (Automatically generated by Diesel.)
    list_posts (list_id, post_id) {
        /// The `list_id` column of the `list_posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `post_id` column of the `list_posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Int4,
        /// The `created` column of the `list_posts` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `list_tags` table.
    ///
    /// (Automatically generated by Diesel.)
    list_tags (list_id, tag_id) {
        /// The `list_id` column of the `list_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `tag_id` column of the `list_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `created` column of the `list_tags` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `list_users` table.
    ///
    /// (Automatically generated by Diesel.)
    list_users (list_id, user_id) {
        /// The `list_id` column of the `list_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        list_id -> Int4,
        /// The `user_id` column of the `list_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `created` column of the `list_users` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `lists` table.
    ///
    /// (Automatically generated by Diesel.)
    lists (id) {
        /// The `id` column of the `lists` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `lists` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `user_id` column of the `lists` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `name` column of the `lists` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
    }
}

diesel::table! {
    /// Representation of the `post_comment_likes` table.
    ///
    /// (Automatically generated by Diesel.)
    post_comment_likes (user_id, post_comment_id) {
        /// The `user_id` column of the `post_comment_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `post_comment_id` column of the `post_comment_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_comment_id -> Int4,
        /// The `created` column of the `post_comment_likes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `post_comments` table.
    ///
    /// (Automatically generated by Diesel.)
    post_comments (id) {
        /// The `id` column of the `post_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `post_comments` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `post_id` column of the `post_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Int4,
        /// The `user_id` column of the `post_comments` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `text` column of the `post_comments` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        text -> Text,
    }
}

diesel::table! {
    /// Representation of the `post_likes` table.
    ///
    /// (Automatically generated by Diesel.)
    post_likes (user_id, post_id) {
        /// The `user_id` column of the `post_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `post_id` column of the `post_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Int4,
        /// The `created` column of the `post_likes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `post_tags` table.
    ///
    /// (Automatically generated by Diesel.)
    post_tags (post_id, tag_id) {
        /// The `post_id` column of the `post_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Int4,
        /// The `tag_id` column of the `post_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `created` column of the `post_tags` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `posts` table.
    ///
    /// (Automatically generated by Diesel.)
    posts (id) {
        /// The `id` column of the `posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `posts` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `user_id` column of the `posts` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `comic_id` column of the `posts` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        comic_id -> Nullable<Int4>,
        /// The `chapter_no` column of the `posts` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        chapter_no -> Nullable<Int4>,
        /// The `title` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Text,
        /// The `text` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        text -> Text,
        /// The `thumbnail_filename` column of the `posts` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        thumbnail_filename -> Text,
        /// The `nsfw` column of the `posts` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        nsfw -> Bool,
    }
}

diesel::table! {
    /// Representation of the `reports` table.
    ///
    /// (Automatically generated by Diesel.)
    reports (id) {
        /// The `id` column of the `reports` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `reports` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `reporter_id` column of the `reports` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        reporter_id -> Int4,
        /// The `post_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        post_id -> Nullable<Int4>,
        /// The `comic_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        comic_id -> Nullable<Int4>,
        /// The `blog_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        blog_id -> Nullable<Int4>,
        /// The `user_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Nullable<Int4>,
        /// The `post_comment_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        post_comment_id -> Nullable<Int4>,
        /// The `blog_comment_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        blog_comment_id -> Nullable<Int4>,
        /// The `tag_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Nullable<Int4>,
        /// The `resolved` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Timestamptz>`.
        ///
        /// (Automatically generated by Diesel.)
        resolved -> Nullable<Timestamptz>,
        /// The `resolved_by_id` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        resolved_by_id -> Nullable<Int4>,
        /// The `resolved_comment` column of the `reports` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        resolved_comment -> Nullable<Text>,
    }
}

diesel::table! {
    /// Representation of the `tag_aliases` table.
    ///
    /// (Automatically generated by Diesel.)
    tag_aliases (id) {
        /// The `id` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `tag_id` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `alias_name` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        alias_name -> Text,
        /// The `language` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        language -> Nullable<Text>,
        /// The `main` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        main -> Bool,
        /// The `canonical` column of the `tag_aliases` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        canonical -> Bool,
    }
}

diesel::table! {
    /// Representation of the `tag_auto_tags` table.
    ///
    /// (Automatically generated by Diesel.)
    tag_auto_tags (tag_id, auto_tag_id) {
        /// The `tag_id` column of the `tag_auto_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `auto_tag_id` column of the `tag_auto_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        auto_tag_id -> Int4,
        /// The `created` column of the `tag_auto_tags` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `tag_details` table.
    ///
    /// (Automatically generated by Diesel.)
    tag_details (id) {
        /// The `id` column of the `tag_details` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `tag_details` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `tag_id` column of the `tag_details` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `differentiator` column of the `tag_details` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        differentiator -> Nullable<Text>,
        /// The `description` column of the `tag_details` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Nullable<Text>,
        /// The `language` column of the `tag_details` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        language -> Nullable<Text>,
    }
}

diesel::table! {
    /// Representation of the `tag_follows` table.
    ///
    /// (Automatically generated by Diesel.)
    tag_follows (user_id, tag_id) {
        /// The `user_id` column of the `tag_follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `tag_id` column of the `tag_follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `created` column of the `tag_follows` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `tag_likes` table.
    ///
    /// (Automatically generated by Diesel.)
    tag_likes (user_id, tag_id) {
        /// The `user_id` column of the `tag_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `tag_id` column of the `tag_likes` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
        /// The `created` column of the `tag_likes` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `tags` table.
    ///
    /// (Automatically generated by Diesel.)
    tags (id) {
        /// The `id` column of the `tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `tags` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `url_slug` column of the `tags` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        url_slug -> Text,
        /// The `image_id` column of the `tags` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        image_id -> Nullable<Int4>,
        /// The `owner_id` column of the `tags` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        owner_id -> Nullable<Int4>,
        /// The `nsfw` column of the `tags` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        nsfw -> Bool,
    }
}

diesel::table! {
    /// Representation of the `user_blocked_users` table.
    ///
    /// (Automatically generated by Diesel.)
    user_blocked_users (blocker_id, blockee_id) {
        /// The `blocker_id` column of the `user_blocked_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blocker_id -> Int4,
        /// The `blockee_id` column of the `user_blocked_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        blockee_id -> Int4,
    }
}

diesel::table! {
    /// Representation of the `user_follows` table.
    ///
    /// (Automatically generated by Diesel.)
    user_follows (follower_user_id, followed_user_id) {
        /// The `follower_user_id` column of the `user_follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        follower_user_id -> Int4,
        /// The `followed_user_id` column of the `user_follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        followed_user_id -> Int4,
        /// The `created` column of the `user_follows` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
    }
}

diesel::table! {
    /// Representation of the `user_hidden_tags` table.
    ///
    /// (Automatically generated by Diesel.)
    user_hidden_tags (user_id, tag_id) {
        /// The `user_id` column of the `user_hidden_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `tag_id` column of the `user_hidden_tags` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        tag_id -> Int4,
    }
}

diesel::table! {
    /// Representation of the `user_muted_users` table.
    ///
    /// (Automatically generated by Diesel.)
    user_muted_users (muter_id, mutee_id) {
        /// The `muter_id` column of the `user_muted_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        muter_id -> Int4,
        /// The `mutee_id` column of the `user_muted_users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        mutee_id -> Int4,
    }
}

diesel::table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `created` column of the `users` table.
        ///
        /// Its SQL type is `Timestamptz`.
        ///
        /// (Automatically generated by Diesel.)
        created -> Timestamptz,
        /// The `display_name` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        display_name -> Text,
        /// The `email_address` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        email_address -> Text,
        /// The `profile_text` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        profile_text -> Text,
        /// The `avatar` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        avatar -> Text,
        /// The `role` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        role -> Text,
        /// The `password_hash` column of the `users` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        password_hash -> Text,
        /// The `nsfw` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        nsfw -> Bool,
        /// The `permission_login` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        permission_login -> Bool,
        /// The `permission_read` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        permission_read -> Bool,
        /// The `permission_comment` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        permission_comment -> Bool,
        /// The `permission_post` column of the `users` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        permission_post -> Bool,
    }
}

diesel::joinable!(blog_comment_likes -> blog_comments (blog_comment_id));
diesel::joinable!(blog_comment_likes -> users (user_id));
diesel::joinable!(blog_comments -> blogs (blog_id));
diesel::joinable!(blog_comments -> users (user_id));
diesel::joinable!(blog_likes -> blogs (blog_id));
diesel::joinable!(blog_likes -> users (user_id));
diesel::joinable!(blogs -> users (user_id));
diesel::joinable!(comic_likes -> comics (comic_id));
diesel::joinable!(comic_likes -> users (user_id));
diesel::joinable!(comic_tags -> comics (comic_id));
diesel::joinable!(comic_tags -> tags (tag_id));
diesel::joinable!(comics -> users (user_id));
diesel::joinable!(images -> posts (post_id));
diesel::joinable!(list_blog_comments -> blog_comments (blog_comment_id));
diesel::joinable!(list_blog_comments -> lists (list_id));
diesel::joinable!(list_blogs -> blogs (blog_id));
diesel::joinable!(list_blogs -> lists (list_id));
diesel::joinable!(list_comics -> comics (comic_id));
diesel::joinable!(list_comics -> lists (list_id));
diesel::joinable!(list_post_comments -> lists (list_id));
diesel::joinable!(list_post_comments -> post_comments (post_comment_id));
diesel::joinable!(list_posts -> lists (list_id));
diesel::joinable!(list_posts -> posts (post_id));
diesel::joinable!(list_tags -> lists (list_id));
diesel::joinable!(list_tags -> tags (tag_id));
diesel::joinable!(list_users -> lists (list_id));
diesel::joinable!(list_users -> users (user_id));
diesel::joinable!(lists -> users (user_id));
diesel::joinable!(post_comment_likes -> post_comments (post_comment_id));
diesel::joinable!(post_comment_likes -> users (user_id));
diesel::joinable!(post_comments -> posts (post_id));
diesel::joinable!(post_comments -> users (user_id));
diesel::joinable!(post_likes -> posts (post_id));
diesel::joinable!(post_likes -> users (user_id));
diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));
diesel::joinable!(posts -> comics (comic_id));
diesel::joinable!(posts -> users (user_id));
diesel::joinable!(reports -> blog_comments (blog_comment_id));
diesel::joinable!(reports -> blogs (blog_id));
diesel::joinable!(reports -> comics (comic_id));
diesel::joinable!(reports -> post_comments (post_comment_id));
diesel::joinable!(reports -> posts (post_id));
diesel::joinable!(reports -> tags (tag_id));
diesel::joinable!(tag_aliases -> tags (tag_id));
diesel::joinable!(tag_details -> tags (tag_id));
diesel::joinable!(tag_follows -> tags (tag_id));
diesel::joinable!(tag_follows -> users (user_id));
diesel::joinable!(tag_likes -> tags (tag_id));
diesel::joinable!(tag_likes -> users (user_id));
diesel::joinable!(tags -> images (image_id));
diesel::joinable!(tags -> users (owner_id));
diesel::joinable!(user_hidden_tags -> tags (tag_id));
diesel::joinable!(user_hidden_tags -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    bans,
    blog_comment_likes,
    blog_comments,
    blog_likes,
    blogs,
    comic_likes,
    comic_tags,
    comics,
    images,
    list_blog_comments,
    list_blogs,
    list_comics,
    list_post_comments,
    list_posts,
    list_tags,
    list_users,
    lists,
    post_comment_likes,
    post_comments,
    post_likes,
    post_tags,
    posts,
    reports,
    tag_aliases,
    tag_auto_tags,
    tag_details,
    tag_follows,
    tag_likes,
    tags,
    user_blocked_users,
    user_follows,
    user_hidden_tags,
    user_muted_users,
    users,
);
