# 🦅 VQ-CAPITAL MASTER PROTOCOL & CONSTITUTION

Bu belge, VQ-Capital ekosistemindeki tüm mikroservislerin, yapay zeka ajanlarının ve geliştiricilerin uymak zorunda olduğu **Tek Doğruluk Kaynağıdır (Source of Truth)**.

## 1. MİMARİ FELSEFE (The VQ Way)
Sistemimiz, mikrosaniye (HFT) gecikmeleri hedefleyen, olay güdümlü (Event-Driven) ve istatistiksel / vektörel tahminlere dayalı bir ticaret omurgasıdır.

- **Diller:** Hot-path üzerinde SADECE `Rust` (tokio) ve `C++` kullanılabilir. Python/NodeJS YASAKTIR.
- **İletişim:** İç iletişimde SADECE `NATS JetStream` ve `Protobuf` (prost) kullanılır. REST/JSON YASAKTIR.
- **Veritabanları:** Zaman serisi için `QuestDB`, Vektörel durum hafızası için `Qdrant`.
- **Yapay Zeka (Faz 4 Kuralı):** Dış API (OpenAI) veya yerel HTTP/REST tabanlı LLM (Ollama) köprüleri YASAKTIR. Modeller `candle` framework'ü ile doğrudan Rust binary'sine gömülmeli ve VRAM üzerinden O(1) hızında çalışmalıdır.
- **Sıfır-Tolerans:** Rust kodlarında `.unwrap()` veya `.expect()` kullanmak YASAKTIR. Hata yoksayılır ve devam edilir.
- **Protobuf Disiplini:** Buf Linter kurallarına SIFIR HATA ile uyulacaktır.

## 2. REPO TOPOLOJİSİ VE GÖREVLERİ
1. **`sentinel-spec`:** (SİZ BURADASINIZ). Şemaların tek doğruluk kaynağıdır.
2. **`sentinel-ingest`:** Multi-Coin WS'den veriyi alır, Protobuf'a çevirip NATS'a basar.
3. **`sentinel-storage`:** NATS'ı dinler, verileri QuestDB ve Qdrant'a yazar.
4. **`sentinel-intelligence`:** Native Rust ML (Candle) kullanan SIFIR GECİKMELİ NLP gRPC sunucusu.
5. **`sentinel-inference`:** Çoklu sembol (Multi-coin) izole pencere yöneticisi ve Qdrant Vektör Arama motoru.
6. **`sentinel-execution`:** Sinyalleri dinler. Risk yönetimi ve Whipsaw koruması ile emri borsaya iletir.
7. **`sentinel-api` & `sentinel-terminal`:** Yönetim paneli (Flutter/Rust).
8. **`sentinel-infra`:** Docker GHCR imajları ile Production-Ready orkestrasyon deposu.