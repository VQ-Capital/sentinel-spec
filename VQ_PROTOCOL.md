# 🦅 VQ-CAPITAL MASTER PROTOCOL & CONSTITUTION

Bu belge, VQ-Capital ekosistemindeki tüm mikroservislerin, yapay zeka ajanlarının ve geliştiricilerin uymak zorunda olduğu **Tek Doğruluk Kaynağıdır (Source of Truth)**.

## 1. MİMARİ FELSEFE (The VQ Way)
Sistemimiz, mikrosaniye (HFT) gecikmeleri hedefleyen, olay güdümlü (Event-Driven) ve istatistiksel / vektörel tahminlere dayalı bir ticaret omurgasıdır.

- **Diller:** Hot-path üzerinde SADECE `Rust` (tokio) ve `C++` kullanılabilir. Python/NodeJS YASAKTIR.
- **İletişim:** İç iletişimde SADECE `NATS JetStream` ve `Protobuf` (prost) kullanılır. REST/JSON YASAKTIR.
- **Veritabanları:** Zaman serisi için `QuestDB`, Vektörel durum hafızası için `Qdrant`.
- **Sıfır-Tolerans:** Rust kodlarında `.unwrap()` veya `.expect()` kullanmak YASAKTIR. Hata yoksayılır ve devam edilir.
- **Protobuf Disiplini (Buf Standartları):** Proto dosyalarında Google Buf Linter kurallarına SIFIR HATA ile uyulacaktır. Klasör dizinleri paket adlarıyla eşleşecek, tüm Request/Response ve Service'ler standartlara uygun isimlendirilecektir. Tüm mesajlar eksiksiz DÖKÜMANTE EDİLECEKTİR. Kopyala/Yapıştır YASAKTIR (Git Submodule kullanılır).

## 2. REPO TOPOLOJİSİ VE GÖREVLERİ
1. **`sentinel-spec`:** (SİZ BURADASINIZ). Şemaların tek doğruluk kaynağıdır. Diğer repolara Git Submodule olarak eklenir.
2. **`sentinel-ingest`:** WS'den veriyi alır, Protobuf'a çevirip NATS'a basar.
3. **`sentinel-storage`:** NATS'ı dinler, verileri QuestDB ve Qdrant'a yazar.
4. **`sentinel-intelligence`:** Gömülü sözlük (Compile-time) kullanan NLP gRPC sunucusu.
5. **`sentinel-inference`:** Yapay Zeka Beyni. 3D Vektör çıkarır, Qdrant'ta Cosine Similarity arar ve "Sinyal" üretir.
6. **`sentinel-execution`:** Sinyalleri dinler. Gölge Borsa mantığıyla Gerçekçi Slippage ve Latency simüle ederek PnL hesaplar.
7. **`sentinel-api` & `sentinel-terminal`:** Yönetim paneli.
   - **Kural:** Terminal kodu İÇİNDE ASLA sabit (hardcoded) IP/URL bulunamaz. Tüm bağlantı ayarları derleme anında (Compile-time) `--dart-define` ile veya çalışma anında alınmalıdır. 
   - **Kural:** `sentinel-api`, NATS üzerinden okuduğu Protobuf verisini WebSocket zarfına (`StreamBundle`) koyar ve `0.0.0.0:8080` üzerinden dış dünyaya (CORS serbest) açar.
8. **`sentinel-infra`:** Docker GHCR imajları ile Production-Ready orkestrasyon deposu.
