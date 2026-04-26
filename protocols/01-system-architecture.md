# SOP-01: System Architecture & Data Flow (Sistem Mimarisi)

Bu belge, sistemi sıfırdan inşa edecek bir mühendisin (veya AI Ajanının) kuracağı iskeleti tanımlar.

## 1. Veri Akışı (Data Pipeline)
Sistem "Soldan Sağa" akar. Asla ters yönde (Sağdan Sola) senkronize istek atılmaz.
*   **[INGEST Katmanı]** Dış dünyayı (WSS/RSS) dinler `->` NATS'a yazar. (Sadece Veri Üretir, okumaz).
*   **[BRAIN Katmanı]** NATS'ı okur `->` Z-Score/ONNX işler `->` Sinyal Üretir. (`sentinel-inference`, `sentinel-intelligence`).
*   **[EXECUTION Katmanı]** NATS'ı okur `->` Borsaya emir atar.
*   **[STORAGE Katmanı]** NATS'ı okur `->` QuestDB/Qdrant'a yazar.

## 2. Repo İskeleti (Rust Boilerplate)
Her Rust reposu şu standartla sıfırdan oluşturulmalıdır:
1. `Cargo.toml` içinde `tokio`, `async-nats`, `prost`, `tracing` zorunludur.
2. `build.rs` kullanılarak `sentinel-spec/proto/` altındaki `.proto` dosyaları derlenir.
3. `src/main.rs` içinde `.unwrap()` kullanılamaz, `anyhow::Result` ile çalışılır.
4. Her mikroservis NATS'a kopma durumunda tekrar bağlanacak `retry` mantığına (Exponential Backoff) sahip olmalıdır.