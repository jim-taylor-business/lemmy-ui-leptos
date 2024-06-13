use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

// use crate::PARSER;

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

  let refer = &comment_view.get().comment.content;
  let parser = pulldown_cmark::Parser::new(refer);
  let mut html = String::new();
  pulldown_cmark::html::push_html(&mut html, parser);

  // let ast  = PARSER.parse(&comment_view.get().comment.content);
  // let html = ast.render();

  view! {
    <div class="pl-4">
      <div class="pb-2">
      // <span> { comment_view.get().comment.content }</span>
      <div class="prose max-w-none prose-img:w-24" inner_html=html/>
        // <div inner_html=html/>
      // <div class="inline-block"> " - " </div>
      // <span class="font-bold"> { comment_view.get().creator.name }</span>
        // <div class="inline-block italic"> " - " { comment_view.get().creator.name }</div>
      </div>
      <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
        <CommentNode comment_view=cv.into() comments=des_sig.get().into() level=level + 1/>
      </For>
    </div>
  }
}
