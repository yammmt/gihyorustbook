use actix_web::{error, web, App, HttpResponse, HttpServer};
use serde_derive::*;
use tera::{Context, Tera};

struct AppState {
    template: Tera,
}

#[derive(Deserialize)]
struct HelloPath {
    name: String,
}

async fn hello_template(
    app: web::Data<AppState>,
    path: web::Path<HelloPath>,
) -> Result<HttpResponse, error::Error> {
    // テンプレートに渡す値を作る
    let mut context = Context::new();
    context.insert("name", &path.name);
    // `app.template` でテンプレートが取得できる
    let body = app
        .template
        // `Tera::render` で指定したテンプレートをレンダリングできる
        .render("index.html.tera", &context)
        // レンダリングに失敗したらサーバ内部のエラーとして扱う
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;
    // レンダリング結果をレスポンスとしてステータス 200 OK で返す
    Ok(HttpResponse::Ok().body(body))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // `AppState` を準備する
        let app = AppState {
            template: Tera::new("templates/**/*").unwrap(),
        };
        App::new()
            .data(app)
            .route("/{name}", web::get().to(hello_template))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
