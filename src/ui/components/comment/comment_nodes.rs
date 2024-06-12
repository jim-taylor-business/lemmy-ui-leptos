use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
  let comments_clone = comments.get().clone();

  view! {
    <For each=move || comments.get() key=|cv| cv.comment.id let:cv>
      {
        let comments_copy = comments_clone.clone();

        view! {
          <CommentNode comment_view=cv.into() comments=comments_copy/>

        }

      }
    </For>
  }
}
