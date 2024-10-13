use crate::{
  errors::LemmyAppError,
  ui::components::common::nav::{BottomNav, TopNav},
  TitleSetter,
};
use codee::string::FromToStringCodec;
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;
use leptos_use::{use_cookie_with_options, SameSite, UseCookieOptions};

#[component]
pub fn Layout(ssr_site: Resource<Option<bool>, Result<GetSiteResponse, LemmyAppError>>) -> impl IntoView {
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();
  let title = move || match ssr_site.get() {
    Some(Ok(site)) => {
      if let Some(TitleSetter(t)) = ui_title.get() {
        if let Some(d) = site.site_view.site.description {
          format!("{} - Tech Demo UI for {} - {}", t, site.site_view.site.name, d)
        } else {
          format!("{} - Tech Demo UI for {}", t, site.site_view.site.name)
        }
      } else {
        if let Some(d) = site.site_view.site.description {
          format!("Tech Demo UI for {} - {}", site.site_view.site.name, d)
        } else {
          format!("Tech Demo UI for {}", site.site_view.site.name)
        }
      }
    }
    _ => "Lemmy".to_string(),
  };

  let (theme_cookie, _) = use_cookie_with_options::<String, FromToStringCodec>(
    "theme",
    UseCookieOptions::default().max_age(2147483647).path("/").same_site(SameSite::Lax),
  );

  view! {
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css" />
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico" />
    <Title text={move || title()} />
    <Meta name="description" content={move || title()} />
    <Transition fallback={|| {}}>
      {move || {
        ssr_site
          .get()
          .map(|_| {
            view! {
              <div class="flex flex-col min-h-screen" data-theme={move || theme_cookie.get()}>
                <TopNav ssr_site />
                <div class="flex flex-col flex-grow w-full">
                  <div class="sm:container sm:mx-auto">
                    <div class="flex flex-col flex-grow px-0 w-full lg:px-6">
                      <Outlet />
                    </div>
                  </div>
                </div>
                <BottomNav ssr_site />
              </div>
            }
          })
      }}
    </Transition>
  }
}
