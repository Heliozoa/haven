use haven::api::*;

fn main() {
    let mut file = std::fs::File::create("../frontend/src/Bindings.elm").unwrap();
    elm_rs::export!(
        "Bindings",
        &mut file, {
            encoders: [
                Login,
                Registration,
                NewComment,
                UserUpdate,
                PostMetadata,
                Thumbnail,
                User,
            ],
            decoders: [
                PostSmall,
                PostFull,
                Image,
                Comment,
                User,
                Tag,
                TagEmbed,
                TagView,
                UserView,
                Error,
            ],
            queries: [
            ],
        }

    )
    .unwrap();
}
