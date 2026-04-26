# SOP-02: Quant Mathematics & AI Integration

Sistemi sıfırdan kurarken, `sentinel-inference` (Beyin) ve `sentinel-intelligence` (Duygu) servislerinde şu matematiksel kurallar koda dökülmelidir:

## 1. Z-Score Normalizasyonu (Kritik!)
Çok boyutlu vektörler Qdrant'a atılmadan önce mutlaka normalize edilmelidir.
*   Fiyat Hızı (Velocity) mikro ondalıklar (`0.00005`) ürettiği için `Z_SCALE` (Örn: 10000.0) ile çarpılarak büyütülmeli, ardından Hareketli Ortalama ve Varyans kullanılarak Z-Skoru hesaplanmalıdır.
*   **Clamp:** Z-Skorlar mutlaka `[-3.0, 3.0]` aralığına sıkıştırılmalıdır (Outlier koruması).

## 2. Yapay Zeka (NLP) Çalışma Zamanı
*   Kullanılacak model: `FinancialBERT` (veya benzeri).
*   Format: Kesinlikle `ONNX` formatında olmalı ve Docker imajına build anında eklenmelidir (Baked Model).
*   Çıkarım (Inference) Engine: Rust `ort` kütüphanesi kullanılacaktır.
*   **SLA:** AI işlemleri `tokio::time::timeout` içine alınmalı. Eğer 15ms içinde yanıt dönmezse işlem düşürülmeli (Graceful Degradation) ve sistem yola devam etmelidir.