# 🦅 SENTINEL-SPEC (The Source of Truth)

Bu repository, VQ-Capital ekosisteminin **Tek Doğruluk Kaynağıdır (Source of Truth)**. 
Sisteme dahil olan bir İnsan veya Yapay Zeka (AI Agent), **sadece bu repoya bakarak** tüm HFT mimarisini, mikroservisleri ve Quant matematiğini sıfırdan, %100 doğrulukla yeniden inşa edebilir.

## 🧭 Kutsal Kitaplar (The Protocols)
Projeyi kodlamadan önce sırasıyla okunması gereken anayasa maddeleri:
*   📜 **[00: Master Manifesto](protocols/00-master-manifesto.md)**: Temel teknoloji yasakları ve felsefemiz (Zero-Allocation, No Python).
*   🏗️ **[01: System Architecture](protocols/01-system-architecture.md)**: Veri akış şeması ve Rust kodlama şablonları (Boilerplate).
*   🧮 **[02: Quant Math & AI](protocols/02-quant-math-and-ai.md)**: Z-Score formülleri, NLP kuralları ve ONNX yönergeleri.
*   ⚙️ **[03: Infrastructure](protocols/03-infrastructure.md)**: Docker, NATS ve Qdrant altyapı standartları.
*   🤖 **[04: Multi-Agent Workflow](protocols/04-multi-agent-workflow.md)**: AI ajanlarının repolarda nasıl bölünüp çalışacağı.

## 🧩 Sistem Tanımları
*   🧠 **[İş Mantığı (Logic)](spec/logic/)**: Risk motoru ve AI çıkarım parametreleri (YAML).
*   🕸️ **[Sistem Topolojisi (Events)](spec/events/)**: NATS ağ haritası (Mesh) ve sunucu yapılandırmaları.
*   🧬 **[Veri Sözleşmeleri (Proto)](proto/)**: Sistemin tek iletişim dili olan Protobuf tanımları.

> **NOT:** Koddaki her mantıksal değişim (yeni bir NATS kanalı veya yeni bir servis), önce buradaki YAML ve Proto dosyalarında güncellenmelidir. `sentinel-spec` güncellenmeden kod yazılamaz.