use crate::{errors::LemmyAppError, ui::components::post::post_listing::PostListing};
use lemmy_api_common::{lemmy_db_views::structs::PostView, site::GetSiteResponse};
use leptos::*;

#[component]
pub fn PostListings(
  posts: MaybeSignal<Vec<PostView>>,
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  view! {
    <table class="table">
      <For each=move || posts.get() key=|pv| pv.post.id let:pv>
        <PostListing post_view=pv.into() site_signal/>
      </For>
    </table>
  }
}
