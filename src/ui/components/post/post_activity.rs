use crate::{
  errors::LemmyAppError,
  lemmy_client::*,
  ui::components::{comment::comment_nodes::CommentNodes, post::post_listing::PostListing},
  TitleSetter,
};

use ev::MouseEvent;
use lemmy_api_common::{
  comment::{CreateComment, GetComments},
  lemmy_db_schema::{newtypes::PostId, CommentSortType},
  post::GetPost,
  site::GetSiteResponse,
};
use leptos::*;
use leptos_router::use_params_map;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlImageElement};

#[component]
pub fn PostActivity(
  site_signal: Resource<Option<bool>, Result<GetSiteResponse, LemmyAppError>>, //RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>
) -> impl IntoView {
  let params = use_params_map();
  let post_id = move || params.get().get("id").cloned().unwrap_or_default().parse::<i32>().ok();
  let error = expect_context::<RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>>();
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();

  let reply_show = RwSignal::new(false);
  let refresh_comments = RwSignal::new(false);
  let content = RwSignal::new(String::default());

  let post = Resource::new(post_id, move |id_string| async move {
    if let Some(id) = id_string {
      let form = GetPost {
        id: Some(PostId(id)),
        comment_id: None,
      };

      let result = LemmyClient.get_post(form).await;

      match result {
        Ok(o) => Some(o),
        Err(e) => {
          error.update(|es| es.push(Some((e, None))));
          None
        }
      }
    } else {
      None
    }
  });

  let comments = Resource::new(
    move || (post_id(), refresh_comments.get()),
    move |(post_id, _refresh)| async move {
      if let Some(id) = post_id {
        let form = GetComments {
          post_id: Some(PostId(id)),
          community_id: None,
          type_: None,
          sort: Some(CommentSortType::Top),
          max_depth: Some(8),
          page: None,
          limit: None,
          community_name: None,
          parent_id: None,
          saved_only: None,
          disliked_only: None,
          liked_only: None,
        };

        let result = LemmyClient.get_comments(form).await;

        match result {
          Ok(o) => Some(o),
          Err(e) => {
            error.update(|es| es.push(Some((e, None))));
            None
          }
        }
      } else {
        None
      }
    },
  );

  let on_reply_click = move |ev: MouseEvent| {
    ev.prevent_default();

    create_local_resource(
      move || (),
      move |()| async move {
        if let Some(id) = post_id() {
          let form = CreateComment {
            content: content.get(),
            post_id: PostId(id),
            parent_id: None,
            language_id: None,
          };

          let result = LemmyClient.reply_comment(form).await;

          match result {
            Ok(o) => {
              refresh_comments.update(|b| *b = !*b);
              reply_show.update(|b| *b = !*b);
            }
            Err(e) => {
              error.update(|es| es.push(Some((e, None))));
            }
          }
        }
      },
    );
  };

  view! {
    <main role="main" class="w-full flex flex-col flex-grow">
      <div class="flex flex-col">
        <div>
          <Transition fallback=|| { }>
            {move || {
                post.get()
                    .unwrap_or(None)
                    .map(|res| {
                      ui_title.set(Some(TitleSetter(res.post_view.post.name.clone())));
                      let text = if let Some(b) = res.post_view.post.body.clone() {
                        if b.len() > 0 {
                          Some(b)
                        } else {
                          res.post_view.post.embed_description.clone()
                        }
                      } else {
                        None
                      };

                      view! {
                        <div>
                          <PostListing post_view=res.post_view.into() site_signal post_number=0 reply_show />
                        </div>
                        {
                          if let Some(ref content) = text {
                            let mut options = pulldown_cmark::Options::empty();
                            options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
                            options.insert(pulldown_cmark::Options::ENABLE_TABLES);
                            let parser = pulldown_cmark::Parser::new_ext(content, options);
                            // let parser = pulldown_cmark::Parser::new(content);
                            let custom = parser.map(|event| match event {
                              pulldown_cmark::Event::Html(text) => {
                                let er = format!("<p>{}</p>",  html_escape::encode_safe(&text).to_string());
                                pulldown_cmark::Event::Html(er.into())
                              }
                              pulldown_cmark::Event::InlineHtml(text) => {
                                let er = html_escape::encode_safe(&text).to_string();
                                pulldown_cmark::Event::InlineHtml(er.into())
                              }
                              _ => event
                            });
                            let mut safe_html = String::new();
                            pulldown_cmark::html::push_html(&mut safe_html, custom);

                            Some(view! {
                              <div class="pl-4 pr-4">
                                <div class="py-2"  on:click=move |e: MouseEvent| {
                                  if let Some(t) = e.target() {
                                    if let Some(i) = t.dyn_ref::<HtmlImageElement>() {
                                      let _ = window().location().set_href(&i.src());
                                    } else if let Some(l) = t.dyn_ref::<HtmlAnchorElement>() {
                                    }
                                  }
                                }>
                                  <div class="prose max-w-none"
                                    inner_html=safe_html
                                  />
                                </div>
                              </div>
                            })
                          } else {
                            None
                            // view! { <div class="hidden"></div> }
                          }
                        }
                        <Show when=move || reply_show.get() fallback=|| {}>
                          <div class="space-y-3 mb-3">
                            <label class="form-control">
                              <textarea class="textarea textarea-bordered text-base h-24" placeholder="Comment text" prop:value={ move || content.get() } on:input={ move |ev| content.set(event_target_value(&ev)) } >{ content.get_untracked() }</textarea>
                            </label>
                            <button on:click=on_reply_click type="button" class="btn btn-neutral" /*class=format!("btn btn-neutral{}", if content.get().len() > 0 { "" } else { " btn-disabled" })*/>
                              "Comment"
                            </button>
                          </div>
                        </Show>
                      }
                    })
            }}
          </Transition>
          <Transition fallback=|| { }>
            {move || {
                comments
                    .get()
                    .unwrap_or(None)
                    .map(|res|
                      view! {
                        <div class="w-full">
                          <CommentNodes comments=res.comments.into() post_id=post_id().into() />
                        </div>
                      }
                    )
            }}
          </Transition>
        </div>
      </div>
    </main>
  }
}
