use std::collections::BTreeMap;

use lemmy_api_common::{lemmy_db_views::structs::PaginationCursor, post::{GetPosts, GetPostsResponse}};
use leptos::*;
use leptos_router::{use_query_map, use_location, A};

use crate::{ui::components::common::about::About, LemmyClient, PublicFetch};

#[component]
pub fn CommunitiesActivity() -> impl IntoView {

  // let query = use_query_map();

  // let ssr_from = move || {
  //   serde_json::from_str::<(usize, Option<PaginationCursor>)>(
  //     &query
  //       .get()
  //       .get("from")
  //       .cloned()
  //       .unwrap_or("".into()),
  //   )
  //   .unwrap_or((0usize, None))
  // };


  // let ssr_limit = move || {
  //   query
  //     .get()
  //     .get("limit")
  //     .cloned()
  //     .unwrap_or("".into())
  //     .parse::<usize>()
  //     .unwrap_or(10usize)
  // };


  // let next_page_cursor = create_rw_signal::<Option<(usize, Option<PaginationCursor>)>>(None);
  // // let loading = create_rw_signal(false);
  // // let refresh = create_rw_signal(false);

  // // ui_title.set(None);

  // let csr_pages: RwSignal<BTreeMap<(usize, Option<PaginationCursor>), GetPostsResponse>> = RwSignal::new(BTreeMap::new());
  // // let csr_pages: RwSignal<BTreeMap<usize, GetPostsResponse>> = RwSignal::new(BTreeMap::new());
  // let csr_from: RwSignal<Option<(usize, Option<PaginationCursor>)>> = RwSignal::new(None);

  // let vec_pages: RwSignal<Vec<usize>> = RwSignal::new(Vec::new());

  // let posts_resource = create_resource(
  //   move || {
  //     (
  //       // refresh.get(),
  //       // user.get(),
  //       // ssr_list(),
  //       // ssr_sort(),
  //       ssr_from(),
  //       csr_from.get(),
  //       ssr_limit()
  //     )
  //   },
  //   move |(/* _refresh, _user, list_type, sort_type,  */from, csr_from, limit)| async move {

  //     // logging::log!("{:#?}", (_refresh.clone(), _user.clone(), list_type.clone(), sort_type.clone(), from.clone(), csr_from.clone(), limit.clone()));

  //     let f = if let Some(f) = csr_from { f } else { from };
  //     // let f = from;

  //     let form = GetPosts {
  //       type_: None, // Some(list_type),
  //       sort: None, //Some(sort_type),
  //       community_name: None,
  //       community_id: None,
  //       page: None,
  //       limit: None, //Some(i64::try_from(limit).unwrap_or(10)),
  //       saved_only: None,
  //       disliked_only: None,
  //       liked_only: None,
  //       page_cursor: None, //f.1.clone(),
  //       // show_hidden: None,
  //     };

  //     let result = LemmyClient.list_posts(form).await;
  //     // loading.set(false);
  //     // ui_title.set(None);

  //     logging::log!("3");

  //     match result {
  //       Ok(o) => {
  //         // #[cfg(not(feature = "ssr"))]
  //         // {
  //         //   window().scroll_to_with_x_and_y(0.0, 0.0);
  //         // }
  //         Some((f, o))
  //         // Some(o)
  //       },
  //       Err(e) => {
  //         None
  //       }
  //     }
    
  //   },
  // );

  // #[cfg(not(feature = "ssr"))]
  // {

  //   let on_scroll = move |_| {
  //     // if use_location().pathname.get().eq("/") {
  //       let iw = window()
  //         .inner_width()
  //         .ok()
  //         .map(|b| b.as_f64().unwrap_or(0.0))
  //         .unwrap_or(0.0);

  //       if iw < 640f64 {
  //         let h = window()
  //           .inner_height()
  //           .ok()
  //           .map(|b| b.as_f64().unwrap_or(0.0))
  //           .unwrap_or(0.0);
  //         let o = window().page_y_offset().ok().unwrap_or(0.0);
  //         let b = f64::from(document().body().map(|b| b.offset_height()).unwrap_or(1));

  //         let endOfPage = (h + o) >= (b - h);

  //         // logging::log!("{} {} {} {} ", endOfPage, h, o, b); 

  //         if endOfPage {
  //           // csr_from.set(next_page_cursor.get()); 
  //           csr_from.update(|cf| {
  //             // logging::log!("{:#?} {:#?}", *cf, next_page_cursor.get());
  //             *cf = next_page_cursor.get();
  //             // logging::log!("{:#?} {:#?}", *cf, next_page_cursor.get());
  //           }); 
  //         }
  //       }
  //     // }
  //   };

  //   let _scroll_handle = window_event_listener_untyped("scroll", on_scroll);
  // }

  view! {
    <main class="mx-auto">
    <About />
    // <About />
    // <About />
    // <About />
    // <About />
    // <About />
    // <About />


    // <Transition fallback=|| {}>
    // {move || {
    //     posts_resource
    //         .get()
    //         .unwrap_or_default()
    //         .map(|posts| {

    //           logging::log!("1");

    //           let u = posts.0.0;

    //           vec_pages.update(|v| v.push(u));

    //           next_page_cursor.set(Some((posts.0.0 + ssr_limit(), posts.1.next_page.clone())));
    //           csr_pages.update(|h| {
    //             // if csr_from.get().is_none() {
    //             //   h.clear();
    //             // }
    //             // h.insert(posts.0.0, posts.1); 
    //             h.insert(posts.0, posts.1); 
    //           });

    //           view! { 
    //             {
    //               logging::log!("2");
    //             }

    //                   // <For each=move || csr_pages.get() key=|h| h.0.clone() let:h>
    //                 <For each=move || vec_pages.get() key=|v| *v let:v>
    //                   {
    //                     // logging::log!("page{}", h.0.0);
    //                     logging::log!("page{}", v);
    //                   }
    //                   <About />

    //                   // <PostListings posts=h.1.posts.into() site_signal page_number=h.0.0.into() />
    //                 </For>

    //         //   {
    //         //     // let mut st = ssr_prev();
    //         //     // st.push(ssr_from());
    //         //     let mut query_params = query.get();
    //         //     // query_params.insert("prev".into(), serde_json::to_string(&st).unwrap_or("[]".into()));
    //         //     query_params.insert("from".into(), serde_json::to_string(&next_page_cursor.get()).unwrap_or("[0,None]".into()));
    //         //     view! {
    //         //         <A 
    //         //           // on:click=move |_| { loading.set(true); } 
    //         //           href=format!("{}", query_params.to_query_string())
    //         //           // class=move || format!("btn join-item{}", if next_page_cursor.get().is_some() && !loading.get() { "" } else { " btn-disabled" } ) 
    //         //           class=move || format!("btn join-item{}", if next_page_cursor.get().is_some() { "" } else { " btn-disabled" } ) 
    //         //         >
    //         //           "Next"
    //         //         </A>
    //         //     }
    //         // }

    //       }

    //       })
    // }}

    // </Transition>

    </main>
  }
}
