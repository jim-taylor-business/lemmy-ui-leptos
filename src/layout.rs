use crate::{
  cookie::get_cookie,
  errors::LemmyAppError,
  ui::components::common::nav::{BottomNav, TopNav}, TitleSetter,
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;
use leptos_meta::*;
use leptos_router::Outlet;

#[component]
pub fn Layout(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();
//   let title = move || match site_signal.get() {
//     Some(Ok(o)) => {
//       logging::log!("oofts");
//       if let Some(s) = o.site_view.site.description {
//         format!("Tech Demo UI for {} - {}", o.site_view.site.name, s)
//       } else {
//         format!("Tech Demo UI for {}", o.site_view.site.name)
//       }
//     }
//     _ => {
//       logging::log!("2");
//       "Lemmy".to_string()
//     },
// };

  let title = move || match site_signal.get() {
    Some(Ok(site)) => {
      // logging::log!("1");
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
    },
    _ => {
      // logging::log!("2");
      "Lemmy".to_string()
    },
  };

  // let title = create_resource(
  //   move || (site_signal.get().is_some(), ui_title.get().is_some()),
  //   move |(s, u)| async move {
  //     let tit = match site_signal.get() {
  //       Some(Ok(site)) => {
  //         logging::log!("oofts");
  //         let q = if let Some(TitleSetter(t)) = ui_title.get() { t } else { "".to_string() };
  //         if let Some(d) = site.site_view.site.description {
  //           format!("{} - Tech Demo UI for {} - {}", q, site.site_view.site.name, d)
  //         } else {
  //           format!("{} - Tech Demo UI for {}", q, site.site_view.site.name)
  //         }
  //       }
  //       _ => {
  //         logging::log!("3");
  //         "Lemmy".to_string()
  //       },
  //     };

  //     logging::log!("nitty {}", tit);
  //     tit

  //     // ui_title.set(Some(TitleSetter(tit)));
  //   },
  // );

  let ui_theme = expect_context::<RwSignal<Option<String>>>();
  let theme = create_resource(
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
    <Link rel="shortcut icon" type_="image/ico" href="/favicon.svg"/>
    // <Meta name="description" content=title/>
    // debug console when there is no dev tools (mobile/desktop)
    // <Script src="//cdn.jsdelivr.net/npm/eruda"/>
    // <Script>eruda.init();</Script>
    // <Transition fallback=|| {}>
    //   {move || {
    //       title
    //           .get()
    //           .map(|m| {
    //               logging::log!("render {}", m);
    //               // ui_theme.set(Some(m));
    //               view! {
    //                 <Title text=m/>
    //               }
    //           })
    //   }}
    // </Transition>
    // <Title text=move || { if let Some(TitleSetter(t)) = ui_title.get() { logging::log!("yes"); t } else { logging::log!("no"); "".to_string() } }/>
    // <Title text=move || { if let Some(TitleSetter(t)) = ui_title.get() { logging::log!("yes"); format!("{} - {}", t, title()) } else { logging::log!("no"); title() } }/>
    <Title text=move || title() />
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
    // <span> { move || { if let Some(TitleSetter(t)) = ui_title.get() { logging::log!("yes"); format!("{} - {}", t, test()) } else { logging::log!("no"); test() } } } </span>
                        <div class="w-full flex flex-col flex-grow p-6">
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
