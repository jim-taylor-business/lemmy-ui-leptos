use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
  let mut comments_clone = comments.get().clone();
  comments_clone.retain(|ct| ct.comment.path.chars().filter(|c| *c == '.').count() == 1);
  let com_sig = RwSignal::new(comments_clone);
  let child_show = RwSignal::new(true);

  view! {
    <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
      <CommentNode show=child_show comment_view=cv.into() comments=comments.get().into() level=1/>
    </For>
  }
}
