FROM rust:latest

RUN apt-get update && \
    apt-get install -y ffmpeg && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

    RUN cargo install yt-dlp

    WORKDIR /app
    COPY . .

    RUN cargo build --release

    CMD ["./target/release/video_downloader"]