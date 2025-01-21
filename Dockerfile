   # Gunakan image dasar Rust  
   FROM rust:latest  
  
   # Set direktori kerja  
   WORKDIR /app  
  
   # Salin file Cargo.toml  
   COPY Cargo.toml ./  
  
   # Salin folder src  
   COPY src ./src  
  
   # Instal dependensi proyek  
   RUN cargo build --release  
  
   # Jalankan aplikasi  
   CMD ["cargo", "run", "--release"]  
