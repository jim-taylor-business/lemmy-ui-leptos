// useful in development to only have errors in compiler output
// #![allow(warnings)]

mod config;
mod cookie;
mod errors;
mod host;
mod indexed_db;
mod layout;
mod lemmy_client;
mod lemmy_error;
mod ui;

use std::collections::BTreeMap;

use crate::{
  errors::LemmyAppError,
  i18n::*,
  layout::Layout,
  lemmy_client::*,
  ui::components::{
    communities::communities_activity::CommunitiesActivity, home::home_activity::HomeActivity, login::login_activity::LoginActivity,
    post::post_activity::PostActivity,
  },
};

use codee::string::FromToStringCodec;
use lemmy_api_common::{lemmy_db_schema::SortType, lemmy_db_views::structs::PaginationCursor, post::GetPostsResponse, site::GetSiteResponse};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};
use ui::components::notifications::notifications_activity::NotificationsActivity;

leptos_i18n::load_locales!();

#[derive(Clone)]
pub struct TitleSetter(String);
#[derive(Clone)]
pub struct UriSetter(String);
#[derive(Clone)]
pub struct OnlineSetter(bool);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum ResourceStatus {
  Loading,
  Ok,
  Err,
}
#[derive(Clone, Debug, PartialEq)]
pub struct NotificationsRefresh(bool);

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  let error: RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>> = RwSignal::new(Vec::new());
  provide_context(error);
  let authenticated: RwSignal<Option<bool>> = RwSignal::new(None);
  provide_context(authenticated);
  let title: RwSignal<Option<TitleSetter>> = RwSignal::new(None);
  provide_context(title);
  let online = RwSignal::new(OnlineSetter(true));
  provide_context(online);
  let notifications_refresh = RwSignal::new(NotificationsRefresh(true));
  provide_context(notifications_refresh);
  let uri: RwSignal<UriSetter> = RwSignal::new(UriSetter("".into()));
  provide_context(uri);

  let on_online = move |b: bool| {
    move |_| {
      online.set(OnlineSetter(b));
    }
  };

  let _offline_handle = window_event_listener_untyped("offline", on_online(false));
  let _online_handle = window_event_listener_untyped("online", on_online(true));

  let csr_resources: RwSignal<BTreeMap<(usize, ResourceStatus), (Option<PaginationCursor>, Option<GetPostsResponse>)>> =
    RwSignal::new(BTreeMap::new());
  provide_context(csr_resources);
  let csr_sort: RwSignal<SortType> = RwSignal::new(SortType::Active);
  provide_context(csr_sort);
  let csr_next_page_cursor: RwSignal<(usize, Option<PaginationCursor>)> = RwSignal::new((0, None));
  provide_context(csr_next_page_cursor);

  let site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>> = RwSignal::new(None);

  let ssr_site = Resource::new(
    move || (authenticated.get()),
    move |user| async move {
      let result = if user == Some(false) {
        if let Some(Ok(mut s)) = site_signal.get() {
          s.my_user = None;
          Ok(s)
        } else {
          LemmyClient.get_site().await
        }
      } else {
        LemmyClient.get_site().await
      };

      match result {
        Ok(o) => Ok(o),
        Err(e) => {
          error.update(|es| es.push(Some((e.clone(), None))));
          Err(e)
        }
      }
    },
  );

  let (get_theme_cookie, set_theme_cookie) = use_cookie_with_options::<String, FromToStringCodec>(
    "theme",
    UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
  );
  if let Some(t) = get_theme_cookie.get() {
    set_theme_cookie.set(Some(t));
  }

  view! {
    <Transition fallback={|| {}}>
      {move || {
        ssr_site
          .get()
          .map(|m| {
            site_signal.set(Some(m));
          });
      }}
    </Transition>
    <I18nContextProvider cookie_options={UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax)}>
      <Router>
        <Routes>
          <Route path="/" view={move || view! { <Layout ssr_site /> }} ssr={SsrMode::Async}>
            <Route path="/*any" view={NotFound} />

            <Route path="" view={move || view! { <HomeActivity ssr_site /> }} />

            <Route path="create_post" view={CommunitiesActivity} />
            <Route path="post/:id" view={move || view! { <PostActivity ssr_site /> }} />

            <Route path="search" view={CommunitiesActivity} />
            <Route path="communities" view={CommunitiesActivity} />
            <Route path="create_community" view={CommunitiesActivity} />
            <Route path="c/:name" view={move || view! { <HomeActivity ssr_site /> }} />

            <Route path="login" methods={&[Method::Get, Method::Post]} view={LoginActivity} />
            <Route path="logout" view={CommunitiesActivity} />
            <Route path="signup" view={CommunitiesActivity} />

            <Route path="inbox" view={CommunitiesActivity} />
            <Route path="settings" view={CommunitiesActivity} />
            <Route path="notifications" view={NotificationsActivity} />
            <Route path="u/:id" view={CommunitiesActivity} />

            <Route path="modlog" view={CommunitiesActivity} />
            <Route path="instances" view={CommunitiesActivity} />
          </Route>
        </Routes>
      </Router>
    </I18nContextProvider>
  }
}

#[component]
fn NotFound() -> impl IntoView {
  #[cfg(feature = "ssr")]
  {
    let resp = expect_context::<leptos_actix::ResponseOptions>();
    resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
  }
  view! { <h1>"Not Found"</h1> }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  mount_to_body(App);
}
