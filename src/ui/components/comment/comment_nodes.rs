use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(comments: MaybeSignal<Vec<CommentView>>) -> impl IntoView {
  let mut comments_clone = comments.get().clone();
  comments_clone.retain(|ct| ct.comment.path.chars().filter(|c| *c == '.').count() == 1);
  let com_sig = RwSignal::new(comments_clone);
  let child_show = RwSignal::new(true);

  // logging::log!("20");

  let now_in_millis = {
    #[cfg(not(feature = "ssr"))]
    {
      chrono::offset::Utc::now().timestamp_millis() as u64
    }
    #[cfg(feature = "ssr")]
    {
      std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64 
    }
  };

  // logging::log!("21");


  view! {
    <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
      <CommentNode show=child_show comment_view=cv.into() comments=comments.get().into() level=1 now_in_millis/>
    </For>
  }
}
