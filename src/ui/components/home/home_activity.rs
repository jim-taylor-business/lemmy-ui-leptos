use std::{collections::BTreeMap, usize, vec};

use crate::{
  errors::{message_from_error, LemmyAppError, LemmyAppErrorType},
  i18n::*,
  lemmy_client::*,
  ui::components::{
    common::about::About,
    home::{site_summary::SiteSummary, trending::Trending},
    post::post_listings::PostListings,
  },
  OnlineSetter, ResourceStatus, TitleSetter,
};
use html::Div;
use lemmy_api_common::{
  lemmy_db_schema::{ListingType, SortType},
  lemmy_db_views::structs::{PaginationCursor, PostView},
  post::{self, GetPosts, GetPostsResponse},
  site::GetSiteResponse,
};
use leptos::*;
use leptos_router::*;
// use strum_macros::Display;
use web_sys::MouseEvent;

use leptos::html::*;
use leptos_use::*;

#[component]
pub fn HomeActivity(
  site_signal: RwSignal<Option<Result<GetSiteResponse, LemmyAppError>>>,
) -> impl IntoView {
  let i18n = use_i18n();

  let error = expect_context::<RwSignal<Vec<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>>();
  let user = expect_context::<RwSignal<Option<bool>>>();
  let ui_title = expect_context::<RwSignal<Option<TitleSetter>>>();
  let online = expect_context::<RwSignal<OnlineSetter>>();

  let param = use_params_map();
  let community_name = move || param.get().get("name").cloned();

  let query = use_query_map();

  let ssr_list = move || {
    serde_json::from_str::<ListingType>(&query.get().get("list").cloned().unwrap_or("".into()))
      .unwrap_or(ListingType::All)
  };

  let ssr_sort = move || {
    serde_json::from_str::<SortType>(&query.get().get("sort").cloned().unwrap_or("".into()))
      .unwrap_or(SortType::Active)
  };

  let ssr_from = move || {
    serde_json::from_str::<(usize, Option<PaginationCursor>)>(
      &query.get().get("from").cloned().unwrap_or("".into()),
    )
    .unwrap_or((0usize, None))
  };

  let ssr_prev = move || {
    serde_json::from_str::<Vec<(usize, Option<PaginationCursor>)>>(
      &query.get().get("prev").cloned().unwrap_or("".into()),
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
          error.update(|es| es.push(Some((e.into(), None))));
          // error.set(Some((e.into(), None)));
        }
      }

      if SortType::Active == s {
        query_params.remove("sort".into());
      }

      let navigate = leptos_router::use_navigate();
      navigate(
        &format!(
          "{}{}",
          use_location().pathname.get(),
          query_params.to_query_string()
        ),
        Default::default(),
      );
    }
  };

  let loading = RwSignal::new(false);
  let refresh = RwSignal::new(false);

  ui_title.set(None);

  let posts_resource = Resource::new(
    move || {
      (
        refresh.get(),
        user.get(),
        ssr_list(),
        ssr_sort(),
        ssr_from(),
        ssr_limit(),
        community_name(),
      )
    },
    move |(_refresh, _user, list_type, sort_type, from, limit, name)| async move {
      let form = GetPosts {
        type_: Some(list_type),
        sort: Some(sort_type),
        community_name: name,
        community_id: None,
        page: None,
        limit: Some(i64::try_from(limit).unwrap_or(10)),
        saved_only: None,
        disliked_only: None,
        liked_only: None,
        page_cursor: from.1.clone(), // show_hidden: None,
      };

      if online.get().0 {
        logging::log!("load!");
        let result = LemmyClient.list_posts(form.clone()).await;
        loading.set(false);
        ui_title.set(None);

        match result {
          Ok(o) => {
            #[cfg(not(feature = "ssr"))]
            if let Ok(Some(s)) = window().local_storage() {
              if let Ok(Some(_)) = s.get_item(&serde_json::to_string(&form).ok().unwrap()) {
                logging::log!("cached!");
              }
              s.set_item(
                &serde_json::to_string(&form).ok().unwrap(),
                &serde_json::to_string(&o).ok().unwrap(),
              );
            }
            Ok((from, o))
          }
          Err(e) => {
            error.update(|es| es.push(Some((e.clone(), None))));
            Err((e, Some(refresh)))
          }
        }
      } else {
        #[cfg(not(feature = "ssr"))]
        logging::log!("test!");
        if let Ok(Some(s)) = window().local_storage() {
          if let Ok(Some(c)) = s.get_item(&serde_json::to_string(&form).ok().unwrap()) {
            logging::log!("cached offline!");
            if let Ok(o) = serde_json::from_str::<GetPostsResponse>(&c) {
              loading.set(false);
              return Ok((from, o));
            }
          }
        }

        logging::log!("here!");

        loading.set(false);

        let e = LemmyAppError {
          error_type: LemmyAppErrorType::OfflineError,
          content: String::from(""),
        };
        error.update(|es| es.push(Some((e.clone(), None))));
        Err((e, Some(refresh)))
      }
    },
  );

  // let csr_resources: RwSignal<
  //   BTreeMap<(usize, ResourceStatus), (Option<PaginationCursor>, Option<GetPostsResponse>)>,
  // > = RwSignal::new(BTreeMap::new());

  // let csr_pages = expect_context::<RwSignal<BTreeMap<usize, GetPostsResponse>>>();
  let csr_resources = expect_context::<
    RwSignal<
      BTreeMap<(usize, ResourceStatus), (Option<PaginationCursor>, Option<GetPostsResponse>)>,
    >,
  >();
  let csr_sort = expect_context::<RwSignal<SortType>>();
  let csr_next_page_cursor = expect_context::<RwSignal<(usize, Option<PaginationCursor>)>>();

  let on_csr_sort_click = move |s: SortType| {
    move |_e: MouseEvent| {
      csr_next_page_cursor.set((0, None));
      csr_sort.set(s);
      // csr_pages.set(BTreeMap::new());
    }
  };

  let resize_element = create_node_ref::<Main>();
  let scroll_element = create_node_ref::<Div>();

  #[cfg(not(feature = "ssr"))]
  {
    use_resize_observer(resize_element, move |entries, _| {
      let rect = entries[0].content_rect();
      // logging::log!("width: {:.0} height: {:.0}", rect.width(), rect.height());
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
        // csr_pages.set(BTreeMap::new());
        csr_resources.set(BTreeMap::new());
        csr_next_page_cursor.set((0, None));
      }

      if prev_limit.ne(&new_limit) {
        let navigate = leptos_router::use_navigate();
        if iw >= 640f64 {
          navigate(
            &format!(
              "{}{}",
              use_location().pathname.get(),
              query_params.to_query_string()
            ),
            Default::default(),
          );
        } else {
          navigate("/", Default::default());
        }
      }
    });

    let UseIntersectionObserverReturn {
      is_active,
      // pause,
      // resume,
      ..
    } = use_intersection_observer_with_options(
      scroll_element,
      move |entries, _| {
        // logging::log!("SCROLL");

        let iw = window()
          .inner_width()
          .ok()
          .map(|b| b.as_f64().unwrap_or(0.0))
          .unwrap_or(0.0);

        // logging::log!("{}", iw);
        if iw < 640f64 {
          if csr_resources
            .get()
            .get(&(csr_next_page_cursor.get().0, ResourceStatus::Loading))
            .is_none()
            && csr_resources
              .get()
              .get(&(csr_next_page_cursor.get().0, ResourceStatus::Ok))
              .is_none()
            && csr_resources
              .get()
              .get(&(csr_next_page_cursor.get().0, ResourceStatus::Err))
              .is_none()
          {
            csr_resources.update(|h| {
              h.insert(
                (csr_next_page_cursor.get().0, ResourceStatus::Loading),
                (csr_next_page_cursor.get().1, None),
              );
            });

            let csr_resource = create_local_resource(
              move || (),
              move |()| async move {
                let from = csr_next_page_cursor.get();

                let form = GetPosts {
                  // type_: Some(list_type),
                  type_: Some(ListingType::All),
                  sort: Some(csr_sort.get()),
                  community_name: None,
                  community_id: None,
                  page: None,
                  // limit: Some(i64::try_from(limit).unwrap_or(10)),
                  limit: Some(10),
                  saved_only: None,
                  disliked_only: None,
                  liked_only: None,
                  page_cursor: from.1.clone(),
                  // show_hidden: None,
                };

                // logging::log!("GET {}", from.0);
                let result = LemmyClient.list_posts(form).await;

                match result {
                  Ok(o) => {
                    csr_next_page_cursor.set((from.0 + ssr_limit(), o.next_page.clone()));

                    csr_resources.update(move |h| {
                      // let f = from.clone();
                      h.remove(&(from.0, ResourceStatus::Loading));
                      h.insert(
                        (from.0, ResourceStatus::Ok),
                        (from.1.clone(), Some(o.clone())),
                      );
                    });

                    // csr_pages.update(|h| {
                    //   h.insert(from.0, o);
                    // });
                    Some(())
                  }
                  Err(e) => {
                    csr_resources.update(move |h| {
                      // let f = from.clone();
                      h.remove(&(from.0, ResourceStatus::Loading));
                      h.insert((from.0, ResourceStatus::Err), (from.1, None));
                    });
                    error.update(|es| es.push(Some((e, Some(refresh)))));
                    // error.set(Some((e, Some(refresh))));
                    None
                  }
                }
              },
            );

            // csr_resource.dispose()

            // csr_resources.update(|h| {
            //   h.remove((csr_next_page_cursor.get().0, false), csr_resource);
            //   h.insert((csr_next_page_cursor.get().0, true), csr_resource);
            // });

            // } else {
          }
        }
      },
      UseIntersectionObserverOptions::default(), //.root(Some(root)),
    );
  }

  // let on_retry_click = move |c: (usize, Option<PaginationCursor>)| {
  let on_retry_click = move |i: (usize, ResourceStatus)| {
    // let value = c.clone();

    move |_e: MouseEvent| {
      let csr_resource = create_local_resource(
        move || (),
        move |()| //{
        // let value = c.clone();
        async move {
          // let from = value;
          let from = csr_resources.get().get(&i).unwrap().0.clone();

          let form = GetPosts {
            // type_: Some(list_type),
            type_: Some(ListingType::All),
            sort: Some(csr_sort.get()),
            community_name: None,
            community_id: None,
            page: None,
            // limit: Some(i64::try_from(limit).unwrap_or(10)),
            limit: Some(10),
            saved_only: None,
            disliked_only: None,
            liked_only: None,
            page_cursor: from.clone(),
            // show_hidden: None,
          };

          let from_clone = from.clone();

          csr_resources.update(move |h| {
            // let f = from.clone();
            h.remove(&(i.0, ResourceStatus::Err));
            h.insert((i.0, ResourceStatus::Loading), (from_clone, None));
          });

          // logging::log!("GET {}", from.0);
          let result = LemmyClient.list_posts(form).await;

          match result {
            Ok(o) => {
              csr_next_page_cursor.set((i.0 + ssr_limit(), o.next_page.clone()));

              csr_resources.update(move |h| {
                // let f = from.clone();
                h.remove(&(i.0, ResourceStatus::Loading));
                h.insert((i.0, ResourceStatus::Ok), (from, Some(o.clone())));
              });

              // csr_pages.update(|h| {
              //   h.insert(from.0, o);
              // });
              Some(())
            }
            Err(e) => {
              csr_resources.update(move |h| {
                // let f = from.clone();
                h.remove(&(i.0, ResourceStatus::Loading));
                h.insert((i.0, ResourceStatus::Err), (from, None));
              });
              error.update(|es| es.push(Some((e, None))));
              // error.set(Some((e, Some(refresh))));
              None
            }
          }

        //}
        },
      );
    }
  };

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
                format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
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
                format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
            }
            class=move || format!("btn join-item{}", if ListingType::Local == ssr_list() { " btn-active" } else { "" })
          >
            "Local"
          </A>
          <A
            href=move || {
                let mut query_params = query.get();
                query_params.remove("list".into());
                format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
            }
            class=move || format!("btn join-item{}", if ListingType::All == ssr_list() { " btn-active" } else { "" })
          >
            "All"
          </A>
        </div>
        <div class="dropdown ml-3 sm:ml-0 hidden sm:inline-block">
          <label tabindex="0" class="btn">
            "Sort"
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
            "Sort"
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
      <main node_ref=resize_element class="w-full flex flex-col sm:flex-row flex-grow">
        <div class="relative w-full lg:w-2/3 2xl:w-3/4 3xl:w-4/5 4xl:w-5/6 sm:pr-4">

          // <div node_ref=scroll_element class="absolute bottom-0 h-96 bg-orange-600 block sm:hidden">"TEST TEST"</div>

                    // <Transition fallback=|| {}>
                    //         // <div class="badge badge-error badge-xs"></div>
                    //   {move || {
                    //       test
                    //           .get()
                    //           // .unwrap_or_default()
                    //           .map(|u| {
                    //               view! {
                    //                 <div class="badge badge-error badge-xs"></div>
                    //               }
                    //           });
                    //   }}

                    // </Transition>
                    // <Transition fallback=|| {}>
                    // //         <div class="badge badge-error badge-xs"></div>
                    // //   {move || {
                    // //       ssr_unread
                    // //           .get()
                    // //           .map(|u| {
                    // //               // view! {
                    // //               //   <div class="badge badge-error badge-xs"></div>
                    // //               // }

                    // //               if let Ok(c) = u {
                    // //                 // logging::log!("lalal {:#?}", c);
                    // //                 view! {
                    // //                   <div class="badge badge-error badge-xs"> { format!("lalal {:#?}", c) } </div>
                    // //                 }
                    // //               //   // view! {
                    // //               //   //   <div class="badge badge-error badge-xs"> { c.replies + c.mentions + c.private_messages } </div>
                    // //               //   // }
                    // //               } else {
                    // //                 view! {
                    // //                   <div class="hidden"> "SO HIDDEN RIGHT NOW" </div>
                    // //                 }
                    // //               }
                    // //           });
                    // //   }}

                    // {move || {
                    //   if posts_resource.loading().get() {
                    //     view! { <div class="badge badge-warning  badge-xs"> { "Loading" } </div> }
                    //   } else {
                    //     view! { <div class="badge badge-success  badge-xs"> { "Finished" } </div> }
                    //   }
                    // }}
                    // </Transition>
  //

          <Transition fallback=|| { }>

                      // {move || {
                      //   match posts_resource.get() {
                      //     Some(_) => {
                      //       view! { <div class="badge badge-success  badge-xs"> { "Result" } </div> }
                      //     }
                      //     None => {
                      //       view! { <div class="badge badge-warning  badge-xs"> { "Loading" } </div> }
                      //     }
                      //   }
                      // }}
            {move || {
              match posts_resource.get() {
                Some(Err(err)) => {
                  // error.map(|err| {
                    view! {
                    <div class="px-8 py-4">
                      <div class="alert alert-error flex justify-between">
                        <span>{message_from_error(&err.0)} " - " {err.0.content}</span>
                        <div>
                          <Show when=move || { if let Some(r) = err.1 { true } else { false } } /* let:r */ fallback=|| {}>
                            <button on:click=move |_| { if let Some(r) = err.1 { r.set(!r.get()); } else { } } class="btn btn-sm"> "Retry" </button>
                          </Show>
                        </div>
                      </div>
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
                                href=format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
                                class=move || format!("btn join-item{}", if !ssr_prev().is_empty() { "" } else { " btn-disabled" } )
                              >
                                "Prev"
                              </A>
                          }
                      }
                      {
                          // let mut st = ssr_prev();
                          // st.push(ssr_from());
                          let mut query_params = query.get();
                          // query_params.insert("prev".into(), serde_json::to_string(&st).unwrap_or("[]".into()));
                          // query_params.insert("from".into(), serde_json::to_string(&next_page).unwrap_or("[0,None]".into()));
                          view! {
                              <A
                                // on:click=move |_| { loading.set(true); }
                                href=format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
                                // class=move || format!("btn join-item{}{}", if next_page.is_some() && !loading.get() { "" } else { " btn-disabled" }, if loading.get() { " btn-disabled" } else { "" } )
                                class="btn join-item btn-disabled"
                              >
                                "Next"
                              </A>
                          }
                      }
                      </div>
                    }
                  // })

                  // view! { <div class="badge badge-error  badge-xs"> { "Error" } </div><div class="badge badge-error  badge-xs"> { "Error" } </div> }
                }
                Some(Ok(posts)) => {

                  // view! { <div class="badge badge-success  badge-xs"> { "Result" } </div> }

              // posts_resource
              //     .get()
              //     .unwrap_or_default()
              //     .map(|posts| {

              // pr.map(|posts| {

                      let next_page = Some((posts.0.0 + ssr_limit(), posts.1.next_page.clone()));

                      view! {
                        {
                          if loading.get() {
                            view! {
                              <div class="px-8 py-4 animate-[popout_0.5s_step-end_2]">
                                  <div class="alert">
                                    <span> "Loading" </span>
                                  </div>
                              </div>
                            }
                          } else {
                            view! { <div class="hidden"></div> }
                          }
                        }
                        // <div class="hidden"></div>
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
                                    href=format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
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
                                    href=format!("{}{}", use_location().pathname.get(), query_params.to_query_string())
                                    class=move || format!("btn join-item{}{}", if next_page.is_some() && !loading.get() { "" } else { " btn-disabled" }, if loading.get() { " btn-disabled" } else { "" } )
                                  >
                                    "Next"
                                  </A>
                              }
                          }
                          </div>
                      }

                  // })
                }
                None => {
                  view! {
                    <div class="px-8 py-4 animate-[popout_0.5s_step-end_2]">
                        <div class="alert">
                          <span> "Loading" </span>
                        </div>
                    </div>
                    <div class="hidden"></div>
                  }
                }
              }

            }}
          </Transition>


          // <For each=move || csr_pages.get() key=|h| h.0.clone() let:h>
          //   // {
          //   //   logging::log!("page");
          //   // }
          //   <PostListings posts=h.1.posts.into() site_signal page_number=h.0.into() />
          // </For>

          <For each=move || csr_resources.get() key=|r| r.0.clone() let:r>

            // <div class="text-center text-lg alert alert-info">
            // </div>

            {
              logging::log!("res {:#?}", r.0);
              let r_copy = r.clone();
            // {move || {
            //     r.1
            //       .get()
            //       .unwrap_or_default()
            //       .map(|o| {

            //         view! {
            //           <div> "uluullul" {o} </div>
            //         }


            //       });
            // }}
              view! {


            <Show when=move || r.0.1 == ResourceStatus::Ok
              fallback=move || {
                match r_copy.0.1 {
                  ResourceStatus::Err => view! {
                    <div class="px-8 py-4">
                      <div class="alert alert-error flex justify-between">
                        <span class="text-lg"> "Error" </span>
                        // <div>
                          <span /* on:click=|_| {} */ on:click=on_retry_click(r_copy.0) /* on:click=on_retry_click((r_copy.0.0.clone(), r_copy.1.0.clone()))  */class="btn btn-sm"> "Retry" </span>
                        // </div>
                      </div>
                    </div>
                  },
                  _ => view! {
                    <div class="px-8 py-4 animate-[popout_0.5s_step-end_2]">
                      <div class="alert">
                        <span> "Loading..." </span>
                      </div>
                    </div>
                  },
                }
              }
            >
              <PostListings posts=r.1.clone().1.unwrap().posts.into() site_signal page_number=r.0.0.into() />
            </Show>
              }


            }
          </For>

          <div node_ref=scroll_element class="sm:hidden block"></div>

        </div>
        <div class="lg:w-1/3 hidden lg:block 2xl:w-1/4 3xl:w-1/5 4xl:w-1/6">
          <About/>
          <SiteSummary site_signal/>
          <Trending/>
        </div>
      </main>
    }
}
