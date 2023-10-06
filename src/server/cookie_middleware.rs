use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_session::{SessionMiddleware, config::{CookieContentSecurity, PersistentSession}, storage::CookieSessionStore};
        use actix_web::cookie::{Key, SameSite};

        pub fn cookie_middleware() -> SessionMiddleware<CookieSessionStore> {
            SessionMiddleware::builder(
                CookieSessionStore::default(), Key::generate() // TODO: Should probably get the key for the cookie from config
            )
            .cookie_name(String::from("jwt"))
            .cookie_secure(false) // TODO: Make cookie secure option depend on whether in dev or prod
            .session_lifecycle(PersistentSession::default())
            .cookie_same_site(SameSite::Strict)
            .cookie_content_security(CookieContentSecurity::Private)
            .cookie_http_only(true)
            .build()
        }
    }
}
