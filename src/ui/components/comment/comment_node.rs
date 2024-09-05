use std::any::Any;

use crate::{
  errors::{LemmyAppError, LemmyAppErrorType},
  lemmy_client::*,
  ui::components::common::icon::{Icon, IconType::*},
};
use ev::{MouseEvent, SubmitEvent, TouchEvent};
use lemmy_api_common::{
  comment::{CreateCommentLike, SaveComment},
  lemmy_db_views::structs::CommentView,
  post::{CreatePostLike, PostResponse},
};
use leptos::*;
use leptos_dom::helpers::TimeoutHandle;
use leptos_router::{Form, A};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlImageElement, HtmlLinkElement};

#[component]
pub fn CommentNode(
  //<F>(
  comment_view: MaybeSignal<CommentView>,
  comments: MaybeSignal<Vec<CommentView>>,
  level: usize,
  // show: MaybeSignal<bool>,
  parent_comment_id: i32,
  now_in_millis: u64,
  hidden_comments: RwSignal<Vec<i32>>,
  // on_toggle: F,
  #[prop(into)] on_toggle: Callback<i32>,
) -> impl IntoView
// where
//   F: Fn(i32) + 'static,
{
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

  let content = &comment_view.get().comment.content;

  let mut options = pulldown_cmark::Options::empty();
  options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
  options.insert(pulldown_cmark::Options::ENABLE_TABLES);
  let parser = pulldown_cmark::Parser::new_ext(content, options);

  // let parser = pulldown_cmark::Parser::new(content);
  let custom = parser.map(|event| match event {
    pulldown_cmark::Event::Html(text) => {
      let er = format!("<p>{}</p>", html_escape::encode_safe(&text).to_string());
      // logging::log!("pc h {}", text);
      pulldown_cmark::Event::Html(er.into())
    }
    pulldown_cmark::Event::InlineHtml(text) => {
      let er = html_escape::encode_safe(&text).to_string();
      // logging::log!("pc i {}", text);
      pulldown_cmark::Event::InlineHtml(er.into())
    }
    _ => {
      // logging::log!("pc o {:?}", event);
      event
    }
  });
  let mut safe_html = String::new();
  pulldown_cmark::html::push_html(&mut safe_html, custom);

  // let child_show = RwSignal::new(true);

  let back_show = RwSignal::new(false);

  let still_down = RwSignal::new(false);
  let vote_show = RwSignal::new(false);
  let still_handle: RwSignal<Option<TimeoutHandle>> = RwSignal::new(None);

  let comment_view = RwSignal::new(comment_view.get());

  // let show = move || {
  //   if hidden_comments.get().contains(&parent_comment_id) {
  //     false
  //   } else {
  //     true
  //   }
  // };

  // let child_show = move || {
  //   if hidden_comments
  //     .get()
  //     .contains(&comment_view.get().comment.id.0)
  //   {
  //     false
  //   } else {
  //     true
  //   }
  // };

  let duration_in_text = pretty_duration::pretty_duration(
    &std::time::Duration::from_millis(
      now_in_millis - comment_view.get().post.published.timestamp_millis() as u64,
    ),
    Some(pretty_duration::PrettyDurationOptions {
      output_format: Some(pretty_duration::PrettyDurationOutputFormat::Compact),
      singular_labels: None,
      plural_labels: None,
    }),
  );
  let abbr_duration = if let Some((index, _)) = duration_in_text.match_indices(' ').nth(1) {
    duration_in_text.split_at(index)
  } else {
    (&duration_in_text[..], "")
  }
  .0
  .to_string();

  let error = expect_context::<RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>>();

  let cancel = move |ev: MouseEvent| {
    ev.stop_propagation();
  };

  let on_vote_submit = move |ev: SubmitEvent, score: i16| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let form = CreateCommentLike {
          comment_id: comment_view.get().comment.id,
          score,
        };

        let result = LemmyClient.like_comment(form).await;

        match result {
          Ok(o) => {
            comment_view.set(o.comment_view);
          }
          Err(e) => {
            error.update(|es| es.push(Some((e, None))));
            // error.set(Some((e, None)));
          }
        }
      },
    );
  };

  let on_up_vote_submit = move |ev: SubmitEvent| {
    let score = if Some(1) == comment_view.get().my_vote {
      0
    } else {
      1
    };
    on_vote_submit(ev, score);
  };

  let on_down_vote_submit = move |ev: SubmitEvent| {
    let score = if Some(-1) == comment_view.get().my_vote {
      0
    } else {
      -1
    };
    on_vote_submit(ev, score);
  };

  let on_save_submit = move |ev: SubmitEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        let form = SaveComment {
          comment_id: comment_view.get().comment.id,
          save: !comment_view.get().saved,
        };

        let result = LemmyClient.save_comment(form).await;

        match result {
          Ok(o) => {
            comment_view.set(o.comment_view);
          }
          Err(e) => {
            error.update(|es| es.push(Some((e, None))));
            // error.set(Some((e, None)));
          }
        }
      },
    );
  };

  // let on_this_toggle = move |i: i32| {
  //   child_show.set(!child_show.get());
  // };

  view! {
    <div
      // on:mouseover=move |e: MouseEvent| { e.stop_propagation(); back_show.set(!back_show.get()); }
      // on:mouseout=move |e: MouseEvent| { e.stop_propagation(); back_show.set(!back_show.get()); }
      class=move || format!("pl-4{}{}{}", if level == 1 { " odd:bg-base-200 pr-4 pt-2 pb-1" } else { "" }, if !hidden_comments.get().contains(&parent_comment_id) { "" } else { " hidden" }, if back_show.get() { " bg-base-300" } else { "" })
    >
      <div on:click=move |e: MouseEvent| {
        if still_down.get() {
          still_down.set(false);
        } else {
          if let Some(t) = e.target() {
            if let Some(i) = t.dyn_ref::<HtmlImageElement>() {
              let _ = window().location().set_href(&i.src());
            } else if let Some(l) = t.dyn_ref::<HtmlAnchorElement>() {

            } else {
              // child_show.set(!child_show.get());
              on_toggle.call(comment_view.get().comment.id.0);
              // on_hide(1);
            }
          }
        }
      } on:mousedown=move |e: MouseEvent| {
        still_handle.set(set_timeout_with_handle(move || {
          vote_show.set(!vote_show.get());
          still_down.set(true);
        }, std::time::Duration::from_millis(500)).ok());
      } on:touchstart=move |e: TouchEvent| {
        still_handle.set(set_timeout_with_handle(move || {
          vote_show.set(!vote_show.get());
          still_down.set(true);
        }, std::time::Duration::from_millis(500)).ok());
      } on:touchend=move |e: TouchEvent| {
        if let Some(h) = still_handle.get() {
          h.clear();
        }
      } on:touchmove=move |e: TouchEvent| {
        if let Some(h) = still_handle.get() {
          h.clear();
        }
      } on:mouseup=move |e: MouseEvent| {
        if let Some(h) = still_handle.get() {
          h.clear();
        }
      } on:dblclick=move |e: MouseEvent| {
        vote_show.set(!vote_show.get());
      } class="pb-2 cursor-pointer">
        // DEBUG
        // <div>{ move || format!("{:?}", hidden_comments.get())}</div>
        <div
          class="prose max-w-none"
          inner_html=safe_html
        />
        <Show when=move || vote_show.get() fallback=|| view! {  }>
          <div on:click=cancel class="flex items-center gap-x-2">
          <Form
            on:submit=on_up_vote_submit
            action="POST"
            class="flex items-center"
          >
            <input type="hidden" name="post_id" value=format!("{}", comment_view.get().post.id)/>
            <input
              type="hidden"
              name="score"
              value=move || if Some(1) == comment_view.get().my_vote { 0 } else { 1 }
            />
            <button
              type="submit"
              class=move || { if Some(1) == comment_view.get().my_vote { " text-secondary" } else { "" } }
              title="Up vote"
            >
              <Icon icon=Upvote/>
            </button>
          </Form>
          <span class="text-sm">{move || comment_view.get().counts.score}</span>
          <Form
            on:submit=on_down_vote_submit
            action="POST"
            on:submit=|_| {}
            class="flex items-center"
          >
            <input type="hidden" name="post_id" value=format!("{}", comment_view.get().post.id)/>
            <input
              type="hidden"
              name="score"
              value=move || if Some(-1) == comment_view.get().my_vote { 0 } else { -1 }
            />
            <button
              type="submit"
              class=move || {
                  if Some(-1) == comment_view.get().my_vote { " text-primary" } else { "" }
              }

              title="Down vote"
            >
              <Icon icon=Downvote/>
            </button>
          </Form>
          <span class="text-base-content/50" title="Reply">
            <Icon icon=Reply/>
          </span>
          <Form
            action="POST"
            on:submit=on_save_submit
            class="flex items-center"
          >
            <input type="hidden" name="post_id" value=format!("{}", comment_view.get().post.id)/>
            <input type="hidden" name="save" value=move || format!("{}", !comment_view.get().saved)/>
            <button
              type="submit"
              title="Save post"
              class=move || if comment_view.get().saved { "text-primary hover:text-primary/50" } else { "hover:text-primary/50" }
            >
              <Icon icon=Save/>
            </button>
          </Form>
          <span class="mb-1 break-words overflow-hidden">
            <span>
              { abbr_duration.clone() }
            </span> " ago, by "
            <A
              href=move || format!("/u/{}", comment_view.get().creator.name)
              class="text-sm hover:text-secondary break-words"
            >
              {comment_view.get().creator.name}
            </A>
          </span>
          </div>
        </Show>
        <span class=move || format!("badge badge-neutral inline-block whitespace-nowrap{}", if hidden_comments
          .get()
          .contains(&comment_view.get().comment.id.0) && com_sig.get().len() > 0 { "" } else { " hidden" })>
          { com_sig.get().len() + des_sig.get().len() } " replies"
        </span>
      </div>
      // {move || {
      //   if hidden_comments.get().contains(&comment_view.get().comment.id.0) {
      //     child_show.set(false);
      //   }
      // }}
      <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
        <CommentNode parent_comment_id=comment_view.get().comment.id.0 hidden_comments=hidden_comments on_toggle comment_view=cv.into() comments=des_sig.get().into() level=level + 1 now_in_millis/>
      </For>
    </div>
  }
}
