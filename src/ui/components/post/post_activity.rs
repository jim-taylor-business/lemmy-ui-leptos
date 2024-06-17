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
        // let tit = match site_signal.get() {
        //   Some(Ok(site)) => {
        //     logging::log!("oofts");
        //     // let q = if let Some(TitleSetter(t)) = ui_t { t } else { "", to_string() };
        //     if let Some(d) = site.site_view.site.description {
        //       format!("{} - Tech Demo UI for {} - {}", o.post_view.post.name.clone(), site.site_view.site.name, d)
        //     } else {
        //       format!("{} - Tech Demo UI for {}", o.post_view.post.name.clone(), site.site_view.site.name)
        //     }
        //   }
        //   _ => "Lemmy".to_string(),
        // };
  
        // ui_title.set(Some(TitleSetter(tit)));
        // ui_title.set(Some(TitleSetter(o.post_view.post.name.clone())));
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

  view! {
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="flex flex-col ">
        <div class="columns-1 2xl:columns-2 4xl:columns-3 gap-3">
          <Transition fallback=|| {
              view! { "Loading..." }
          }>
            {move || {
                post.get()
                    .unwrap_or(None)
                    .map(|res| {
                      ui_title.set(Some(TitleSetter(res.post_view.post.name.clone())));
                      view! {
                        <table class="table">
                          <PostListing post_view=res.post_view.into() site_signal post_number=RwSignal::new(0usize)/>
                        </table>
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
