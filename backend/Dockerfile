# Stage 1: Build
FROM rust:1.75-bullseye as builder

WORKDIR /app

# Copiar archivos de manifiesto
COPY Cargo.toml Cargo.lock ./

# Crear un proyecto dummy para cachear dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copiar código fuente
COPY . .

# Actualizar timestamps para forzar recompilación
RUN touch src/main.rs

# Construir el proyecto
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

WORKDIR /app

# Instalar dependencias necesarias (OpenSSL, etc.)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copiar el binario desde el builder
COPY --from=builder /app/target/release/web_project /app/web_project

# Copiar archivos de migración y esquema si son necesarios en runtime
COPY schema.sql .
COPY migrations ./migrations

# Exponer el puerto (aunque Render lo maneja via env)
EXPOSE 8000

# Comando de ejecución
CMD ["./web_project"]
