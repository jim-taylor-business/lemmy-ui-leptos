use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
  view! {
    <For each=move || comments.get() key=|cv| cv.comment.id let:cv>
      <CommentNode comment_view=cv.into()/>
    </For>
  }
}
