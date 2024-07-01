// useful in development to only have errors in compiler output
#![allow(warnings)]

use cfg_if::cfg_if;
use lemmy_ui_leptos::*;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_files::Files;
        use actix_web::*;

        use leptos::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        use awc::Client;

        #[actix_web::get("favicon.svg")]
        async fn lemmy(
            leptos_options: web::Data<leptos::LeptosOptions>,
        ) -> actix_web::Result<actix_files::NamedFile> {
            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.svg"))?)
        }

        #[actix_web::get("favicon.ico")]
        async fn favicon(
            leptos_options: web::Data<leptos::LeptosOptions>,
        ) -> actix_web::Result<actix_files::NamedFile> {
            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
        }

        #[actix_web::get("icons.svg")]
        async fn icons(
            leptos_options: web::Data<leptos::LeptosOptions>
        ) -> actix_web::Result<actix_files::NamedFile> {
            let leptos_options = leptos_options.into_inner();
            let site_root = &leptos_options.site_root;
            Ok(actix_files::NamedFile::open(format!("{site_root}/icons.svg"))?)
        }

        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            let conf = get_configuration(None).await.unwrap();
            let addr = conf.leptos_options.site_addr;
            let routes = generate_route_list(App);

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;
                let routes = &routes;

                let client = web::Data::new(Client::new());

                App::new()
                    .route("/serverfn/{tail:.*}", leptos_actix::handle_server_fns())
                    .service(Files::new("/pkg", format!("{site_root}/pkg")))
                    .service(Files::new("/assets", site_root))
                    .service(favicon)
                    .service(icons)
                    .service(lemmy)
                    .leptos_routes(
                        leptos_options.to_owned(),
                        routes.to_owned(),
                        App
                    )
                    .app_data(web::Data::new(leptos_options.to_owned()))
                    .app_data(client)
            })
            .bind(&addr)?
            .run()
            .await
        }
    }
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
  // for pure client-side testing
  // see lib.rs for hydration function
  // a client-side main function is required for using `trunk serve`
  // to run: `trunk serve --open --features hydrate`
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
  // a client-side main function is required for using `trunk serve`
  // to run: `trunk serve --open --features csr`
  use wasm_bindgen::prelude::wasm_bindgen;
  // required for better debug messages
  console_error_panic_hook::set_once();
  leptos::mount_to_body(App);
}
