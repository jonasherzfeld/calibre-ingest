use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::{Method, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::{env, path::Path};
use tokio::{fs, io::AsyncWriteExt};
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let upload_dir = "./uploads";
    let allowed_extensions = env::var("ALLOWED_FILE_TYPES").unwrap_or_else(|_| "epub,pdf,mobi,azw,azw3,txt".to_string());
    
    info!("Upload directory: {}", upload_dir);
    info!("Allowed file types: {}", allowed_extensions);

    // Create upload directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&upload_dir).await {
        warn!("Failed to create upload directory: {}", e);
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(health_check))
        .route("/upload", post(upload_file))
        .layer(cors)
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024)); // 25MB limit

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    let allowed_extensions = env::var("ALLOWED_FILE_TYPES").unwrap_or_else(|_| "epub,pdf,mobi,azw,azw3,txt".to_string());
    
    Json(json!({
        "status": "ok",
        "message": "Calibre Ingest Backend",
        "allowed_file_types": allowed_extensions.split(',').collect::<Vec<&str>>(),
        "max_file_size_mb": 25
    }))
}

fn is_allowed_file_type(filename: &str, allowed_extensions: &str) -> bool {
    if let Some(extension) = Path::new(filename).extension() {
        if let Some(ext_str) = extension.to_str() {
            let ext_lower = ext_str.to_lowercase();
            return allowed_extensions
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .any(|allowed| allowed == ext_lower);
        }
    }
    false
}

fn get_unique_filename(upload_dir: &str, original_filename: &str) -> String {
    let path = Path::new(original_filename);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    
    let mut counter = 0;
    let mut filename = original_filename.to_string();
    
    while Path::new(upload_dir).join(&filename).exists() {
        counter += 1;
        if extension.is_empty() {
            filename = format!("{}_{}", stem, counter);
        } else {
            filename = format!("{}_{}.{}", stem, counter, extension);
        }
    }
    
    filename
}

async fn upload_file(mut multipart: Multipart) -> Result<Json<Value>, StatusCode> {
    let upload_dir = "./uploads";
    let allowed_extensions = env::var("ALLOWED_FILE_TYPES").unwrap_or_else(|_| "epub,pdf,mobi,azw,azw3,txt".to_string());

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("unknown");
        
        if name == "file" {
            let filename = field
                .file_name()
                .unwrap_or("unknown")
                .to_string();
            
            // Validate file type
            if !is_allowed_file_type(&filename, &allowed_extensions) {
                warn!("Rejected file with invalid extension: {}", filename);
                return Ok(Json(json!({
                    "success": false,
                    "message": format!("File type not allowed. Allowed types: {}", allowed_extensions),
                    "allowed_types": allowed_extensions.split(',').collect::<Vec<&str>>()
                })));
            }
            
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            
            // Check file size (25MB = 25 * 1024 * 1024 bytes)
            if data.len() > 25 * 1024 * 1024 {
                warn!("Rejected file too large: {} ({} bytes)", filename, data.len());
                return Ok(Json(json!({
                    "success": false,
                    "message": "File too large. Maximum size is 25MB."
                })));
            }
            
            // Generate unique filename to avoid conflicts
            let unique_filename = get_unique_filename(upload_dir, &filename);
            let file_path = Path::new(upload_dir).join(&unique_filename);
            
            let mut file = fs::File::create(&file_path)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            file.write_all(&data)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            info!("File uploaded: {} ({} bytes)", unique_filename, data.len());
            
            return Ok(Json(json!({
                "success": true,
                "message": "File uploaded successfully",
                "filename": unique_filename,
                "size": data.len()
            })));
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}
