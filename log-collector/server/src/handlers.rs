use crate::db;
use crate::Server;

use actix_multipart::Multipart;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse, Result};
use diesel::pg::PgConnection;
use futures::prelude::*;
use itertools::Itertools;
use log::debug;
use std::io::{self, prelude::*, Seek, SeekFrom};

async fn load_file(conn: &PgConnection, file: impl Read) -> Result<usize> {
    use crate::model::NewLog;

    let mut ret = 0;

    let in_csv = io::BufReader::new(file);
    let in_log = csv::Reader::from_reader(in_csv).into_deserialize::<::api::Log>();
    // Itertools の chunks を用いて 1,000 件ずつ処理する
    // blocking
    for logs in &in_log.chunks(1000) {
        let logs = logs
            .filter_map(Result::ok)
            .map(|log| NewLog {
                user_agent: log.user_agent,
                response_time: log.response_time,
                timestamp: log.timestamp.naive_utc(),
            })
            .collect_vec();

        // blocking
        let inserted = db::insert_logs(conn, &logs).map_err(ErrorInternalServerError)?;
        ret += inserted.len();
    }

    Ok(ret)
}

/// POST /csv のハンドラ
pub async fn handle_post_csv(
    server: web::Data<Server>,
    mut multipart: Multipart,
) -> Result<HttpResponse> {
    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    let mut total_size = 0;

    while let Some(field) = multipart.next().await {
        // データ (ファイル) を取り出す度に `await` しているので並列性質｀はない.
        // ただし他のリクエストハンドラが動けるのでサーバ全体のパフォーマンスは上がる.

        let mut field = field.map_err(ErrorBadRequest)?;
        if field.content_type().as_ref() != "text/csv" {
            continue;
        }

        // これで `curl -F` 経由のファイルがスキップされる
        // disposition が FormData になっているため
        if !field
            .content_disposition()
            .map(|c| c.is_attachment())
            .unwrap_or(false)
        {
            continue;
        }

        // 一旦データをファイルに書き出す
        // blocking
        let mut file = io::BufWriter::new(tempfile::tempfile().map_err(ErrorInternalServerError)?);
        // データが分割されて送られてくるのでチマチマ受け取ってファイルに書き出す
        while let Some(data) = field.next().await {
            let data = data.map_err(ErrorInternalServerError)?;
            // blocking
            file.write(&data).map_err(ErrorInternalServerError)?;
        }
        file.seek(SeekFrom::Start(0)).unwrap();
        // 書き出したデータを DB にロードする
        // blocking
        let file = file.into_inner().map_err(ErrorInternalServerError)?;
        // blocking
        total_size = load_file(&conn, file).await?;
    }

    Ok(HttpResponse::Ok().json(api::csv::post::Response(total_size)))
}

/// POST /logs のハンドラ
pub async fn handle_post_logs(
    server: web::Data<Server>,
    log: web::Json<api::logs::post::Request>,
) -> Result<HttpResponse> {
    use crate::model::NewLog;
    use chrono::Utc;

    let log = NewLog {
        user_agent: log.user_agent.clone(),
        response_time: log.response_time,
        timestamp: log.timestamp.unwrap_or_else(|| Utc::now()).naive_utc(),
    };
    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    db::insert_log(&conn, &log).map_err(ErrorInternalServerError)?;

    debug!("received log: {:?}", log);

    Ok(HttpResponse::Accepted().finish())
}

/// GET /logs のハンドラ
pub async fn handle_get_logs(
    server: web::Data<Server>,
    range: web::Query<api::logs::get::Query>,
) -> Result<HttpResponse> {
    use chrono::{DateTime, Utc};
    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    debug!("{:?}", range);
    // blocking
    let logs = db::logs(&conn, range.from, range.until).map_err(ErrorInternalServerError)?;
    let logs = logs
        .into_iter()
        .map(|log| api::Log {
            user_agent: log.user_agent,
            response_time: log.response_time,
            timestamp: DateTime::from_utc(log.timestamp, Utc),
        })
        .collect();

    Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// GET /csv のハンドラ
pub async fn handle_get_csv(
    server: web::Data<Server>,
    range: web::Query<api::csv::get::Query>,
) -> Result<HttpResponse> {
    use chrono::{DateTime, Utc};

    let conn = server.pool.get().map_err(ErrorInternalServerError)?;
    debug!("{:?}", range);
    // blocking
    let logs = db::logs(&conn, range.from, range.until).map_err(ErrorInternalServerError)?;
    let v = Vec::new();
    let mut w = csv::Writer::from_writer(v);

    for log in logs.into_iter().map(|log| ::api::Log {
        user_agent: log.user_agent,
        response_time: log.response_time,
        timestamp: DateTime::from_utc(log.timestamp, Utc),
    }) {
        w.serialize(log).map_err(ErrorInternalServerError)?;
    }

    let csv = w.into_inner().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok()
        .header("Content-Type", "text/csv")
        .body(csv))
}
