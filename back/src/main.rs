use axum::{
    Json, Router,
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::Serialize;
use std::fs;
use tower_http::services::ServeDir;

#[derive(Serialize, Clone)]
struct PostMetadata {
    slug: String,
    title: String,
}

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new("../dist").not_found_service(get(index_fallback));

    let app = Router::new()
        .fallback_service(static_files)
        .route("/api/posts", get(list_posts))
        .route("/api/post/{slug}", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Erro bind port");

    axum::serve(listener, app).await.unwrap();
}

async fn list_posts() -> Json<Vec<PostMetadata>> {
    let paths = std::fs::read_dir("../manuscritos").expect("falha ao ler diretorio manuscritos");

    let posts = paths
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.extension()? == "md" {
                let slug = path.file_stem()?.to_str()?.to_string();
                let title = slug
                    .split("-")
                    .map(|s| {
                        let mut chars = s.chars();
                        match chars.next() {
                            Some(first) => {
                                first.to_uppercase().collect::<String>() + chars.as_str()
                            }
                            None => String::new(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                Some(PostMetadata { slug, title })
            } else {
                None
            }
        })
        .collect();
    Json(posts)
}

async fn get_post(Path(slug): Path<String>) -> Json<String> {
    let path = format!("../manuscritos/{}.md", slug);
    match fs::read_to_string(path) {
        Ok(content) => Json(content),
        Err(_) => Json("Erro ao ler artigo".into()),
    }
}

async fn index_fallback() -> impl IntoResponse {
    match tokio::fs::read_to_string("../dist/index.html").await {
        Ok(html) => Html(html).into_response(),
        Err(_) => (
            axum::http::StatusCode::NOT_FOUND,
            "PAgina nao econtrada nengue",
        )
            .into_response(),
    }
}
