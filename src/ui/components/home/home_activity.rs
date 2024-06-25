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

  let error = expect_context::<RwSignal<Option<LemmyAppError>>>();
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
        // .unwrap_or(serde_json::to_string(&ListingType::All).ok().unwrap()),
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
        // .unwrap_or("\"Active\"".to_string()),
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
    // if let Some(t) = query.get().get("from").cloned() {
    //   if !t.is_empty() {
    //     Some(PaginationCursor(t))
    //   } else {
    //     None
    //   }
    // } else {
    //   None
    // }
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
    // query.get().get("prev").cloned()    
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

  let on_sort_click = move |lt: SortType| {
    move |_me: MouseEvent| {
      let r = serde_json::to_string::<SortType>(&lt);

      match r {
        Ok(o) => {
          let mut query_params = query.get();
          query_params.insert("sort".into(), o);

          let navigate = leptos_router::use_navigate();
          navigate(&query_params.to_query_string(), Default::default());
        }
        Err(e) => {
          error.set(Some(e.into()));
        }
      }
    }
  };

  // let page_cursor = create_rw_signal::<Option<PaginationCursor>>(None);
  // let prev_cursor_stack = create_rw_signal::<Vec<Option<PaginationCursor>>>(vec![]);
  let next_page_cursor = create_rw_signal::<Option<(usize, Option<PaginationCursor>)>>(None);
  // let page_number = create_rw_signal(0usize);

  // let csr_paginator = RwSignal::new(None::<PaginationCursor>);
  // let csr_infinite_scroll_hashmap: RwSignal<BTreeMap<usize, Vec<PostView>>> = RwSignal::new(BTreeMap::new());

  // let page_cursor = expect_context::<RwSignal<PageCursorSetter>>();
  // let prev_cursor_stack = expect_context::<RwSignal<PrevCursorStackSetter>>();
  // let next_page_cursor = expect_context::<RwSignal<NextCursorSetter>>();
  // let page_number = expect_context::<RwSignal<PageNumberSetter>>();

  // let csr_paginator = expect_context::<RwSignal<CsrPageCursorSetter>>();
  // let csr_infinite_scroll_hashmap = expect_context::<RwSignal<CsrHashMapSetter>>();

  // let refresh = create_rw_signal(true);
  let loading = create_rw_signal(false);

  ui_title.set(None);

  let csr_pages: RwSignal<BTreeMap<usize, GetPostsResponse>> = RwSignal::new(BTreeMap::new());
  let csr_from: RwSignal<Option<(usize, Option<PaginationCursor>)>> = RwSignal::new(None);

  let posts = create_resource(
    move || {
      (
        user.get(),
        ssr_list(),
        ssr_sort(),
        ssr_from(),
        csr_from.get(),
        // hashmap.get(), //.keys().collect::<Vec<&(usize, Option<PaginationCursor>)>>(),
        ssr_limit(),
      )
    },
    move |(_user, list_type, sort_type, from, csr_from, limit)| async move {

      // let mut page_refs: BTreeMap<usize, GetPostsResponse> = BTreeMap::new();
      // let hash_ref = hashmap.get();

      // for p in pages.iter() {

      //   if let Some(ps) = hash_ref.get(&p.0) {
      //     page_refs.insert(p.0, ps.clone());
      //   } else {

          // let p = pages.last().unwrap();

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
            page_cursor: if let Some(ref f) = csr_from { f.1.clone() } else { from.1.clone() },
            // page_cursor: p.1.clone(),
            // show_hidden: None,
          };
    
          let result = LemmyClient.list_posts(form).await;
          // loading.set(false);
          // ui_title.set(None);
    
          match result {
            Ok(o) => {
              // hashmap.update(|h| { h.insert(p.0, o.clone()); });
              // let hash_ref = hashmap.get();
              // logging::log!("here");
              // next_page_cursor.set(o.next_page);
              // page_refs.insert(p.0, o);
              // #[cfg(not(feature = "ssr"))]
              // {
              //   window().scroll_to_with_x_and_y(0.0, 0.0);
              // }
              Some((if let Some(f) = csr_from { f } else { from }, o))
            },
            Err(e) => {
              error.set(Some(e));
              // page_refs.insert(p.0, vec![]);
              None
            }
          }
    
      //   }
    
      // }

      // page_refs

    },
  );

  // let ssr_posts = create_resource(
  //   move || {
  //     (
  //       refresh.get(),
  //       user.get(),
  //       list_func(),
  //       sort_func(),
  //       // from_func(),
  //       ssr_limit(),
  //     )
  //   },
  //   move |(_refresh, _user, list_type, sort_type, /* from, */ limit)| async move {
  //     let form = GetPosts {
  //       type_: list_type,
  //       sort: sort_type,
  //       community_name: None,
  //       community_id: None,
  //       page: None,
  //       limit,
  //       saved_only: None,
  //       disliked_only: None,
  //       liked_only: None,
  //       // page_cursor: from,
  //       page_cursor: page_cursor.get(),
  //       // show_hidden: None,
  //     };

  //     let result = LemmyClient.list_posts(form).await;
  //     loading.set(false);

  //     match result {
  //       Ok(o) => {
  //         next_page_cursor.set(o.next_page.clone());
  //         ui_title.set(None);
  //         #[cfg(not(feature = "ssr"))]
  //         {
  //           window().scroll_to_with_x_and_y(0.0, 0.0);
  //         }
  //         Some(o)
  //       },
  //       Err(e) => {
  //         error.set(Some(e));
  //         None
  //       }
  //     }
  //   },
  // );


  #[cfg(not(feature = "ssr"))]
  {
    // let csr_page_number = create_rw_signal(10usize);

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
          // query_params.insert("limit".into(), "10".to_string());
          // Some("10".to_string())
          query_params.remove("limit");
          None
        };

        if iw >= 640f64 {
          // csr_paginator.set(None);
          // csr_page_number.set(10usize);
          // csr_infinite_scroll_hashmap.set(BTreeMap::new());
          // prev_cursor_stack.set(vec![]);
          // page_cursor.set(None);
          // page_number.set(0usize);
        }

        if prev_limit.ne(&new_limit) {
          let navigate = leptos_router::use_navigate();
          if iw >= 640f64 {
            // let navigate = leptos_router::use_navigate();
            navigate(
              &format!("{}", query_params.to_query_string()),
              Default::default(),
            );
          } else {
            navigate("/",
              // &format!("{}", query_params.to_query_string()),
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
            csr_from.set(next_page_cursor.get()); 

            // if csr_infinite_scroll_hashmap.get().get(&csr_page_number.get()).is_none() {
            //   csr_infinite_scroll_hashmap.update(|h| { h.insert(csr_page_number.get(), vec![]); });

            //   create_local_resource(
            //     move || (),
            //     move |()| async move {
            //       let form = GetPosts {
            //         type_: list_func(),
            //         sort: sort_func(),
            //         community_name: None,
            //         community_id: None,
            //         page: None,
            //         limit: None,
            //         saved_only: None,
            //         disliked_only: None,
            //         liked_only: None,
            //         page_cursor: csr_paginator.get(),
            //         // show_hidden: None,
            //       };

            //       let result = LemmyClient.list_posts(form).await;

            //       match result {
            //         Ok(o) => {
            //           csr_paginator.set(o.next_page);
            //           csr_infinite_scroll_hashmap.update(|h| { h.insert(csr_page_number.get(), o.posts.clone()); });
            //           csr_page_number.update(|p| *p = *p + ssr_limit().unwrap_or(10i64) as usize);
            //         }
            //         Err(e) => {
            //           csr_infinite_scroll_hashmap.update(|h| { h.remove(&csr_page_number.get()); });
            //           error.set(Some(e));
            //         }
            //       }
            //     },
            //   );
            // }
          }
        }
      }
    };

    let _scroll_handle = window_event_listener_untyped("scroll", on_scroll);
  }

  // let page_cursors_writable = RwSignal::new(false);
  // #[cfg(not(feature = "ssr"))]
  // {
  //   page_cursors_writable.set(true);
  // }

  view! {
    <div class="block">
      <div class="join mr-3 hidden sm:inline-block">
        <button class="btn join-item btn-active">"Posts"</button>
        <button class="btn join-item btn-disabled">"Comments"</button>
      </div>
      <div class="join mr-3 hidden sm:inline-block">
        {move || {
            let mut query_params = query.get();
            query_params.insert("list".into(), "\"Subscribed\"".into());
            view! {
              <A
                href=move || query_params.to_query_string()
                class=move || {
                    format!(
                        "btn join-item{}{}",
                        if ListingType::Subscribed == ssr_list() { " btn-active" } else { "" },
                        { if let Some(Ok(GetSiteResponse { my_user: Some(_), .. })) = site_signal.get() { "" } else { " btn-disabled" } },
                    )
                }
              >

                "Subscribed"
              </A>
            }
        }}
        <A
          href=move || {
              let mut query_params = query.get();
              query_params.insert("list".into(), serde_json::to_string(&ListingType::Local).ok().unwrap());
              query_params.to_query_string()
          }

          class=move || {
              format!(
                  "btn join-item {}",
                  if ListingType::Local == ssr_list() { "btn-active" } else { "" },
              )
          }
        >

          "Local"
        </A>
        <A
          href=move || {
              let mut query_params = query.get();
              query_params.insert("list".into(), serde_json::to_string(&ListingType::All).ok().unwrap());
              query_params.to_query_string()
          }

          class=move || {
              format!(
                  "btn join-item {}",
                  if ListingType::All == ssr_list() { "btn-active" } else { "" },
              )
          }
        >

          "All"
        </A>
      </div>
      <div class="dropdown hidden sm:inline-block">
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
                (if SortType::New == ssr_sort() { "btn-active" } else { "" }).to_string()
            }
            on:click=on_sort_click(SortType::New)
          >
            <span>{t!(i18n, new)}</span>
          </li>
        </ul>
      </div>
    </div>
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="w-full lg:w-2/3 2xl:w-3/4 3xl:w-4/5 4xl:w-5/6">

      <Transition fallback=|| {}>
        {move || {
            posts
                .get()
                .unwrap_or_default()
                .map(|post| {
                    // if csr_infinite_scroll_hashmap.get().keys().len() == 0 {
                    //     csr_paginator.set(p.next_page.clone());
                    // }

                    // if next_page_cursor.get().is_none() {
                    //     next_page_cursor.set(p.next_page.clone());
                    // }
                    // if let Some(p) = post.clone() {
                      next_page_cursor.set(Some((post.0.0 + ssr_limit(), post.1.next_page.clone())));
                      csr_pages.update(|h| { h.insert(post.0.0, post.1); });
                    // }

                    // let p_copy = post_map.clone();
                    view! {
                        <div class="columns-1 2xl:columns-2 3xl:columns-3 4xl:columns-4 gap-3">
                          // <PostListings posts=post.posts.into() site_signal page_number=0.into() />
                          <For each=move || csr_pages.get().into_iter() key=|h| h.0 let:h>
                            <PostListings posts=h.1.posts.into() site_signal page_number=h.0.into() />
                          </For>
                        </div>
  
                        <div class="join hidden sm:block">

                        {
                          // if let Some(s) = ssr_prev() {
                          //     if !s.is_empty() {
                                  // let mut st = ssr_prev().unwrap_or("".into()).split(',').collect::<Vec<_>>();
                                  // let p = st.pop().unwrap_or("");
                                  let mut st = ssr_prev();
                                  let p = st.pop();
                                  let mut query_params = query.get();
                                  query_params.insert("prev".into(), serde_json::to_string(&st).unwrap_or("[]".into()));
                                  query_params.insert("from".into(), serde_json::to_string(&p).unwrap_or("[0,None]".into()));
                                  view! {
                                    <span>
                                      <A
                                        href=format!("{}", query_params.to_query_string())
                                        class=move || format!("btn join-item{}", if !ssr_prev().is_empty() && !loading.get() { "" } else { " btn-disabled" } ) 
                                        // class="btn"
                                        // on:click=move |e: MouseEvent| { e.prevent_default(); csr_from.update(|ps| { ps.pop(); }); }
                                      >
                                        "Prev"
                                      </A>
                                    </span>
                                  }
                          //     } else {
                          //         view! { <span></span> }
                          //     }
                          // } else {
                          //     view! { <span></span> }
                          // }
                        }
                        {
                          // if let Some(n) = post.unwrap().next_page.clone() {
                              // let s = ssr_prev().unwrap_or_default();
                              // let mut st = s.split(',').collect::<Vec<_>>();
                              // let f = if let (_, Some(PaginationCursor(g))) = ssr_from() {
                              //     g
                              // } else {
                              //     "".to_string()
                              // };
                              // st.push(&f);
                              // let mut query_params = query.get();
                              // query_params.insert("prev".into(), st.join(",").to_string());
                              // query_params.insert("from".into(), n.0.clone());
                              let mut st = ssr_prev();
                              st.push(ssr_from());
                              // let p = post.unwrap().next_page.clone();
                              let mut query_params = query.get();
                              query_params.insert("prev".into(), serde_json::to_string(&st).unwrap_or("[]".into()));
                              query_params.insert("from".into(), serde_json::to_string(&next_page_cursor.get()).unwrap_or("[0,None]".into()));
                              view! {
                                <span>
                                  <A 
                                    href=format!("{}", query_params.to_query_string())
                                    class=move || format!("btn join-item{}", if next_page_cursor.get().is_some() && !loading.get() { "" } else { " btn-disabled" } ) 
                                  // on:click=move |e: MouseEvent| { e.prevent_default(); csr_from.update(|ps| { ps.push((ps.last().unwrap().0 + 10, Some(n.clone()))); }); }
                                  >
                                    "Next"
                                  </A>
                                </span>
                              }
                        //   } else {
                        //       view! { <span></span> }
                        //   }
                        }

                        </div>
                }
              })
        }}

        </Transition>

      // <Transition fallback=|| {}>
      //   {move || {
      //       ssr_posts
      //           .get()
      //           .unwrap_or(None)
      //           .map(|p| {
      //               if csr_infinite_scroll_hashmap.get().keys().len() == 0 {
      //                   csr_paginator.set(p.next_page.clone());
      //               }

      //               if next_page_cursor.get().is_none() {
      //                   next_page_cursor.set(p.next_page.clone());
      //               }
      //               view! {
      //                   <div class="columns-1 2xl:columns-2 3xl:columns-3 4xl:columns-4 gap-3">
      //                     <PostListings posts=p.posts.into() site_signal page_number />
      //                     <For each=move || csr_infinite_scroll_hashmap.get().into_iter() key=|h| h.0 let:h>
      //                       <PostListings posts=h.1.into() site_signal page_number=h.0.into() />
      //                     </For>
      //                   </div>
  
      //                   <div class=move || format!("join hidden{}", if page_cursors_writable.get() { " sm:block" } else { "" })>

      //                     <button
      //                       class=move || format!("btn join-item{}", if prev_cursor_stack.get().len() > 0 { "" } else { " btn-disabled" } ) 
      //                       on:click=move |_| {
      //                           // PageCursors are not writable in v 0.19.3
      //                           let mut p = prev_cursor_stack.get();
      //                           let s = p.pop().unwrap_or(None);
      //                           prev_cursor_stack.set(p);
      //                           page_cursor.set(s);
      //                           refresh.set(!refresh.get());
      //                           page_number.update(|p| *p = (*p) - ssr_limit().unwrap_or(10i64) as usize);
      //                       }
      //                     >
      //                       "Prev"
      //                     </button>
      //                     <button
      //                       class=move || format!("btn join-item{}", if next_page_cursor.get().is_some() && !loading.get() { "" } else { " btn-disabled" } ) 
      //                       on:click=move |_| {
      //                           // PageCursors are not writable in v 0.19.3
      //                           let mut p = prev_cursor_stack.get();
      //                           p.push(page_cursor.get());
      //                           prev_cursor_stack.set(p);
      //                           page_cursor.set(next_page_cursor.get());
      //                           loading.set(true);
      //                           refresh.set(!refresh.get());
      //                           page_number.update(|p| *p = (*p) + ssr_limit().unwrap_or(10i64) as usize);
      //                       }
      //                     >
      //                       "Next"
      //                     </button>

      //                   </div>
      //           }
      //         })
      //   }}

      //   </Transition>
      </div>
      <div class="lg:w-1/3 hidden lg:block 2xl:w-1/4 3xl:w-1/5 4xl:w-1/6">
        <About/>
        <SiteSummary site_signal/>
        <Trending/>
      </div>
    </main>
  }
}
