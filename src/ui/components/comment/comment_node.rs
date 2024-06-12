use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(
  comment_view: MaybeSignal<CommentView>,
  comments: MaybeSignal<Vec<CommentView>>,
  level: usize,
) -> impl IntoView {
  let mut comments_clone = comments.get().clone();
  let id = comment_view.get().comment.id.to_string();
  comments_clone.retain(|ct| {
    let tree = ct.comment.path.split('.').collect::<Vec<_>>();
    if tree.len() > level + 1 {
      tree.get(level).unwrap_or(&"").eq(&id)
    } else {
      false
    }
  });
  let com_sig = RwSignal::new(comments_clone);

  // if level > 3 { return view! { <div> </div> }; }

  view! {
    <div class="pl-4">
      <div class="pb-2">
        <span> { comment_view.get().comment.content }</span>
        " - "
        <span class="font-bold"> { comment_view.get().creator.name }</span>
      </div>
      <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
        <CommentNode comment_view=cv.into() comments=comments.get().into() level=level + 1/>
      </For>
    </div>
  }
}
