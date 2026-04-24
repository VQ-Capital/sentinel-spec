# 🏛️ PROJECT SENTINEL: ANA MİMARİ & MANİFESTO (v3.0 - HFT & QUANT ERA)

## 1. TEMEL FELSEFE
Project Sentinel, **Kurumsal Seviyede Olay Tabanlı Algoritmik Trading Altyapısıdır**. 
Amacımız geleceği kahin gibi tahmin etmek değil; mikro-yapı (microstructure) dinamiklerini, emir defteri dengesizliğini (orderbook imbalance) ve anlamsal duygu skorlarını (semantic sentiment) Z-Score ile normalize ederek geçmişteki vektörel benzerlikleri (Cosine Similarity) bulmaktır.

## 2. ANAYASAL KURALLAR (Sıfır Tolerans)
1. **Dil Kısıtı:** Hot-path (Veri akışı ve Emir iletimi) SADECE **Rust** (tokio) olmak zorundadır. Ağır matematiksel modelleme ve TensorRT/ONNX operasyonları **C++** ile yapılır. Python sadece çevrimdışı analiz ve model eğitimi içindir.
2. **İletişim Kısıtı:** İç servisler arası HTTP/REST yasaktır. Tüm iletişim **NATS JetStream** ve **Protobuf** (`prost`) üzerinden asenkron yapılır.
3. **Zero-Allocation (Sıfır Bellek Tahsisi):** Hot-loop içinde (Özellikle Tokenization ve JSON parsing süreçlerinde) heap allocation yapılamaz. `bytes::BytesMut` ve referanslar (`&str`) kullanılır.
4. **Veritabanı:** Zaman serisi için **QuestDB** (ILP Protokolü), Vektörel hafıza için **Qdrant** (gRPC).
5. **Bellek Güvenliği & Timeout:** `.unwrap()` kullanımı yasaktır. Uzun süren AI/Tensor hesaplamaları `tokio::time::timeout` ile sınırlandırılır. Model belirlenen SLA süresinde (Örn: 4ms) yanıt vermezse `abort()` edilir ve anında O(1) Lexicon moduna geçilir (Graceful Degradation).

## 3. MLOps VE INFERENCE STRATEJİSİ (Baked Models)
* **Runtime Download Yasaktır:** Makine öğrenmesi modelleri (LLM/SLM) sistem çalışırken internetten çekilemez. Modeller çevrimdışı olarak ONNX veya Safetensors formatına dönüştürülür, INT8/FP16 olarak Quantize edilir ve Docker imajının içine (Baked) statik dosya olarak yerleştirilir.
* **Inference Motoru:** Prototipler için kullanılan pure-Rust çözümleri, production ortamında yerini **ONNX Runtime (`ort`)** veya **TensorRT** C++ binding'lerine bırakacaktır.

## 4. MATEMATİKSEL FÜZYON (Z-Score Normalization)
Çok boyutlu vektörler `[Price_Velocity, Volume_Imbalance, Sentiment_Score]` doğrudan birleştirilemez. Her metrik kendi geçmiş hareketli ortalamasına (Rolling Mean) göre `(x - μ)/σ` formülüyle **Z-Score Normalizasyonuna** tabi tutulur ve ağırlıklandırılarak Qdrant'a gönderilir. Sentiment (Duygu), fiyatı önceden tahmin etmez; fiyatın yarattığı anomalinin (flash crash) gerçekliğini teyit eder.

## 5. MİKRO-CÜZDAN PRENSİBİ
Sistem, **10 USD bakiye** ile borsanın komisyon ve gecikme bariyerlerini aşabilecek kadar verimli çalışmak üzere kalibre edilmiştir. 10 Doları büyütebilen sistem, 10 Milyon Doları da büyütebilir. Matematik ölçeklenebilirdir.