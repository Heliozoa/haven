use crate::{
    api::{Login, Registration, User},
    authorization::{self, Authentication},
    domain,
    error::{HavenError, HavenResult},
    query, CookiesWrapper, Pool,
};
use axum::{extract::Extension, Json};
use cookie::{
    time::{Duration as TDuration, OffsetDateTime},
    SameSite,
};
use diesel::prelude::*;
use tower_cookies::{Cookie, Key};

pub async fn logged(pool: Pool, auth: Option<Authentication>) -> HavenResult<Json<Option<User>>> {
    use crate::schema::users as u;
    let mut conn = pool.get()?;

    if let Some(authentication) = auth {
        let user_name = tokio::task::spawn_blocking(move || {
            u::table
                .find(authentication.user_id)
                .select(u::display_name)
                .get_result(&mut conn)
        })
        .await??;
        Ok(Json(Some(User {
            id: authentication.user_id,
            name: user_name,
        })))
    } else {
        Ok(Json(None))
    }
}

pub async fn login(
    pool: Pool,
    CookiesWrapper(cookies): CookiesWrapper,
    key: Extension<Key>,
    login: Json<Login>,
) -> HavenResult<Json<User>> {
    use crate::schema::users as u;
    let mut conn = pool.get()?;

    query! {
        struct LoginUser {
            id: i32 = users::id,
            name: String = users::display_name,
            password_hash: String = users::password_hash,
        }
    }
    let (user, login) = tokio::task::spawn_blocking(move || {
        let user = u::table
            .filter(u::email_address.eq(&login.email))
            .select(LoginUser::as_select())
            .get_result(&mut conn)?;
        QueryResult::Ok((user, login.0))
    })
    .await??;

    if authorization::verify_password(login.password.as_bytes(), &user.password_hash) {
        let auth = Authentication { user_id: user.id };
        let private = cookies.private(&key);
        let mut cookie = Cookie::new(
            Authentication::COOKIE_NAME,
            serde_json::to_string(&auth).unwrap(),
        );
        cookie.set_secure(true);
        cookie.set_path("/");
        cookie.set_same_site(Some(SameSite::None));
        cookie.set_expires(OffsetDateTime::now_utc() + TDuration::days(1));
        private.add(cookie);

        let user = User {
            id: user.id,
            name: user.name,
        };
        Ok(Json(user))
    } else {
        Err(HavenError::Auth)
    }
}

pub async fn logout(CookiesWrapper(cookies): CookiesWrapper) {
    cookies.remove(Cookie::named(Authentication::COOKIE_NAME));
}

pub async fn register(pool: Pool, registration: Json<Registration>) -> HavenResult<()> {
    use crate::schema::users as u;
    let mut conn = pool.get().unwrap();

    let (user_id, registration) = tokio::task::spawn_blocking(move || {
        let user_id = u::table
            .filter(u::email_address.eq(&registration.email))
            .select(u::id)
            .get_result::<i32>(&mut conn)
            .optional()?;
        QueryResult::Ok((user_id, registration.0))
    })
    .await??;
    if user_id.is_some() {
        return Err(HavenError::Auth);
    }

    let mut conn = pool.get().unwrap();
    tokio::task::spawn_blocking(move || {
        domain::users::register(
            &mut conn,
            &registration.name,
            &registration.email,
            &registration.password,
        )
    })
    .await??;
    Ok(())
}
