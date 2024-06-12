use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNode(
  comment_view: MaybeSignal<CommentView>,
  comments: MaybeSignal<Vec<CommentView>>,
  level: usize,
) -> impl IntoView {
  let mut comments_descendants = comments.get().clone();
  let id = comment_view.get().comment.id.to_string();

  let mut comments_children: Vec<CommentView> = vec![];

  comments_descendants.retain(|ct| {
    let tree = ct.comment.path.split('.').collect::<Vec<_>>();
    if tree.len() == level + 2 {
      if tree.get(level).unwrap_or(&"").eq(&id) {
        comments_children.push(ct.clone());
      }
      false
    } else if tree.len() > level + 2 {
      tree.get(level).unwrap_or(&"").eq(&id)
    } else {
      false
    }
  });
  
  let com_sig = RwSignal::new(comments_children);
  let des_sig = RwSignal::new(comments_descendants);

  view! {
    <div class="pl-4">
      <div class="pb-2">
        <span> { comment_view.get().comment.content }</span>
        " - "
        <span class="font-bold"> { comment_view.get().creator.name }</span>
      </div>
      <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
        <CommentNode comment_view=cv.into() comments=des_sig.get().into() level=level + 1/>
      </For>
    </div>
  }
}
