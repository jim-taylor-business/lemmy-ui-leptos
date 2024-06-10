use crate::{
  //   cookie::{remove_cookie, set_cookie},
  errors::{message_from_error, LemmyAppError},
  //   i18n::*,
  //   lemmy_client::*,
  //   ui::components::common::icon::{
  //     Icon,
  //     IconType::{Donate, Notifications, Search},
  //   },
};
use lemmy_api_common::site::GetSiteResponse;
use leptos::*;

#[component]
pub fn About(
  // site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  //   let i18n = use_i18n();
  //   const FE_VERSION: &str = env!("CARGO_PKG_VERSION");

  view! {
      <div class="container mx-auto alert">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-info shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
        <div>
          <h3 class="font-bold text-3xl">"About this site"</h3>
          <p>"This is a technical demo and proof of concept of the technical objectives specified by myself on my "<a class="link" href="//github.com/jim-taylor-business/lemmy-ui-leptos#objectives">"Lemmy UI Leptos homepage"</a>"."</p>
          <p>"It is also intended to be near feature complete with the homepage functionality of "<a class="link" href="//lemmy.world">"Lemmy world"</a>", and near issue free."</p>
          <p>"This site is not affiliated with Lemmy World in any way."</p>
        </div>
      </div>
    }
}
