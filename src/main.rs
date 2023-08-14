use axum::{extract::Path, extract::State, http::header::HeaderMap, routing::get, Router};
//use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::*;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct AppState {
    cpu_percent: f64,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState { cpu_percent: 0_f64 }));
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route(
            "/set_cpu_percent/:percent",
            get(set_cpu_percent).with_state(state.clone()),
        );

    let cpus = num_cpus::get();
    for _ in 0..cpus {
        let state = state.clone();
        thread::spawn(move || loop {
            let run_time = {
                let state = state.lock().unwrap();
                1000_f64 * state.cpu_percent / 100_f64
            };
            let now = Instant::now();
            while now.elapsed().as_millis() < run_time as u128 {}
            let sleep_time = 1000 - run_time as u64;
            thread::sleep(Duration::from_millis(sleep_time as u64));
        });
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn set_cpu_percent(
    Path(percent): Path<f64>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> String {
    let mut state = state.lock().unwrap();
    state.cpu_percent = percent;
    println!("Set cpu usage percentage to {}", percent);
    format!("Set cpu usage percentage to {}", percent)
}

async fn root(headers: HeaderMap) -> String {
    format!(
        "Hostname: {:?}\nHeaders:\n{:?}",
        nix::unistd::gethostname().unwrap(),
        headers
    )
}
