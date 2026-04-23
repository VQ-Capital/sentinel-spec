# SOP-04: Multi-Agent Context Matrix (Böl ve Yönet)

VQ-Capital projesi HFT kurallarına göre çalıştığı için tek bir otonom ajanın (LLM) tüm sistemi bilmesine gerek yoktur. Geliştirme şu 3 İzole Odaya (Squad) bölünür:

## 🟥 SQUAD 1: DATA INGESTION & STORAGE (Vacuum)
**Sorumluluk:** Dış dünyadan (Binance) gelen veriyi sıfır kayıpla alıp NATS'a basmak ve Storage'a (QuestDB) yazmak.
**Bağlam (Snapshot) Verilecek Repolar:** `sentinel-ingest`, `sentinel-storage`, `sentinel-spec`.
**Kural:** Bu ekip asla işlem (Trade) açamaz, PnL hesaplayamaz. İş mantığı ekleyemez.

## 🟦 SQUAD 2: AI INFERENCE & INTELLIGENCE (Brain)
**Sorumluluk:** Vektör uzayı matematiği (Cosine Similarity), Qdrant aramaları ve NLP (Duygu Analizi).
**Bağlam (Snapshot) Verilecek Repolar:** `sentinel-inference`, `sentinel-intelligence`, `sentinel-spec`.
**Kural:** Bu ekip bakiye bilemez, borsa komisyonu hesaplayamaz. Sadece `TradeSignal` (Sinyal) üretir.

## 🟩 SQUAD 3: RISK & EXECUTION (Sniper)
**Sorumluluk:** Sinyali alır, Drawdown ve Whipsaw korumasını işletir, emri borsaya çeker ve PnL yazar.
**Bağlam (Snapshot) Verilecek Repolar:** `sentinel-execution`, `sentinel-api`, `sentinel-terminal`, `sentinel-spec`.
**Kural:** Bu ekip asla geçmiş verilere bakıp tahmin (Prediction) yapamaz. Tahmin işi Squad 2'nin görevidir.