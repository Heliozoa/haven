use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Type {
    String,
    Int32,
    Bool,
}

#[derive(Serialize)]
struct Field {
    name: &'static str,
    #[serde(rename = "type")]
    ty: Type,
}

#[derive(Serialize)]
struct Schema {
    name: &'static str,
    fields: &'static [Field],
}

fn main() {
    let tags = Schema {
        name: "tags",
        fields: &[
            Field {
                name: "id",
                ty: Type::Int32,
            },
            Field {
                name: "url_slug",
                ty: Type::String,
            },
            Field {
                name: "image_id",
                ty: Type::String,
            },
            Field {
                name: "owner_id",
                ty: Type::String,
            },
            Field {
                name: "nsfw",
                ty: Type::Bool,
            },
        ],
    };
}
