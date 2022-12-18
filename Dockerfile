FROM rust:1.66.0-alpine3.17
WORKDIR /src
COPY . .
RUN cargo build --release

# vimagick/youtube-dl contains ffmpeg, youtube-dl and other dependencies.
FROM vimagick/youtube-dl
COPY --from=0 /app /app
ENTRYPOINT ["/app/target/release/rustyt"]
