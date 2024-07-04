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
  let error = expect_context::<RwSignal<Option<(LemmyAppError, Option<RwSignal<bool>>)>>>();
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
        error.set(Some((e, None)));
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
        error.set(Some((e, None)));
        None
      }
    }

    } else {
      None
    }
  });

  // fn test_basic_markdown() {
  //   let tests: Vec<_> = vec![
  //     (
  //       "headings",
  //       "# h1\n## h2\n### h3\n#### h4\n##### h5\n###### h6",
  //       "<h1>h1</h1>\n<h2>h2</h2>\n<h3>h3</h3>\n<h4>h4</h4>\n<h5>h5</h5>\n<h6>h6</h6>\n"
  //     ),
  //     (
  //       "line breaks",
  //       "First\rSecond",
  //       "<p>First\nSecond</p>\n"
  //     ),
  //     (
  //       "emphasis",
  //       "__bold__ **bold** *italic* ***bold+italic***",
  //       "<p><strong>bold</strong> <strong>bold</strong> <em>italic</em> <em><strong>bold+italic</strong></em></p>\n"
  //     ),
  //     (
  //       "blockquotes",
  //       "> #### Hello\n > \n > - Hola\n > - 안영 \n>> Goodbye\n",
  //       "<blockquote>\n<h4>Hello</h4>\n<ul>\n<li>Hola</li>\n<li>안영</li>\n</ul>\n<blockquote>\n<p>Goodbye</p>\n</blockquote>\n</blockquote>\n"
  //     ),
  //     (
  //       "lists (ordered, unordered)",
  //       "1. pen\n2. apple\n3. apple pen\n- pen\n- pineapple\n- pineapple pen",
  //       "<ol>\n<li>pen</li>\n<li>apple</li>\n<li>apple pen</li>\n</ol>\n<ul>\n<li>pen</li>\n<li>pineapple</li>\n<li>pineapple pen</li>\n</ul>\n"
  //     ),
  //     (
  //       "code and code blocks",
  //       "this is my amazing `code snippet` and my amazing ```code block```",
  //       "<p>this is my amazing <code>code snippet</code> and my amazing <code>code block</code></p>\n"
  //     ),
  //     (
  //       "links",
  //       "[Lemmy](https://join-lemmy.org/ \"Join Lemmy!\")",
  //       "<p><a href=\"https://join-lemmy.org/\" title=\"Join Lemmy!\">Lemmy</a></p>\n"
  //     ),
  //     (
  //       "images",
  //       "![My linked image](https://image.com \"image alt text\")",
  //       "<p><img src=\"https://image.com\" alt=\"My linked image\" title=\"image alt text\" /></p>\n"
  //     ),
  //     // Ensure any custom plugins are added to 'MARKDOWN_PARSER' implementation.
  //     (
  //       "basic spoiler",
  //       "::: spoiler click to see more\nhow spicy!\n:::\n",
  //       "<details><summary>click to see more</summary><p>how spicy!\n</p></details>\n"
  //     ),
  //     (
  //       "lulu",
  //       "<input type=\"submit\" /><iframe src=\"http://www.google.com\" />",
  //       "not this"
  //     ),
  //     (
  //       "bubu",
  //       "<buu><fake></fake>",
  //       "not this"
  //     ),
  //     (
  //       "pupu",
  //       "<arch joke><etc></fake>",
  //       "not this"
  //     ),
  //     (
  //         "escape html special chars",
  //         "<script>alert('xss');</script> hello &\"",
  //         "<p>&lt;script&gt;alert(‘xss’);&lt;/script&gt; hello &amp;&quot;</p>\n"
  //     )
  //   ];

  //   tests.iter().for_each(|&(msg, input, expected)| {
  //     let parser = pulldown_cmark::Parser::new(input);
  //     let custom = parser.map(|event| match event {
  //       pulldown_cmark::Event::Html(text) => {
  //         let er = format!("<p>{}</p>",  html_escape::encode_safe(&text).to_string());
  //         pulldown_cmark::Event::Html(er.into())
  //       }
  //       pulldown_cmark::Event::InlineHtml(text) => {
  //         let er = html_escape::encode_safe(&text).to_string();
  //         pulldown_cmark::Event::InlineHtml(er.into())
  //       }
  //       _ => event
  //     });
  //     let mut html = String::new();
  //     pulldown_cmark::html::push_html(&mut html, custom);

  //     let result = html;


  //     // let result = markdown_to_html(input);

  //     if result.ne(&expected) {
  //       logging::log!(
  //         "Testing {}, with original input '{}'\n{}\n{}",
  //         msg, input, result, expected
  //       );
  //     }
  //   });
  // }

  // test_basic_markdown();

  #[cfg(not(feature = "ssr"))]
  {
    let on_resize = move |_| { };
    window_event_listener_untyped("resize", on_resize);
    let on_scroll = move |_| { };
    window_event_listener_untyped("scroll", on_scroll);
  }

  view! {
    <main role="main" class="w-full flex flex-col sm:flex-row flex-grow">
      <div class="flex flex-col">
        <div>
          <Transition fallback=|| {}>
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
                          if let Some(ref content) = text {
                            // let parser = pulldown_cmark::Parser::new(refer);
                            // let mut html = String::new();
                            // pulldown_cmark::html::push_html(&mut html, parser);
                            // let safe_html = ammonia::clean(&*html);

                            let parser = pulldown_cmark::Parser::new(content);
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

                            view! {
                              <div class="pl-4 pr-4">
                                <div class="py-2">
                                  <div class="prose max-w-none"
                                    inner_html=safe_html
                                  />
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
          <Transition fallback=|| {}>
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
