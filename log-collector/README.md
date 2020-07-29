# log-collector

## はじめに

本の内容がレガシー化したため, [ghmagazine/rustbook#2](https://github.com/ghmagazine/rustbook/pull/2) を写経する.

## 実行手順

(*多分* 環境依存ではなく…) 所々動作しない. 一旦コミットして後から直す.

### Docker 起動する

ローカル (Docker 外) 環境で PostgreSQL が動いていると詰む.

```console
docker-compose up -d
```

一度動かしたものがあるならば `docker restart` でも

### DB 作る

Docker 起動後, というよりはコンテナ作成後に一度初期化する.

```console
export DATABASE_URL=postgresql://postgres:password@localhost:5432/log-collector
cd server
diesel setup
diesel migration run
```

### デバッグ出力を有効にして起動する

```console
cd server
RUST_LOG=server=debug cargo run
```

### クエリ投げる

```console
curl -v -H 'Content-Type: application/json' \
  -d'{"user_agent": "Mozilla", "response_time": 200}' \
  localhost:3000/logs
```

あるいは `text.csv` のあるところで

```console
curl -H "Content-Type: text/csv" -F 'file=@test.csv;type=text/csv' http://localhost:3000/csv
```

### DB 見る

```console
docker-compose exec postgresql psql -U postgres log-collector
```

```console
log_collector=# \c log_collector
You are now connected to database "log_collector" as user "postgres".
log_collector=# select * from logs;
 id | user_agent | response_time |         timestamp          
----+------------+---------------+----------------------------
  1 | Mozilla    |           200 | 2020-07-18 14:34:46.930631
  2 | Mozilla    |           200 | 2020-07-27 11:55:48.391938
  3 | hogehoge   |            10 | 2017-08-26 13:13:29.93132
(3 rows)
```

`docker-compose exec postgresql psql -U postgres` で CUI 操作

### 圧縮済受け取る

:warning: なぜか動作しない

```console
curl -H 'Accept-Encoding: deflate, gzip' http://localhost:3000/csv > response.csv.gz
file response.csv.gz
```

### CLI ツール

```console
cd cli
cat ../test.csv | cargo run -- post
cargo run -- get --format json
```
