use ev::{MouseEvent, SubmitEvent};
use lemmy_api_common::{comment::{CreateCommentLike, SaveComment}, lemmy_db_views::structs::CommentView, post::{CreatePostLike, PostResponse}};
use leptos::*;
use leptos_dom::helpers::TimeoutHandle;
use leptos_router::{Form, A};
use web_sys::HtmlImageElement;
use web_sys::wasm_bindgen::JsCast;
use crate::{
  errors::{LemmyAppError, LemmyAppErrorType},
  lemmy_client::*,
  ui::components::common::icon::{
    Icon,
    IconType::*,
  },
};

#[component]
pub fn CommentNode(
  comment_view: MaybeSignal<CommentView>,
  comments: MaybeSignal<Vec<CommentView>>,
  level: usize,
  show: RwSignal<bool>,
  now_in_millis: u64,
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

  // html.push_str(&format!("<A href=/u/{} class=\"text-sm hover:text-secondary break-words\">{}</A>", comment_view.get().creator.name, comment_view.get().creator.name));

  let child_show = RwSignal::new(true);
  let back_show = RwSignal::new(false);

  // let down = RwSignal::new(false);
  let still_down = RwSignal::new(false);
  let vote_show = RwSignal::new(false);
  let still_handle: RwSignal<Option<TimeoutHandle>> = RwSignal::new(None);

  let comment_view = create_rw_signal(comment_view.get());

  let duration_in_text = pretty_duration::pretty_duration(
    &std::time::Duration::from_millis(now_in_millis - comment_view.get().post.published.timestamp_millis() as u64),
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
  }.0.to_string();

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();

  let cancel = move |ev: MouseEvent| {
    ev.stop_propagation();
    // ev.cancel_bubble();
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
            error.set(Some(e));
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
            error.set(Some(e));
          }
        }
      },
    );
  };


  view! {
    <div 
      // on:mouseover=move |e: MouseEvent| { e.stop_propagation(); back_show.set(!back_show.get()); } 
      // on:mouseout=move |e: MouseEvent| { e.stop_propagation(); back_show.set(!back_show.get()); } 
      class=move || format!("pl-4{}{}{}", if level == 1 { " odd:bg-base-200 pr-4 pt-2 pb-1" } else { "" }, if show.get() { "" } else { " hidden" }, if back_show.get() { " bg-base-300" } else { "" }) 
    >
      <div on:click=move |e: MouseEvent| {
        if still_down.get() {
          still_down.set(false);
        } else {
          if let Some(t) = e.target() {
            if let Some(i) = t.dyn_ref::<HtmlImageElement>() {
                let _ = window().location().set_href(&i.src());
            } else {
                child_show.set(!child_show.get());
            }
          }
        }
        // down.set(false);
      } on:mousedown=move |e: MouseEvent| {
        // down.set(true);
        still_handle.set(set_timeout_with_handle(move || {
          // if down.get() {
            // logging::log!("still down");
            vote_show.set(!vote_show.get());
            still_down.set(true);
            // down.set(false);            
          // }
        }, std::time::Duration::from_millis(500)).ok());
      } on:mouseup=move |e: MouseEvent| {
        if let Some(h) = still_handle.get() {
          h.clear();
        }
        // down.set(false);
      } on:dblclick=move |e: MouseEvent| {
        vote_show.set(!vote_show.get());
        // logging::log!("still down");
        // down.set(false);
      } class="pb-2 cursor-pointer">
        <div class="prose max-w-none prose-ol:list-inside prose-ol:pl-0 prose-pre:relative prose-pre:h-40 prose-pre:overflow-auto prose-p:break-words prose-hr:my-2 prose-img:w-24 prose-img:my-2 prose-p:leading-6 prose-p:my-0 prose-p:mb-1 prose-ul:my-0 prose-blockquote:my-0 prose-blockquote:mb-1 prose-li:my-0" inner_html=html/>
        // <A
        //   href=format!("/u/{}", comment_view.get().creator.name)
        //   class="text-sm inline-block hover:text-secondary break-words"
        // >
        //   {comment_view.get().creator.name}
        // </A>
        // " "
        <Show when=move || vote_show.get() fallback=|| view! {  }>
        <div on:click=cancel class="flex items-center gap-x-2" /* move || format!("flex items-center gap-x-2{}", if vote_show.get() { "" } else { " hidden"}) */>
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
          <span class="block text-sm">{move || comment_view.get().counts.score}</span>
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
          // <span
          //   class="flex items-center"
          //   title=move || format!("{} comments", comment_view.get().unread_comments)
          // >
          //   <A
          //     href=move || { format!("/post/{}", comment_view.get().post.id) }
          //     class="text-sm whitespace-nowrap hover:text-accent "
          //   >
          //     <Icon icon=Comments class="inline".into()/>
          //     " "
          //     {comment_view.get().counts.comments}
          //     {if comment_view.get().unread_comments != comment_view.get().counts.comments && comment_view.get().unread_comments > 0 { format!(" ({})", comment_view.get().unread_comments) } else { "".to_string() }}
          //   </A>
          // </span>
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
          // <span class="text-base-content/50" title="Cross post" on:click=move |e: MouseEvent| { if e.ctrl_key() && e.shift_key() { let _ = window().location().set_href(&format!("//lemmy.world/post/{}", comment_view.get().post.id)); } }>
          //   // <A href="/create_post">
          //     <Icon icon=Crosspost/>
          //   // </A>
          // </span>
          // <div class="dropdown max-sm:dropdown-end">
          //   <label tabindex="0">
          //     <Icon icon=VerticalDots/>
          //   </label>
          //   <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          //     <li>
          //       <Form
          //         action="POST"
          //         on:submit=|_| {}
          //         // on:submit=on_report_submit 
          //         class="flex flex-col items-start"
          //       >
          //         <input type="hidden" name="post_id" value=format!("{}", comment_view.get().post.id)/>
          //         <button class="text-xs whitespace-nowrap pointer-events-none text-base-content/50" title="Report post" type="submit">
          //           <Icon icon=Notifications class="inline-block".into()/>
          //           "Direct message"
          //         </button>
          //       </Form>
          //     </li>
          //     <li>
          //       <Form
          //         action="POST"
          //         on:submit=|_| {}
          //         // on:submit=on_report_submit 
          //         class="flex flex-col items-start"
          //       >
          //         <input type="hidden" name="post_id" value=format!("{}", comment_view.get().post.id)/>
          //         <input
          //           class="input input-bordered input-disabled"
          //           type="text"
          //           // on:input=move |e| update!(| reason | * reason = event_target_value(& e))
          //           name="reason"
          //           placeholder="reason"
          //         />
          //         <button class="text-xs whitespace-nowrap pointer-events-none text-base-content/50" title="Report post" type="submit">
          //           <Icon icon=Report class="inline-block".into()/>
          //           "Report comment"
          //         </button>
          //       </Form>
          //     </li>
          //     <li>
          //       <Form 
          //         action="POST"
          //         on:submit=|_| {}
          //         // on:submit=on_block_submit
          //       >
          //         <input
          //           type="hidden"
          //           name="person_id"
          //           value=format!("{}", comment_view.get().creator.id.0)
          //         />
          //         <input type="hidden" name="block" value="true"/>
          //         <button class="text-xs whitespace-nowrap pointer-events-none text-base-content/50" title="Block user" type="submit">
          //           <Icon icon=Block class="inline-block".into()/>
          //           "Block user"
          //         </button>
          //       </Form>
          //     </li>
          //   </ul>
          // </div>
          <span class="block mb-1">
            <span>
              { abbr_duration.clone() }
            </span> " ago, by "
            <A
              href=move || format!("/u/{}", comment_view.get().creator.name)
              class="text-sm inline-block hover:text-secondary break-words"
            >
              {comment_view.get().creator.name}
            </A>
          </span>
        </div>
        </Show>
        <span class=move || format!("badge badge-neutral inline-block whitespace-nowrap{}", if !child_show.get() && com_sig.get().len() > 0 { "" } else { " hidden" })>
          { com_sig.get().len() + des_sig.get().len() } " replies"
        </span>
      </div>
      <For each=move || com_sig.get() key=|cv| cv.comment.id let:cv>
        <CommentNode show=child_show comment_view=cv.into() comments=des_sig.get().into() level=level + 1 now_in_millis/>
      </For>
    </div>
  }
}
