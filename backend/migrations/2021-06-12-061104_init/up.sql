-- users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    display_name TEXT UNIQUE NOT NULL,
    email_address TEXT UNIQUE NOT NULL,
    profile_text TEXT NOT NULL DEFAULT '',
    avatar TEXT NOT NULL DEFAULT 'default.png',
    role TEXT NOT NULL DEFAULT 'user',
    password_hash TEXT NOT NULL,
    nsfw BOOLEAN NOT NULL DEFAULT false,
    permission_login BOOLEAN NOT NULL DEFAULT true,
    permission_read BOOLEAN NOT NULL DEFAULT true,
    permission_comment BOOLEAN NOT NULL DEFAULT true,
    permission_post BOOLEAN NOT NULL DEFAULT true
);
-- comics & posts & blogs
CREATE TABLE comics (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    thumbnail_filename TEXT NOT NULL,
    nsfw BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users,
    comic_id INTEGER REFERENCES comics,
    chapter_no INTEGER,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    thumbnail_filename TEXT NOT NULL,
    nsfw BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE images (
    id SERIAL PRIMARY KEY,
    post_id INTEGER NOT NULL REFERENCES posts,
    filename TEXT NOT NULL,
    order_no INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL
);
CREATE TABLE blogs (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    nsfw BOOLEAN NOT NULL DEFAULT false
);
-- tags
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    url_slug TEXT UNIQUE NOT NULL,
    image_id INTEGER REFERENCES images,
    owner_id INTEGER REFERENCES users,
    nsfw BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE tag_aliases (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tag_id INTEGER REFERENCES tags NOT NULL,
    alias_name TEXT NOT NULL,
    language TEXT,
    main BOOLEAN NOT NULL DEFAULT false,
    canonical BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE tag_details (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tag_id INTEGER NOT NULL REFERENCES tags,
    differentiator TEXT,
    description TEXT,
    language TEXT
);
CREATE TABLE post_tags (
    post_id INTEGER NOT NULL REFERENCES posts,
    tag_id INTEGER NOT NULL REFERENCES tags,
    PRIMARY KEY (post_id, tag_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE comic_tags (
    comic_id INTEGER NOT NULL REFERENCES comics,
    tag_id INTEGER NOT NULL REFERENCES tags,
    PRIMARY KEY (comic_id, tag_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE tag_auto_tags (
    tag_id INTEGER NOT NULL REFERENCES tags,
    auto_tag_id INTEGER NOT NULL REFERENCES tags,
    PRIMARY KEY (tag_id, auto_tag_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- follows
CREATE TABLE user_follows (
    follower_user_id INTEGER NOT NULL REFERENCES users,
    followed_user_id INTEGER NOT NULL REFERENCES users,
    PRIMARY KEY (follower_user_id, followed_user_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE tag_follows (
    user_id INTEGER NOT NULL REFERENCES users,
    tag_id INTEGER NOT NULL REFERENCES tags,
    PRIMARY KEY (user_id, tag_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- comments
CREATE TABLE post_comments (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    post_id INTEGER NOT NULL REFERENCES posts,
    user_id INTEGER NOT NULL REFERENCES users,
    text TEXT NOT NULL
);
CREATE TABLE blog_comments (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    blog_id INTEGER NOT NULL REFERENCES blogs,
    user_id INTEGER NOT NULL REFERENCES users,
    text TEXT NOT NULL
);
-- likes
CREATE TABLE post_likes (
    user_id INTEGER NOT NULL REFERENCES users,
    post_id INTEGER NOT NULL REFERENCES posts,
    PRIMARY KEY (user_id, post_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE comic_likes (
    user_id INTEGER NOT NULL REFERENCES users,
    comic_id INTEGER NOT NULL REFERENCES comics,
    PRIMARY KEY (user_id, comic_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE blog_likes (
    user_id INTEGER NOT NULL REFERENCES users,
    blog_id INTEGER NOT NULL REFERENCES blogs,
    PRIMARY KEY (user_id, blog_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE tag_likes (
    user_id INTEGER NOT NULL REFERENCES users,
    tag_id INTEGER NOT NULL REFERENCES tags,
    PRIMARY KEY (user_id, tag_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE post_comment_likes (
    user_id INTEGER NOT NULL REFERENCES users,
    post_comment_id INTEGER NOT NULL REFERENCES post_comments,
    PRIMARY KEY (user_id, post_comment_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE blog_comment_likes (
    user_id INTEGER NOT NULL REFERENCES users,
    blog_comment_id INTEGER NOT NULL REFERENCES blog_comments,
    PRIMARY KEY (user_id, blog_comment_id),
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
-- lists
CREATE TABLE lists (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL REFERENCES users,
    name TEXT NOT NULL
);
CREATE TABLE list_posts (
    list_id INTEGER NOT NULL REFERENCES lists,
    post_id INTEGER NOT NULL REFERENCES posts,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, post_id)
);
CREATE TABLE list_comics (
    list_id INTEGER NOT NULL REFERENCES lists,
    comic_id INTEGER NOT NULL REFERENCES comics,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, comic_id)
);
CREATE TABLE list_blogs (
    list_id INTEGER NOT NULL REFERENCES lists,
    blog_id INTEGER NOT NULL REFERENCES blogs,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, blog_id)
);
CREATE TABLE list_users (
    list_id INTEGER NOT NULL REFERENCES lists,
    user_id INTEGER NOT NULL REFERENCES users,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, user_id)
);
CREATE TABLE list_tags (
    list_id INTEGER NOT NULL REFERENCES lists,
    tag_id INTEGER NOT NULL REFERENCES tags,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, tag_id)
);
CREATE TABLE list_post_comments (
    list_id INTEGER NOT NULL REFERENCES lists,
    post_comment_id INTEGER NOT NULL REFERENCES post_comments,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, post_comment_id)
);
CREATE TABLE list_blog_comments (
    list_id INTEGER NOT NULL REFERENCES lists,
    blog_comment_id INTEGER NOT NULL REFERENCES blog_comments,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (list_id, blog_comment_id)
);
-- user blocks/mutes
CREATE TABLE user_blocked_users (
    blocker_id INTEGER NOT NULL REFERENCES users,
    blockee_id INTEGER NOT NULL REFERENCES users,
    PRIMARY KEY (blocker_id, blockee_id)
);
CREATE TABLE user_muted_users (
    muter_id INTEGER NOT NULL REFERENCES users,
    mutee_id INTEGER NOT NULL REFERENCES users,
    PRIMARY KEY (muter_id, mutee_id)
);
CREATE TABLE user_hidden_tags (
    user_id INTEGER NOT NULL REFERENCES users,
    tag_id INTEGER NOT NULL REFERENCES tags,
    PRIMARY KEY (user_id, tag_id)
);
-- reports
CREATE TABLE reports (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    reporter_id INTEGER NOT NULL REFERENCES users,
    post_id INTEGER REFERENCES posts,
    comic_id INTEGER REFERENCES comics,
    blog_id INTEGER REFERENCES blogs,
    user_id INTEGER REFERENCES users,
    post_comment_id INTEGER REFERENCES post_comments,
    blog_comment_id INTEGER REFERENCES blog_comments,
    tag_id INTEGER REFERENCES tags,
    resolved TIMESTAMPTZ,
    resolved_by_id INTEGER REFERENCES users,
    resolved_comment TEXT
);
-- user bans
CREATE TABLE bans (
    id SERIAL PRIMARY KEY,
    created TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    banned_id INTEGER NOT NULL REFERENCES users,
    banned_by_id INTEGER NOT NULL REFERENCES users,
    reason TEXT NOT NULL,
    expires TIMESTAMPTZ
);