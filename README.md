# Calibre Ingest Web App

A simple web application for uploading files to a Calibre library management system.

## Features

- **Frontend**: Svelte 5 with Tailwind 4 for a clean, responsive interface
- **Backend**: Rust with Axum for fast, reliable file uploads
- **Docker**: Containerized deployment with docker-compose
- **CI/CD**: GitHub Actions for automated building and testing

## Quick Start

### Using Docker (Recommended)

1. Clone the repository
2. Run with docker-compose:

```bash
docker-compose up --build
```

3. Open http://localhost:8080 in your browser

### Development

#### Backend (Rust)
```bash
cd backend
cargo run
```

#### Frontend (Svelte)
```bash
cd frontend
npm install
npm run dev
```

## API

### POST /upload
Upload a file using multipart/form-data with a `file` field.

**Response:**
```json
{
  "success": true,
  "message": "File uploaded successfully",
  "filename": "uuid_originalname.ext",
  "size": 1234567
}
```

## Architecture

- **Frontend**: Runs on port 5173 (development) or served by nginx (production)
- **Backend**: Rust API server on port 3000
- **Reverse Proxy**: nginx routes frontend and API requests
- **File Storage**: Configurable upload directory with unique filenames

## Deployment

The application is designed for easy deployment:

1. **Docker Compose**: For local or single-server deployment
2. **GitHub Container Registry**: Automated builds push to GHCR
3. **Multi-platform**: Supports both AMD64 and ARM64 architectures

## Security Features

- File size limits (100MB default)
- Unique filename generation to prevent conflicts
- CORS configuration for cross-origin requests
- Input validation and error handling
