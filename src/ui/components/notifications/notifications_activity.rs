use std::collections::BTreeMap;

use ev::MouseEvent;
use lemmy_api_common::{
  lemmy_db_schema::{newtypes::CommentReplyId, source::comment_reply::CommentReply, CommentSortType},
  lemmy_db_views::structs::{CommentView, PaginationCursor},
  lemmy_db_views_actor::structs::CommentReplyView,
  person::{GetPersonMentions, GetReplies, MarkCommentReplyAsRead},
  post::{GetPosts, GetPostsResponse},
  private_message::GetPrivateMessages,
};
use leptos::*;
use leptos_router::{use_location, use_query_map, A};

use crate::{
  errors::{message_from_error, LemmyAppError},
  ui::components::{comment::comment_node::CommentNode, common::about::About},
  LemmyApi, LemmyClient, NotificationsRefresh, TitleSetter,
};

#[component]
pub fn NotificationsActivity() -> impl IntoView {
  let errors = expect_context::<RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>>();
  let notifications_refresh = expect_context::<RwSignal<NotificationsRefresh>>();
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();

  ui_title.set(Some(TitleSetter("Notifications".into())));

  let replies_refresh = RwSignal::new(true);

  let replies = Resource::new(
    move || (replies_refresh.get()),
    move |(_replies_refresh)| async move {
      let form = GetReplies {
        sort: Some(CommentSortType::New),
        page: Some(1),
        limit: Some(10),
        unread_only: Some(true),
      };
      let result = LemmyClient.replies_user(form).await;
      match result {
        Ok(o) => Some(o),
        Err(e) => {
          errors.update(|es| es.push(Some((e, None))));
          None
        }
      }
    },
  );

  let mentions = Resource::new(
    move || (),
    move |()| async move {
      let form = GetPersonMentions {
        sort: Some(CommentSortType::New),
        page: Some(1),
        limit: Some(10),
        unread_only: Some(true),
      };
      let result = LemmyClient.mention_user(form).await;
      match result {
        Ok(o) => Some(o),
        Err(e) => {
          errors.update(|es| es.push(Some((e, None))));
          None
        }
      }
    },
  );

  let messages = Resource::new(
    move || (),
    move |()| async move {
      let form = GetPrivateMessages {
        page: Some(1),
        limit: Some(10),
        unread_only: Some(true),
        creator_id: None,
      };
      let result = LemmyClient.messages_user(form).await;
      match result {
        Ok(o) => Some(o),
        Err(e) => {
          errors.update(|es| es.push(Some((e, None))));
          None
        }
      }
    },
  );

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

  let on_hide_show = |_| {};

  let on_clear_reply_click = move |id: CommentReplyId| {
    move |_e: MouseEvent| {
      // move |ev: ClickEvent, id: i32| {
      create_local_resource(
        move || (),
        move |()| async move {
          let form = MarkCommentReplyAsRead {
            comment_reply_id: id,
            read: true,
          };

          let result = LemmyClient.mark_comment(form).await;

          match result {
            Ok(o) => {
              replies_refresh.update(|b| *b = !*b);
              notifications_refresh.update(|n| n.0 = !n.0);
            }
            Err(e) => {
              errors.update(|es| es.push(Some((e, None))));
            }
          }
        },
      );
    }
  };

  view! {
    <main class="mx-auto">
      <Transition fallback=|| { }>
        {move || {
            replies
                .get()
                .unwrap_or(None)
                .map(|g| {
                  view! {
                    <div class="w-full">
                      <For each=move || g.replies.clone() key=|r| r.comment.id let:r>
                        {
                          let c = CommentView {
                            comment: r.comment,
                            creator: r.creator,
                            post: r.post,
                            community: r.community,
                            counts: r.counts,
                            creator_banned_from_community: false,
                            creator_is_moderator: r.creator_is_moderator,
                            creator_is_admin: r.creator_is_admin,
                            subscribed: r.subscribed,
                            saved: r.saved,
                            creator_blocked: r.creator_blocked,
                            my_vote: r.my_vote,
                          };

                          view! {
                            <div class="mb-6">
                              <CommentNode parent_comment_id=0 hidden_comments=RwSignal::new(vec![]) on_toggle=on_hide_show comment_view=c.into() comments=vec![].into() level=1 now_in_millis/>
                              <button class="btn btn-sm" on:click=on_clear_reply_click(r.comment_reply.id)> "Clear" </button>
                            </div>
                          }
                        }
                      </For>
                    </div>
                  }
                })
        }}
      </Transition>
      <Transition fallback=|| { }>
        {move || {
            mentions
                .get()
                .unwrap_or(None)
                .map(|m| {
                  if m.mentions.len() > 0 {
                    view! {
                      <div class="w-full">
                          <div class="px-8 mb-6">
                              <div class="alert">
                                <span> {m.mentions.len()} " mentions" </span>
                              </div>
                          </div>
                      </div>
                    }
                  } else {
                    view! {
                      <div class="hidden"/>
                    }
                  }
                })
        }}
      </Transition>
      <Transition fallback=|| { }>
        {move || {
            messages
                .get()
                .unwrap_or(None)
                .map(|p| {
                  if p.private_messages.len() > 0 {
                    view! {
                      <div class="w-full">
                          <div class="px-8 mb-6">
                              <div class="alert">
                                <span> {p.private_messages.len()} " messages" </span>
                              </div>
                          </div>
                      </div>
                    }
                  } else {
                    view! {
                      <div class="hidden"/>
                    }
                  }
                })
        }}
      </Transition>
      {move || {
        errors.get().into_iter().enumerate().map(|(i, error)| {
          error.map(|err| {
            view! {
              <div class="px-8 mb-6">
                <div class="alert alert-error flex justify-between">
                  <span>{message_from_error(&err.0)} " - " {err.0.content}</span>
                  <div>
                    <Show when=move || { if let Some(r) = err.1 { true } else { false } } /* let:r */ fallback=|| {}>
                      <button on:click=move |_| { if let Some(r) = err.1 { r.set(!r.get()); } else { } } class="btn btn-sm"> "Retry" </button>
                    </Show>
                    <button class="btn btn-sm" on:click=move |_| { errors.update(|es| { es.remove(i); }); }> "Clear" </button>
                  </div>
                </div>
              </div>
            }
          })
        }).collect::<Vec<_>>()
      }}
      <div class="px-8 mb-6">
        <button class=move || format!("btn{}", if errors.get().len() > 0 { "" } else { " hidden" }) on:click=move |_| { errors.set(vec![]); }> "Clear All Errors" </button>
      </div>
      <About />
    </main>
  }
}
