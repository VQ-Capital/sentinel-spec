# 🏛️ PROJECT SENTINEL: ANA MİMARİ & MANİFESTO (v2.0)

## 1. TEMEL FELSEFE
Project Sentinel, **Kurumsal Seviyede Olay Tabanlı Algoritmik Trading Altyapısıdır**. 
Amacımız, geleceği tahmin etmek değil; mevcut piyasa durumunu mikrosaniye hassasiyetinde tanımlamak ve geçmişteki vektörel benzerlikleri kullanarak "Haksız Avantaj" (Unfair Advantage) elde etmektir.

## 2. ANAYASAL KURALLAR (Sıfır Tolerans)
1. **Dil Kısıtı:** Hot-path (Veri akışı ve Emir iletimi) SADECE **Rust** (tokio) olmak zorundadır. Ağır matematiksel modelleme ve TensorRT operasyonları **C++** ile yapılır. Python sadece çevrimdışı analiz için kullanılabilir.
2. **İletişim Kısıtı:** İç servisler arası HTTP/REST yasaktır. Tüm iletişim **NATS JetStream** üzerinden asenkron ve olay güdümlü yapılır.
3. **Serileştirme:** Tüm veri paketleri **Protobuf** (`prost`) ile paketlenir. Dahili JSON trafiği yasaktır.
4. **Veritabanı:** Zaman serisi için **QuestDB** (ILP Protokolü), Vektörel hafıza için **Qdrant** (gRPC) kullanılır.
5. **Bellek Güvenliği:** Hot-loop içinde bellek tahsisi (allocation) yapılamaz. `.unwrap()` ve `.expect()` kullanımı mimari ihanettir; her hata `anyhow::Result` ile yönetilmelidir.

## 3. MİKRO-CÜZDAN (STRESS TEST) PRENSİBİ
Sistem, **10 USD bakiye** ile borsanın komisyon ve gecikme bariyerlerini aşabilecek kadar verimli çalışmak üzere kalibre edilmiştir. 10 Doları büyütebilen sistem, 10 Milyon Doları da büyütebilir. Matematik ölçeklenebilirdir.

## 4. SPEC-FIRST GELİŞTİRME
Kodda yapılan her mantıksal değişiklik (Örn: Stop-Loss yüzdesi), önce `sentinel-spec/logic/` altındaki YAML dosyalarına işlenmelidir. Spec, sistemin **Zeka Kaynağıdır**.