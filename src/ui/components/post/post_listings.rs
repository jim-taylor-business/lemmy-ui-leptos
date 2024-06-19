use crate::{errors::LemmyAppError, ui::components::post::post_listing::PostListing, PageNumberSetter};
use lemmy_api_common::{lemmy_db_views::structs::PostView, site::GetSiteResponse};
use leptos::*;

#[component]
pub fn PostListings(
  posts: MaybeSignal<Vec<PostView>>,
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
  page_number: RwSignal<PageNumberSetter>,
) -> impl IntoView {
  let post_number = RwSignal::new(page_number.get());
  view! {
    <div>
      <For each=move || posts.get() key=|pv| pv.post.id let:pv>
        {
          post_number.set(PageNumberSetter(post_number.get().0 + 1)); 

          view! {
            <PostListing post_view=pv.into() site_signal post_number=post_number.get().0/>
          }
        }
      </For>
    </div>
  }
}
