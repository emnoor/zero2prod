use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn submit_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    let idempotency_key = uuid::Uuid::new_v4();

    HttpResponse::Ok()
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
                {msg_html}
                <form action="/admin/newsletters" method="post">
                  <label>Title:
                    <input
                        type="text"
                        placeholder="Enter the issue title"
                        name="title"
                    />
                  </label>
                  <br/>
                  <label>Plain text content<br>
                    <textarea
                        placeholder="Enter the content in plain text"
                        name="text_content"
                        rows="20"
                        cols="50"
                    ></textarea>
                  </label>
                  <br/>
                  <label>HTML content:
                    <textarea
                        placeholder="Enter the content in HTML"
                        name="html_content"
                        rows="20"
                        cols="50"
                    ></textarea>
                  </label>
                  <br/>
                  <input hidden type="text" name="idempotency_key" value="{idempotency_key}">
                  <button type="submit">Publish</button>
                </form>
                <p><a href="/admin/dashboard">&lt;- Back</a></p>
              </body>
            </html>
            "#
        ))
}
