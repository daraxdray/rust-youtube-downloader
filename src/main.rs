use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::process::Command;



#[derive(Serialize)]
struct DownloadResponse {
    status: String,
    message: String,
    error: String
}

#[derive(Deserialize)]
struct DownloadQuery {
    id: String
}

async fn download_video(url: web::Query<DownloadQuery>) -> impl Responder {
    let yt_url = format!("https://www.youtube.com/watch?v={}", url.id);
    print!("{}",yt_url);
    // Call yt-dlp command to download the video
    let output = Command::new("yt-dlp")
        .arg(&yt_url)
        .output()
        .expect("Failed to download video");

    if output.status.success() {
        HttpResponse::Ok().json(DownloadResponse {
            status: "success".into(),
            message: "Video downloaded".into(),
            error: String::from("")

        })
    } else {
        print!("{:?}",output);
        HttpResponse::InternalServerError().json(DownloadResponse {
            status: "error".into(),
            message: "Failed to download video".into(),
            error:String::from_utf8_lossy(&output.stderr).to_string()
        })
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
        .route("/",web::get().to(welcome))
        .route("/download", web::get().to(download_video))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


async fn welcome() -> impl Responder {
    "Welcome to the API!"
}