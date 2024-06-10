use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(comment_view: MaybeSignal<CommentView>) -> impl IntoView {
  view! {
    <div class="p-1">
      // {move || {
      //     format!("{} - {}", 
      <span class="font-bold"> { comment_view.get().creator.name }</span>
      " - "
      <span> { comment_view.get().comment.content }</span>
        
      //   )
      // }}

    </div>
  }
}
