use ev::MouseEvent;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

// use crate::PARSER;

#[component]
pub fn CommentNode(
  comment_view: MaybeSignal<CommentView>,
  comments: MaybeSignal<Vec<CommentView>>,
  level: usize,
  show: RwSignal<bool>,
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

  let refer = &comment_view.get().comment.content;
  let parser = pulldown_cmark::Parser::new(refer);
  let mut html = String::new();
  pulldown_cmark::html::push_html(&mut html, parser);

  let child_show = RwSignal::new(true);
  let back_show = RwSignal::new(false);

  // let ast  = PARSER.parse(&comment_view.get().comment.content);
  // let html = ast.render();

  view! {
    <div on:mouseover=move |e: MouseEvent| { e.stop_propagation(); back_show.set(!back_show.get()); } on:mouseout=move |e: MouseEvent| { e.stop_propagation(); back_show.set(!back_show.get()); } /* class="pl-4"  */class=move || format!("pl-4{}{}", if show.get() { "" } else { " hidden" }, if back_show.get() { " bg-base-300" } else { "" }) >
      // <button on:mouseup=move |_: MouseEvent| { logging::log!("ohye"); }> "hide" </button>
      <div class="cursor-pointer pb-2">
        <div on:mousedown=move |_| { child_show.set(!child_show.get()); } class="prose max-w-none prose-img:w-24 prose-img:my-2 prose-p:my-0 prose-p:mb-1 prose-ul:my-0 prose-blockquote:my-0 prose-blockquote:mb-1 prose-li:my-0" inner_html=html/>
      </div>
      <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
        <CommentNode show=child_show comment_view=cv.into() comments=des_sig.get().into() level=level + 1/>
      </For>
    </div>
  }
}
