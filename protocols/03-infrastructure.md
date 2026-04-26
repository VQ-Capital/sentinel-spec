# SOP-03: Infrastructure & CI/CD (Altyapı)

Bu belge, projenin canlı ortama nasıl taşınacağını tanımlar (`sentinel-infra` reposunu oluşturma rehberi).

## 1. Docker Topolojisi
Tüm sistem `docker-compose` ile ayağa kalkmalıdır.
*   **Ağ:** Bütün servisler aynı `bridge` network (örn: `sentinel-net`) içinde olmalıdır.
*   **Değişkenler:** Koda gömülü (Hardcoded) parametreler YASAKTIR. Tüm eşik değerleri (Threshold, Scale, Window) `.env` dosyasından okunmalıdır.

## 2. State (Hafıza) Yönetimi
Sistem kapatılıp açıldığında (Crash) NATS JetStream verileri kalıcı (File Storage) olmalı, QuestDB ve Qdrant volümleri host makineye map edilmelidir (`/var/lib/questdb` vb.).

## 3. GitHub Actions
Her repoda `.github/workflows/docker-build.yml` olmalıdır. Bu dosya kodu derleyip, GitHub Container Registry'ye (`ghcr.io/vq-capital/repismi:latest`) göndermelidir.