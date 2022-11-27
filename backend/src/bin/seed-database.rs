use diesel::{Connection, PgConnection};
use haven::domain::*;
use std::{
    env,
    process::{Command, Output},
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    // reset db
    assert_success(
        &Command::new("diesel")
            .args(["database", "reset"])
            .output()
            .unwrap(),
    );

    // reset static files
    std::fs::remove_dir_all("./static/avatar").unwrap();
    std::fs::create_dir("./static/avatar").unwrap();
    std::fs::remove_dir_all("./static/image").unwrap();
    std::fs::create_dir("./static/image").unwrap();
    std::fs::remove_dir_all("./static/thumbnail").unwrap();
    std::fs::create_dir("./static/thumbnail").unwrap();
    std::fs::write(
        concat!(env!("CARGO_MANIFEST_DIR"), "/static/avatar/default.png"),
        include_bytes!("default.png"),
    )
    .unwrap();

    let db = env::var("DATABASE_URL").unwrap();
    let mut conn = PgConnection::establish(&db).unwrap();

    let a = create_user(&mut conn, "a");
    let b = create_user(&mut conn, "b");
    let c = create_user(&mut conn, "c");
    let d = create_user(&mut conn, "d");

    println!("done");
}

fn create_user(conn: &mut PgConnection, thing: &str) -> i32 {
    let id = users::register(conn, thing, thing, thing).unwrap();
    id
}

fn create_post(conn: &mut PgConnection, thing: &str, user: i32) -> i32 {
    todo!()
}

fn generate_image(
    filename: &str,
    dir: &str,
    height: i64,
    width: i64,
) -> Result<(), std::io::Error> {
    let out = Command::new("convert")
        .current_dir(dir)
        .args([
            "-size",
            "8x8",
            "xc:",
            "+noise",
            "Random",
            "-scale",
            &format!("{}x{}!", height, width),
            &format!("PNG:{}", filename),
        ])
        .output()?;
    assert!(out.status.success());
    Ok(())
}

fn assert_success(output: &Output) {
    if !output.status.success() {
        println!("{}", String::from_utf8(output.stdout.clone()).unwrap());
        println!("{}", String::from_utf8(output.stderr.clone()).unwrap());
        panic!()
    }
}
