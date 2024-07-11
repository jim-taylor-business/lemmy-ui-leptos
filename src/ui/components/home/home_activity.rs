use core::hash;
use std::{collections::BTreeMap, usize, vec};

use crate::{
  errors::LemmyAppError, i18n::*, lemmy_client::*, ui::components::{
    common::about::About, home::{site_summary::SiteSummary, trending::Trending}, post::post_listings::PostListings
  }, TitleSetter
};
use lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  lemmy_db_views::structs::{PaginationCursor, PostView},
  post::{self, GetPosts, GetPostsResponse},
  site::GetSiteResponse,
};
use leptos::*;
use leptos_router::*;
use web_sys::MouseEvent;

#[component]
pub fn HomeActivity(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>();
  let user = expect_context::<RwSignal<Option<bool>>>();
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();

  let query = use_query_map();

  let ssr_list = move || {
    serde_json::from_str::<ListingType>(
      &query
        .get()
        .get("list")
        .cloned()
        .unwrap_or("".into()),
    )
    .unwrap_or(ListingType::All)
  };

  let ssr_sort = move || {
    serde_json::from_str::<SortType>(
      &query
        .get()
        .get("sort")
        .cloned()
        .unwrap_or("".into()),
    )
    .unwrap_or(SortType::Active)
  };

  let ssr_from = move || {
    serde_json::from_str::<(usize, Option<PaginationCursor>)>(
      &query
        .get()
        .get("from")
        .cloned()
        .unwrap_or("".into()),
    )
    .unwrap_or((0usize, None))
  };

  let ssr_prev = move || {
    serde_json::from_str::<Vec<(usize, Option<PaginationCursor>)>>(
      &query
        .get()
        .get("prev")
        .cloned()
        .unwrap_or("".into()),
    )
    .unwrap_or(vec![])
  };

  let ssr_limit = move || {
    query
      .get()
      .get("limit")
      .cloned()
      .unwrap_or("".into())
      .parse::<usize>()
      .unwrap_or(10usize)
  };

  let on_sort_click = move |s: SortType| {
    move |_e: MouseEvent| {
      let r = serde_json::to_string::<SortType>(&s);

      let mut query_params = query.get();

      match r {
        Ok(o) => {
          query_params.insert("sort".into(), o);
        }
        Err(e) => {
          error.set(Some((e.into(), None)));
        }
      }

      if SortType::Active == s {
        query_params.remove("sort".into());
      }

      let navigate = leptos_router::use_navigate();
      navigate(&query_params.to_query_string(), Default::default());}
  };

  let loading = create_rw_signal(false);
  let refresh = create_rw_signal(false);

  ui_title.set(None);

  let posts_resource = create_resource(
    move || {
      (
        refresh.get(),
        user.get(),
        ssr_list(),
        ssr_sort(),
        ssr_from(),
        ssr_limit(),
      )
    },
    move |(_refresh, _user, list_type, sort_type, from, limit)| async move {
      let form = GetPosts {
        type_: Some(list_type),
        sort: Some(sort_type),
        community_name: None,
        community_id: None,
        page: None,
        limit: Some(i64::try_from(limit).unwrap_or(10)),
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        page_cursor: from.1.clone()
        // show_hidden: None,
      };

      let result = LemmyClient.list_posts(form).await;
      loading.set(false);
      ui_title.set(None);

      match result {
        Ok(o) => {
          Some((from, o))
        },
        Err(e) => {
          error.set(Some((e, Some(refresh))));
          None
        }
      }    
    },
  );

  let csr_pages: RwSignal<BTreeMap<usize, GetPostsResponse>> = RwSignal::new(BTreeMap::new());
  let csr_from: RwSignal<Option<(usize, Option<PaginationCursor>)>> = RwSignal::new(None);
  let csr_sort: RwSignal<SortType> = RwSignal::new(SortType::Active);
  let csr_next_page_cursor = create_rw_signal::<Option<(usize, Option<PaginationCursor>)>>(None);

  let on_csr_sort_click = move |s: SortType| {
    move |_e: MouseEvent| {
      csr_from.set(Some((0, None)));
      csr_pages.set(BTreeMap::new());
      csr_sort.set(s);
    }
  };

  let csr_resource = create_local_resource(
    move || {
      (
        // user.get(),
        // ssr_list(),
        csr_sort.get(),
        csr_from.get(),
        // ssr_limit(),
      )
    },
    move |(/* _user, list_type,  */sort_type, csr_from/* , limit */)| async move {
      if let Some(from) = csr_from {
        let form = GetPosts {
          // type_: Some(list_type),
          type_: Some(ListingType::All),
          sort: Some(sort_type),
          community_name: None,
          community_id: None,
          page: None,
          // limit: Some(i64::try_from(limit).unwrap_or(10)),
          limit: Some(10),
          saved_only: None,
          disliked_only: None,
          liked_only: None,
          page_cursor: from.1,
          // show_hidden: None,
        };

        let result = LemmyClient.list_posts(form).await;

        match result {
          Ok(o) => {
            csr_next_page_cursor.set(Some((from.0 + ssr_limit(), o.next_page.clone())));
            csr_pages.update(|h| {
              h.insert(from.0, o); 
            });
            Some(())
          },
          Err(e) => {
            error.set(Some((e, Some(refresh))));
            None
          }
        }
      } else {
        None
      }
    },
  );

  #[cfg(not(feature = "ssr"))]
  {
    let on_resize = move |_| {
      if use_location().pathname.get().eq("/") {
        let iw = window()
          .inner_width()
          .ok()
          .map(|b| b.as_f64().unwrap_or(0.0))
          .unwrap_or(0.0);

        let mut query_params = query.get();

        let prev_limit = if let Some(l) = query_params.get("limit".into()) {
          Some(l.clone())
        } else {
          None
        };

        let new_limit = if iw >= 2560f64 {
          query_params.insert("limit".into(), "40".to_string());
          Some("40".to_string())
        } else if iw >= 1920f64 {
          query_params.insert("limit".into(), "30".to_string());
          Some("30".to_string())
        } else if iw >= 1536f64 {
          query_params.insert("limit".into(), "20".to_string());
          Some("20".to_string())
        } else {
          query_params.remove("limit");
          None
        };

        if iw >= 640f64 {
          csr_pages.set(BTreeMap::new());
          csr_from.set(None);
        } else {
          csr_from.set(Some((0, None)))
        }

        if prev_limit.ne(&new_limit) {
          let navigate = leptos_router::use_navigate();
          if iw >= 640f64 {
            navigate(
              &format!("{}", query_params.to_query_string()),
              Default::default(),
            );
          } else {
            navigate("/",
              Default::default(),
            );
          }
        }
      }
    };

    if let Ok(e) = web_sys::Event::new("resize") {
      on_resize(e);
    }

    let _resize_handle = window_event_listener_untyped("resize", on_resize);

    let on_scroll = move |_| {
      if use_location().pathname.get().eq("/") {
        let iw = window()
          .inner_width()
          .ok()
          .map(|b| b.as_f64().unwrap_or(0.0))
          .unwrap_or(0.0);

        if iw < 640f64 {
          let h = window()
            .inner_height()
            .ok()
            .map(|b| b.as_f64().unwrap_or(0.0))
            .unwrap_or(0.0);
          let o = window().page_y_offset().ok().unwrap_or(0.0);
          let b = f64::from(document().body().map(|b| b.offset_height()).unwrap_or(1));

          let endOfPage = (h + o) >= (b - h);

          if endOfPage {
            csr_from.update(|cf| {
              *cf = csr_next_page_cursor.get();
            }); 
          }
        }
      }
    };

    let _scroll_handle = window_event_listener_untyped("scroll", on_scroll);
  }

  view! {
    <div class="block">
      <div class="join mr-3 hidden sm:inline-block">
        <button class="btn join-item btn-active">"Posts"</button>
        <button class="btn join-item btn-disabled">"Comments"</button>
      </div>
      <div class="join mr-3 hidden sm:inline-block">
        <A
          href=move || {
              let mut query_params = query.get();
              query_params.insert("list".into(), serde_json::to_string(&ListingType::Subscribed).ok().unwrap());
              query_params.to_query_string()
          }
          class=move || format!(
            "btn join-item{}{}", 
            if ListingType::Subscribed == ssr_list() { " btn-active" } else { "" },
            if let Some(Ok(GetSiteResponse { my_user: Some(_), .. })) = site_signal.get() { "" } else { " btn-disabled" }
          )
        >
          "Subscribed"
        </A>
        <A
          href=move || {
              let mut query_params = query.get();
              query_params.insert("list".into(), serde_json::to_string(&ListingType::Local).ok().unwrap());
              query_params.to_query_string()
          }
          class=move || format!("btn join-item{}", if ListingType::Local == ssr_list() { " btn-active" } else { "" })
        >
          "Local"
        </A>
        <A
          href=move || {
              let mut query_params = query.get();
              query_params.remove("list".into());
              query_params.to_query_string()
          }
          class=move || format!("btn join-item{}", if ListingType::All == ssr_list() { " btn-active" } else { "" })
        >
          "All"
        </A>
      </div>
      <div class="dropdown ml-3 sm:ml-0 hidden sm:inline-block">
        <label tabindex="0" class="btn">
          "Sort type"
        </label>
        <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          <li
            class=move || {
                (if SortType::Active == ssr_sort() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_sort_click(SortType::Active)
          >
            <span>{t!(i18n, active)}</span>
          </li>
          <li
            class=move || {
                (if SortType::Hot == ssr_sort() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_sort_click(SortType::Hot)
          >
            <span>{t!(i18n, hot)}</span>
          </li>
          <li
            class=move || {
                (if SortType::Scaled == ssr_sort() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_sort_click(SortType::Scaled)
          >
            <span>{ "Scaled" }</span>
          </li>
          <li
            class=move || {
                (if SortType::New == ssr_sort() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_sort_click(SortType::New)
          >
            <span>{t!(i18n, new)}</span>
          </li>
        </ul>
      </div>
      <div class="dropdown ml-3 sm:ml-0 inline-block sm:hidden">
        <label tabindex="0" class="btn">
          "Sort type"
        </label>
        <ul tabindex="0" class="menu dropdown-content z-[1] bg-base-100 rounded-box shadow">
          <li
            class=move || {
                (if SortType::Active == csr_sort.get() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_csr_sort_click(SortType::Active)
          >
            <span>{t!(i18n, active)}</span>
          </li>
          <li
            class=move || {
                (if SortType::Hot == csr_sort.get() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_csr_sort_click(SortType::Hot)
          >
            <span>{t!(i18n, hot)}</span>
          </li>
          <li
            class=move || {
                (if SortType::Scaled == csr_sort.get() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_csr_sort_click(SortType::Scaled)
          >
            <span>{ "Scaled" }</span>
          </li>
          <li
            class=move || {
                (if SortType::New == csr_sort.get() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_csr_sort_click(SortType::New)
          >
            <span>{t!(i18n, new)}</span>
          </li>
        </ul>
      </div>
    </div>
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="w-full lg:w-2/3 2xl:w-3/4 3xl:w-4/5 4xl:w-5/6 sm:pr-4">
        <Transition fallback=|| {}>
          {move || {
            posts_resource
                .get()
                .unwrap_or_default()
                .map(|posts| {

                    let next_page = Some((posts.0.0 + ssr_limit(), posts.1.next_page.clone()));

                    view! {
                        <div class=move || format!("hidden sm:block columns-1 2xl:columns-2 3xl:columns-3 4xl:columns-4 gap-0{}", if loading.get() { " opacity-25" } else { "" })>
                          <PostListings posts=posts.1.posts.into() site_signal page_number=posts.0.0.into() />
                        </div>
                        <div class="join hidden sm:block">
                        {
                            let mut st = ssr_prev();
                            let p = st.pop();
                            let mut query_params = query.get();
                            if st.len() > 0 {
                              query_params.insert("prev".into(), serde_json::to_string(&st).unwrap_or("[]".into()));
                            } else {
                              query_params.remove("prev".into());
                            }
                            if p.ne(&Some((0, None))) {
                              query_params.insert("from".into(), serde_json::to_string(&p).unwrap_or("[0,None]".into()));
                            } else {
                              query_params.remove("from".into());
                            }
                            view! {
                                <A
                                  on:click=move |_| { loading.set(true); } 
                                  href=format!("{}", query_params.to_query_string())
                                  class=move || format!("btn join-item{}", if !ssr_prev().is_empty() { "" } else { " btn-disabled" } ) 
                                >
                                  "Prev"
                                </A>
                            }
                        }
                        {
                            let mut st = ssr_prev();
                            st.push(ssr_from());
                            let mut query_params = query.get();
                            query_params.insert("prev".into(), serde_json::to_string(&st).unwrap_or("[]".into()));
                            query_params.insert("from".into(), serde_json::to_string(&next_page).unwrap_or("[0,None]".into()));
                            view! {
                                <A 
                                  on:click=move |_| { loading.set(true); } 
                                  href=format!("{}", query_params.to_query_string())
                                  class=move || format!("btn join-item{}", if next_page.is_some() && !loading.get() { "" } else { " btn-disabled" } ) 
                                >
                                  "Next"
                                </A>
                            }
                        }
                        </div>
                    }
              })
          }}
        </Transition>
        <For each=move || csr_pages.get() key=|h| h.0.clone() let:h>
          <PostListings posts=h.1.posts.into() site_signal page_number=h.0.into() />
        </For>
      </div>
      <div class="lg:w-1/3 hidden lg:block 2xl:w-1/4 3xl:w-1/5 4xl:w-1/6">
        <About/>
        <SiteSummary site_signal/>
        <Trending/>
      </div>
    </main>
  }
}
