use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(
  comment_view: MaybeSignal<CommentView>,
  comments: Vec<CommentView>,
) -> impl IntoView {
  view! {
    <div class="p-1">
      <span> { comment_view.get().comment.content }</span>
      " - "
      <span class="font-bold"> { comment_view.get().creator.name }</span>
      // {
      //   let id = comment_view.get().comment.id;
      //   comments.iter().filter(|c| c.comment.parent_id == )
      // }
    </div>
  }
}
