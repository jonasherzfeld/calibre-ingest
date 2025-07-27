# Backend build stage
FROM rust:1.82-alpine AS backend-builder

# Install build dependencies for Alpine
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY backend/src ./src
RUN touch src/main.rs
RUN cargo build --release

# Frontend build stage
FROM node:20-alpine AS frontend-builder

WORKDIR /app
COPY frontend/package.json ./
RUN npm install

COPY frontend/ ./
RUN npm run build

# Final runtime image
FROM nginx:alpine

# Install required packages
RUN apk add --no-cache ca-certificates shadow su-exec

# Copy nginx config
COPY nginx.conf /etc/nginx/nginx.conf

# Copy frontend build
COPY --from=frontend-builder /app/build /usr/share/nginx/html

# Copy backend binary
COPY --from=backend-builder /app/target/release/calibre-ingest-backend /usr/local/bin/

# Create upload directory
RUN mkdir -p /uploads

# Copy and set up startup script
COPY start.sh /start.sh
RUN chmod +x /start.sh

EXPOSE 80

CMD ["/start.sh"]
