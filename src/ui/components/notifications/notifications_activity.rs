use std::collections::BTreeMap;

use lemmy_api_common::{
  lemmy_db_views::structs::PaginationCursor,
  post::{GetPosts, GetPostsResponse},
};
use leptos::*;
use leptos_router::{use_location, use_query_map, A};

use crate::{
  errors::{message_from_error, LemmyAppError},
  ui::components::common::about::About,
  LemmyClient, PublicFetch,
};

#[component]
pub fn NotificationsActivity() -> impl IntoView {
  let errors = expect_context::<RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>>();

  view! {

    <main class="mx-auto">

    {move || {
        // for error in error.get().iter() {
          errors.get().into_iter().enumerate()
                      .map(|(i, error)| {

                        error.map(|err| {
                          view! {
                          <div class="px-8 py-4">
                            <div class="alert alert-error flex justify-between">
                              <span>{message_from_error(&err.0)} " - " {err.0.content}</span>
                              <div>
                                <Show when=move || { if let Some(r) = err.1 { true } else { false } } /* let:r */ fallback=|| {}>
                                  <button on:click=move |_| { if let Some(r) = err.1 { r.set(!r.get()); } else { } } class="btn btn-sm"> "Retry" </button>
                                </Show>
                                <button class="btn btn-sm" on:click=move |_| { errors.update(|es| { es.remove(i); }); }> "Clear" </button>
                              </div>
                            </div>
                          </div>
                          }
                        })

          // if let Some(err) = error {
          // } else {
          //   view! {}
          // }
                      }).collect::<Vec<_>>()
        // }
          // .map(|es| {

          //     // es.iter().
          // })
    }}
      <button class=move || format!("btn{}", if errors.get().len() > 0 { "" } else { " btn-disabled" }) on:click=move |_| { errors.set(vec![]); }> "Clear All" </button>
      <About />
    </main>
  }
}
