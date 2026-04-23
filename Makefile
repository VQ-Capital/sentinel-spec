.PHONY: build generate lint clean

# Rust aracımızı (release modda hızlıca) derler
build:
	@echo "🦀 Building Rust CLI tools..."
	@cd scripts/vq-spec-tools && cargo build --release

# Dokümanları üretir
generate: build
	@echo "🚀 Executing Rust Generator..."
	@cd scripts/vq-spec-tools && cargo run --release --quiet -- generate

# Linter'ı çalıştırır (CI/CD bu komutu kullanacak)
lint: build
	@echo "🔍 Executing Rust Zero-Tolerance Linter..."
	@cd scripts/vq-spec-tools && cargo run --release --quiet -- lint \
		--target-dir ../../ \
		--repo-name sentinel-spec \
		--spec-dir ../../

clean:
	@echo "🧹 Cleaning up Rust artifacts and dist folder..."
	rm -rf dist/
	@cd scripts/vq-spec-tools && cargo clean