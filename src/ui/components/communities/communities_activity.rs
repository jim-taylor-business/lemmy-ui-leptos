use std::collections::BTreeMap;

use lemmy_api_common::{
  lemmy_db_views::structs::PaginationCursor,
  post::{GetPosts, GetPostsResponse},
};
use leptos::*;
use leptos_router::{use_location, use_query_map, A};

use crate::{ui::components::common::about::About, LemmyApi, LemmyClient};

#[component]
pub fn CommunitiesActivity() -> impl IntoView {
  view! {
    <main class="mx-auto">
      <About />
    </main>
  }
}
