use crate::{
  cookie::get_cookie,
  errors::LemmyAppError,
  ui::components::common::nav::{BottomNav, TopNav},
  TitleSetter,
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

#[component]
pub fn Layout(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
  ssr_site: Resource<Option<bool>, Result<GetSiteResponse, LemmyAppError>>,
) -> impl IntoView {
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

  // let title = move || match site_signal.get() {
  //   Some(Ok(site)) => {
  //     if let Some(TitleSetter(t)) = ui_title.get() {
  //       if let Some(d) = site.site_view.site.description {
  //         format!("{} - Tech Demo UI for {} - {}", t, site.site_view.site.name, d)
  //       } else {
  //         format!("{} - Tech Demo UI for {}", t, site.site_view.site.name)
  //       }
  //     } else {
  //       if let Some(d) = site.site_view.site.description {
  //         format!("Tech Demo UI for {} - {}", site.site_view.site.name, d)
  //       } else {
  //         format!("Tech Demo UI for {}", site.site_view.site.name)
  //       }
  //     }
  //   }
  //   _ => "Lemmy".to_string(),
  // };

  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let theme = Resource::new(
    move || (),
    move |()| async move {
      let r = get_cookie("theme").await;
      match r {
        Ok(Some(o)) => o,
        _ => "".to_string(),
      }
    },
  );

  view! {
    <Stylesheet id="leptos" href="/pkg/lemmy-ui-leptos.css"/>
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
    <Title text=move || title() />
    <Meta name="description" content=move || title() />
    <Transition fallback=|| {}>
      {move || {
          theme
              .get()
              .map(|m| {
                  ui_theme.set(Some(m));
                  view! {
                    <div class="flex flex-col min-h-screen" data-theme=move || ui_theme.get()>
                      <TopNav site_signal/>
                      <div class="w-full flex flex-col flex-grow">
                        <div class="sm:container sm:mx-auto">
                          <div class="w-full flex flex-col flex-grow px-0 lg:px-6">
                            <Outlet/>
                          </div>
                        </div>
                      </div>
                      <BottomNav site_signal/>
                    </div>
                  }
              })
      }}
    </Transition>
  }
}
