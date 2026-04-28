# 🏛️ PROJECT SENTINEL: MASTER MANIFESTO (v5.0 - THE PURE RUST WAY)

## 1. THE NORTH STAR (Temel Felsefe)
Project Sentinel, geleceği tahmin etmeye çalışan stokastik bir kumarbaz değil; mikro-yapı (microstructure) dinamiklerini, emir defteri dengesizliğini ve anlamsal duygu skorlarını mikrosaniye hassasiyetinde Z-Score ile normalize ederek geçmişteki vektörel anomalileri (Cosine Similarity) avlayan deterministik bir makinedir.

## 2. KESİN TEKNOLOJİ ANAYASASI (Sıfır Tolerans)
1. **Hot-Path Dili (Pure Rust):** Veri akışı, emir iletimi ve YAPAY ZEKA ÇIKARIMI dahil her şey SADECE **Rust** ile yazılır. `C++`, `CUDA`, `Python` çalışma zamanında (runtime) KESİNLİKLE YASAKTIR.
2. **Asenkron İletişim (IPC):** İç servisler arası HTTP/REST kesinlikle YASAKTIR. Tüm iletişim **NATS JetStream** ve **Protobuf (`prost`)** üzerinden zero-copy prensibiyle yapılır.
3. **Sıfır Bellek Tahsisi (Zero-Allocation):** Hot-loop içinde (tokenization, parsing) heap allocation ihanettir. `bytes::BytesMut` ve referanslar (`&str`) kullanılır.
4. **Veri Katmanı:** Zaman serisi ve tick-data için **QuestDB** (ILP), Vektörel hafıza için **Qdrant** (gRPC).

## 3. MLOps VE INFERENCE STRATEJİSİ (Lock-Free & C++ Free)
* **Zero External Bindings:** Makine öğrenmesi modelleri için `ort` (ONNX Runtime) veya `libtorch` gibi C++ binding kütüphaneleri YASAKTIR. Modeller `tract` ile saf Rust'a derlenir veya `ndarray` ile manuel matris çarpımlarına (Hard-Coded Math) dönüştürülür.
* **Lock-Free Concurrency:** AI çıkarımlarında `Mutex` kilidi kullanılamaz. Modeller bellekte `Send + Sync` (TypedSimplePlan) olarak tutulup gerçek paralel çalıştırılır.
* **Baked Models:** Modeller Docker imajına build anında eklenir (Baked). Runtime sırasında indirme yapılamaz.

## 4. MATEMATİKSEL FÜZYON & TİCARET FELSEFESİ
* **Matematik:** Çok boyutlu vektörler doğrudan birleştirilemez. Her metrik kendi hareketli ortalamasına göre `(x - μ)/σ` formülüyle **Z-Score Normalizasyonuna** tabi tutulur. 
* **Mikro-Cüzdan Prensibi:** 10 USD bakiye ile borsanın gecikme bariyerlerini aşabilen sistem, 10 Milyon Doları da yönetebilir.

## 5. FAIL-FAST, RECOVER-FASTER
Hız sadece kâr için değil, sistemin güvenliği içindir. Eğer bir model belirtilen SLA süresinde (Örn: 25ms) cevap vermezse, `tokio::time::timeout` devreye girer, işlemi "Drop" eder ve Lexicon moduna düşer (Graceful Degradation).