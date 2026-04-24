# SOP-04: Multi-Agent Context Matrix (Böl ve Yönet)

VQ-Capital projesi HFT kurallarına göre çalıştığı için tek bir otonom ajanın (LLM) tüm sistemi bilmesine gerek yoktur. Geliştirme şu 3 İzole Odaya (Squad) bölünür:

## 🟥 SQUAD 1: DATA INGESTION & STORAGE (The Data Engineers)
**Sorumluluk:** Dış dünyadan (WebSockets, FIX, gRPC) gelen veriyi zero-allocation (`BytesMut`) prensibiyle sıfır kayıpla alıp NATS'a basmak ve Storage'a (QuestDB) yazmak. RSS gibi yavaş kaynakları reddedip Low-Latency kanallara odaklanmak.
**Bağlam (Snapshot) Verilecek Repolar:** `sentinel-ingest`, `sentinel-news-ingest`, `sentinel-storage`, `sentinel-spec`.
**Kural:** Bu ekip asla işlem (Trade) açamaz, PnL hesaplayamaz. İş mantığı ekleyemez.

## 🟦 SQUAD 2: AI INFERENCE & MLOps (The Quants & Data Scientists)
**Sorumluluk:** ONNX/TensorRT tabanlı INT8 Quantize edilmiş modelleri çalıştırmak. Z-Score normalizasyonu yapmak, Cosine Similarity ile Qdrant aramaları yapmak. SLA (4ms) aşılırsa Timeout ile işlemi kesip (Cancellation Safe) Lexicon'a düşmek.
**Bağlam (Snapshot) Verilecek Repolar:** `sentinel-inference`, `sentinel-intelligence`, `sentinel-spec`.
**Kural:** Bu ekip bakiye bilemez, borsa komisyonu hesaplayamaz. Sadece normalize edilmiş `TradeSignal` (Sinyal) üretir.

## 🟩 SQUAD 3: RISK & EXECUTION (The Core HFT Architects)
**Sorumluluk:** Sinyali alır, Drawdown ve Whipsaw korumasını işletir, emri borsaya çeker ve PnL yazar. C/C++ seviyesinde optimizasyonlarla emrin borsaya ulaşma süresini minimize eder.
**Bağlam (Snapshot) Verilecek Repolar:** `sentinel-execution`, `sentinel-api`, `sentinel-terminal`, `sentinel-spec`.
**Kural:** Bu ekip asla geçmiş verilere bakıp tahmin (Prediction) yapamaz. Tahmin işi Squad 2'nin görevidir.