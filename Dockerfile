# Gunakan base image Rust
FROM rust:latest

# Set working directory
WORKDIR /app

# Salin file manifest
COPY Cargo.toml .

# Salin file Cargo.lock jika ada
COPY Cargo.lock .

# Unduh dan kompilasi dependensi tanpa kode sumber
RUN cargo fetch

# Salin semua file sumber ke container
COPY src ./src

# Kompilasi aplikasi dalam mode release
RUN cargo build --release

EXPOSE 8000

# Jalankan aplikasi
CMD ["/app/target/release/rust-firstproject"]