use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use serde::Deserialize;

async fn submit_job(data: web::Data<AppState>) -> impl Responder {
    let job_id = format!("job:{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    println!("JOb id ..{:?}", job_id.clone());
    data.jobs.lock().unwrap().insert(job_id.clone(), 0);
    update_job(data.jobs.clone(), job_id.clone());
    HttpResponse::Ok().body(job_id)
}

async fn check_status(query: web::Query<JobQuery>, data: web::Data<AppState>) -> impl Responder {
    let jobs = data.jobs.lock().unwrap();
    println!("{:?}", &query.job_id);
    let progress = jobs.get(&query.job_id).unwrap_or(&-1).to_string();
    HttpResponse::Ok().body(format!("JobStatus: {}%", progress))
}

fn update_job(jobs: Arc<Mutex<HashMap<String, i32>>>, job_id: String) {
    thread::spawn(move || {
        let mut progress = 0;
        while progress < 100 {
            thread::sleep(Duration::from_secs(3));
            progress += 10;
            jobs.lock().unwrap().insert(job_id.clone(), progress);
            println!("progress... {:?}", progress)
        }
    });
}

#[derive(Clone)]
struct AppState {
    jobs: Arc<Mutex<HashMap<String, i32>>>,
}

#[derive(Deserialize)]
struct JobQuery {
    job_id: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        jobs: Arc::new(Mutex::new(HashMap::new())),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/submit", web::post().to(submit_job))
            .route("/checkstatus", web::get().to(check_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

