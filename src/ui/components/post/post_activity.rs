use crate::{
  errors::LemmyAppError, lemmy_client::*, ui::components::{comment::comment_nodes::CommentNodes, post::post_listing::PostListing}, TitleSetter
};
use lemmy_api_common::{comment::GetComments, lemmy_db_schema::{newtypes::PostId, CommentSortType}, post::GetPost, site::GetSiteResponse};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn PostActivity(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let params = use_params_map();

  let post_id = move || params.get().get("id").cloned().unwrap_or_default();
  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();

  let post = create_resource(post_id, move |id_string| async move {
    if let Ok(id) = id_string.parse::<i32>() {

    let form = GetPost {
      id: Some(PostId(id)),
      comment_id: None,
    };

    let result = LemmyClient.get_post(form).await;

    match result {
      Ok(o) => {
        Some(o)
      },
      Err(e) => {
        error.set(Some(e));
        None
      }
    }

    } else {
      None
    }
  });

  let comments = create_resource(post_id, move |id_string| async move {
    if let Ok(id) = id_string.parse::<i32>() {

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
        error.set(Some(e));
        None
      }
    }

    } else {
      None
    }
  });

  #[cfg(not(feature = "ssr"))]
  {
    let on_resize = move |_| { };
    window_event_listener_untyped("resize", on_resize);
    let on_scroll = move |_| { };
    window_event_listener_untyped("scroll", on_scroll);
  }

  view! {
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="flex flex-col ">
        <div>
          <Transition fallback=|| {
              view! { "Loading..." }
          }>
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
                          <PostListing post_view=res.post_view.into() site_signal post_number=0/>
                        </div>
                        {
                          if let Some(ref refer) = text {
                            // let refer = &;
                            let parser = pulldown_cmark::Parser::new(refer);
                            let mut html = String::new();
                            pulldown_cmark::html::push_html(&mut html, parser);                    

                            view! {
                              <div class="pl-4 pr-4">
                                <div class="py-2">
                                  <div class="prose max-w-none prose-hr:my-2 prose-img:w-24 prose-img:my-2 prose-p:my-0 prose-p:mb-1 prose-ul:my-0 prose-blockquote:my-0 prose-blockquote:mb-1 prose-li:my-0" inner_html=html/>
                                </div>
                              </div>
                            }
                          } else {
                            view! { <div class="hidden"></div> }
                          }
                        }
                      }
                    })
            }}
          </Transition>
          <Transition fallback=|| {
              view! { "Loading..." }
          }>
            {move || {
                comments
                    .get()
                    .unwrap_or(None)
                    .map(|res| 
                      view! {
                        <div class="w-full">
                          <CommentNodes comments=res.comments.into()/>
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
