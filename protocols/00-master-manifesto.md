# 🏛️ PROJECT SENTINEL: MASTER MANIFESTO (v4.0 - THE VQ WAY)

## 1. THE NORTH STAR (Temel Felsefe)
Project Sentinel, geleceği tahmin etmeye çalışan stokastik bir kumarbaz değil; mikro-yapı (microstructure) dinamiklerini, emir defteri dengesizliğini ve anlamsal duygu skorlarını mikrosaniye hassasiyetinde Z-Score ile normalize ederek geçmişteki vektörel anomalileri (Cosine Similarity) avlayan deterministik bir makinedir.

## 2. KESİN TEKNOLOJİ ANAYASASI (Sıfır Tolerans)
1. **Hot-Path Dili:** Veri akışı ve emir iletimi SADECE **Rust (tokio)** ile yazılır. Ağır tensor operasyonları **C++** ile yapılır. Python yalnızca çevrimdışı model eğitimi içindir; canlı sistemde yeri yoktur.
2. **Asenkron İletişim (IPC):** İç servisler arası HTTP/REST kesinlikle YASAKTIR. Tüm iletişim **NATS JetStream** ve **Protobuf (`prost`)** üzerinden zero-copy prensibiyle yapılır.
3. **Sıfır Bellek Tahsisi (Zero-Allocation):** Hot-loop içinde (tokenization, parsing) heap allocation ihanettir. `bytes::BytesMut` ve referanslar (`&str`) kullanılır. Veri bekleyemez; veri akışı durduğunda sistem körleşir.
4. **Veri Katmanı:** Zaman serisi ve tick-data için **QuestDB** (ILP), Vektörel hafıza için **Qdrant** (gRPC).

## 3. MLOps VE INFERENCE STRATEJİSİ (Baked Models)
* **Runtime Download Yasaktır:** Makine öğrenmesi modelleri sistem çalışırken internetten çekilemez. Modeller ONNX veya Safetensors formatına dönüştürülür, INT8/INT4 Quantize edilir ve Docker imajına "Baked" (Gömülü) edilir.
* **Inference Motoru:** Üretim ortamında saf Rust çözümleri yerini **ONNX Runtime (`ort`)** veya **TensorRT** C++ binding'lerine bırakır. Uzun süren AI hesaplamaları `tokio::time::timeout` ile sınırlandırılır.

## 4. MATEMATİKSEL FÜZYON & TİCARET FELSEFESİ
* **Matematik:** Çok boyutlu vektörler doğrudan birleştirilemez. Her metrik kendi hareketli ortalamasına göre `(x - μ)/σ` formülüyle **Z-Score Normalizasyonuna** tabi tutulur. 
* **Tetikleyici (Trigger):** Haber/Sentiment sadece rejimi doğrular (Risk-On/Off). Asıl tetikleyici Orderbook Imbalance ve fiyat ivmesidir.
* **Mikro-Cüzdan Prensibi:** 10 USD bakiye ile borsanın gecikme bariyerlerini aşabilen sistem, 10 Milyon Doları da yönetebilir.

## 5. FAIL-FAST, RECOVER-FASTER
Hız sadece kâr için değil, sistemin güvenliği içindir. Hot-path içindeki bir servis belirlenen SLA süresinde (Örn: 1ms) cevap vermezse, o veri paketini bekletmez, "Drop" eder (düşürür). Çöken bir model anında O(1) Lexicon moduna düşer (Graceful Degradation). HFT'de beklemek, batmaktır.