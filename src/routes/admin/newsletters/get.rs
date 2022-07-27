use actix_web::{cookie::Cookie, http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn submit_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let mut response = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
              <head>
                <meta charset="UTF-8" />
                <meta http-equiv="content-type" content="text/html; charset=utf-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Publish Newsletter</title>
              </head>
              <body>
                {error_html}
                <form action="/admin/newsletters" method="post">
                  <label>
                    Title
                    <input type="text" placeholder="Enter Title" name="title" />
                  </label>
                  <br/>
                  <label>
                    Text Content
                    <textarea
                        placeholder="Enter Text Content"
                        name="text_content"
                        rows="5"
                    ></textarea>
                  </label>
                  <br/>
                  <label>
                    HTML Content
                    <textarea
                        placeholder="Enter HTML Content"
                        name="html_content"
                        rows="5"
                    ></textarea>
                  </label>
                  <br/>
                  <button type="submit">Submit</button>
                </form>
              </body>
            </html>
            "#
        ));

    response
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();

    response
}
