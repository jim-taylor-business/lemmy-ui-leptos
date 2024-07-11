use std::collections::BTreeMap;

use lemmy_api_common::{lemmy_db_views::structs::PaginationCursor, post::{GetPosts, GetPostsResponse}};
use leptos::*;
use leptos_router::{use_query_map, use_location, A};

use crate::{ui::components::common::about::About, LemmyClient, PublicFetch};

#[component]
pub fn CommunitiesActivity() -> impl IntoView {

  view! {
    <main class="mx-auto">
      <About />
    </main>
  }
}
