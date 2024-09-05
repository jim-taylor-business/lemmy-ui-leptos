#[cfg(not(feature = "ssr"))]
use crate::indexed_db::*;
use crate::ui::components::comment::comment_node::CommentNode;
use lemmy_api_common::lemmy_db_views::structs::CommentView;
use leptos::*;

#[component]
pub fn CommentNodes(
  comments: MaybeSignal<Vec<CommentView>>,
  post_id: MaybeSignal<Option<i32>>,
  // hidden_comments: RwSignal<Vec<i32>>,
) -> impl IntoView {
  let mut comments_clone = comments.get().clone();
  comments_clone.retain(|ct| ct.comment.path.chars().filter(|c| *c == '.').count() == 1);
  let com_sig = RwSignal::new(comments_clone);
  // let child_show = RwSignal::new(true);

  // logging::log!("20");

  let now_in_millis = {
    #[cfg(not(feature = "ssr"))]
    {
      chrono::offset::Utc::now().timestamp_millis() as u64
    }
    #[cfg(feature = "ssr")]
    {
      std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
    }
  };

  let hidden_comments: RwSignal<Vec<i32>> = RwSignal::new(vec![]);

  #[cfg(not(feature = "ssr"))]
  let hidden_comments_resource = create_local_resource(
    move || (),
    move |()| async move {
      if let Some(p) = post_id.get() {
        // if let Some(u) = u32::try_from(p).ok() {
        if let Ok(d) = build_comment_database().await {
          // logging::log!("ID {}", u);
          if let Ok(Some(CommentRquest { comment_id, .. })) = get_array(&d, p).await {
            hidden_comments.set(comment_id);
          } else {
          }
          // } else {
          // }
        } else {
        }
      } else {
      }
    },
  );

  // logging::log!("21");
  let on_hide_show = move |i: i32| {
    // child_show.update(|value| *value = !*value);
    //         {move || {
    if hidden_comments.get().contains(&i) {
      hidden_comments.update(|hc| hc.retain(|c| i != *c));
      logging::log!("unthing {} {:?}", i, hidden_comments.get());
    } else {
      hidden_comments.update(|hc| hc.push(i));
      logging::log!("thing {} {:?}", i, hidden_comments.get());
    }
    let hidden_comments_resource = create_local_resource(
      move || (),
      move |()| async move {
        if let Some(p) = post_id.get() {
          #[cfg(not(feature = "ssr"))]
          if let Ok(d) = build_comment_database().await {
            if let Ok(_) = add_array(&d, p, hidden_comments.get()).await {
              logging::log!("yay");
            } else {
              logging::log!("boo");
            }
          } else {
          }
        } else {
        }
      },
    );
  };

  view! {
    <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
      <CommentNode parent_comment_id=0 hidden_comments=hidden_comments on_toggle=on_hide_show comment_view=cv.into() comments=comments.get().into() level=1 now_in_millis/>
    </For>
  }
}
