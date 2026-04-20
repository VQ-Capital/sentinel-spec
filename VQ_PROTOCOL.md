# 🦅 VQ-CAPITAL MASTER PROTOCOL & CONSTITUTION

Bu belge, VQ-Capital ekosistemindeki tüm mikroservislerin, yapay zeka ajanlarının ve geliştiricilerin uymak zorunda olduğu **Tek Doğruluk Kaynağıdır (Source of Truth)**.

## 1. MİMARİ FELSEFE (The VQ Way)
Sistemimiz, mikrosaniye (HFT) gecikmeleri hedefleyen, olay güdümlü (Event-Driven) ve istatistiksel / vektörel tahminlere dayalı bir ticaret omurgasıdır.

- **Diller:** Hot-path (Veri ve Emir akışı) üzerinde SADECE `Rust` (tokio) ve `C++` kullanılabilir. İçeride Python/NodeJS YASAKTIR.
- **İletişim:** Tüm servisler birbirleriyle SADECE `NATS JetStream` üzerinden konuşur. İç ağda HTTP REST, gRPC (Intelligence hariç) ve JSON kesinlikle YASAKTIR.
- **Serileştirme:** Tüm veri paketleri `Protobuf` (prost) formatındadır.
- **Veritabanları:** Zaman serisi ve loglar için `QuestDB`, Vektörel durum hafızası ve AI aramaları (Cosine Similarity) için `Qdrant`.
- **Sıfır-Tolerans (Zero-Tolerance):** Rust kodlarında runtime anında `.unwrap()` veya `.expect()` kullanmak YASAKTIR. Bozuk veri gelirse paket düşürülür (Drop), loglanır ve sistem çökmeksizin devam eder.

## 2. REPO TOPOLOJİSİ VE GÖREVLERİ
1. **`sentinel-spec`:** (SİZ BURADASINIZ). Hiçbir uygulama kodu içermez. Protobuf şemalarını, NATS kanal listesini ve sistem protokolünü (bu dosya) barındırır.
2. **`sentinel-ingest`:** Dış dünyadan (WS) veriyi alır, Protobuf'a çevirip NATS'a basar. Hiçbir hesaplama yapmaz.
3. **`sentinel-storage`:** NATS'ı dinler, verileri QuestDB ve Qdrant'a yazar. Hata anında (auto-reconnect) ile kendini kurtarır.
4. **`sentinel-intelligence`:** Gömülü sözlük (Compile-time Lexicon) kullanan, O(1) hızında Rust tabanlı NLP/Duygu Analizi (gRPC) sunucusu.
5. **`sentinel-inference`:** Yapay Zeka Beyni. NATS'tan veriyi alır, 3D Vektör çıkarır, Qdrant'ta Cosine Similarity arar ve "Sinyal" (Signal) üretir.
6. **`sentinel-execution`:** Sinyalleri dinler, Risk yönetimi yapar. **Gölge Borsa (Shadow Exchange)** mantığıyla NATS üzerinden canlı fiyatlara bakarak gerçekçi Slippage, Latency ve Komisyon hesaplayarak Paper Trade yapar.
7. **`sentinel-api` & `sentinel-terminal` (Gelecek Vizyonu):** Sistemin yönetim paneli. Terminal (Flutter) üzerinden sisteme API key girilir, simülasyon başlatılır veya durdurulur.
8. **`sentinel-infra`:** Tüm bu sistemin Docker (GHCR imajları ile) üzerinden saniyeler içinde Production-Ready ayağa kalkmasını sağlayan orkestrasyon deposu.

## 3. YAPAY ZEKA DEVAMLILIK PROTOKOLÜ (AI Continuation Protocol)
Ey Yapay Zeka! Yeni bir oturum açtığında ilk yapman gereken bu dosyayı okumaktır. 
**ŞU ANKİ AŞAMA (Phase 5):** Kurumsallaşma ve Protobuf standardizasyonu.
**SONRAKİ GÖREVLER:**
- Tüm repolardaki yerel `proto/` klasörlerini silip `sentinel-spec` reposunu Git Submodule olarak eklemek.
- `docker-compose.yml` altyapısını GHCR hazır imajlarını çekecek şekilde Prod-Ready formata sokmak.
- Flutter (sentinel-terminal) ve Rust API (sentinel-api) repolarını başlatarak sistemi komuta edilebilir hale getirmek.