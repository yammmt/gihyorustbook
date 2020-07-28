use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // これではレポジトリ以下全ファイルルーティングされる
            // 公開範囲を制限するならば、例えば `Files::new("/a", "src/")` とすれば
            // `http://localhost:3000/a/main.rs` が有効になる
            Files::new("/", "")
                .show_files_listing(),
        )
    })
    .bind("localhost:3000")?
    .run()
    .await
}
