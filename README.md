# Actix-web: Rust web framework

> 這個專案最主要用於學習 Rust 語言，並且實作一個簡單的 Web 應用。

## Run Server:
```shell
# 產生 release 版本的應用
cargo build --release

# 在我電腦使用時，發生了一些動態鏈結庫缺失問題；故需要加上 DYLD_LIBRARY_PATH 讓 Rust 找到動態庫的位置。
DYLD_LIBRARY_PATH=/usr/local/lib:$DYLD_LIBRARY_PATH ./target/release/actix-api
```

## 使用到的套件介紹:

- `actix-web`: 一個 Rust 的 Web 框架。
- `actix-cors`: 一個 actix-web 的 CORS 中間件。
- `log`: 一個 Rust 的日誌框架。
- `env_logger`: 一個 log 的環境變數日誌輸出器。
- `serde`: 一個 Rust 的序列化/反序列化框架。 (features = ["derive"])
- `dotenvy`: 一個 Rust 的 .env 檔案解析器。
- `env_logger`: 一個 Rust 的環境變數日誌輸出器。
- `chrono`: 一個 Rust 的日期時間處理框架。
- `diesel`: 一個 Rust 的 ORM 框架。
- `r2d2`: 一個 Rust 的資料庫連線池框架。
- `lazy_static`: 一個 Rust 的靜態變數框架。 (用於初始化與 global 變數 - Arc ... lib.rs 中使用)

## Benchmark:

### K6:

```shell
brew install k6

k6 run --vus 10 --duration 30s http://localhost:8999

## 或是
k6 run ./k6-script.js
```

## 參考:
[actix-example](https://github.com/actix/examples)
