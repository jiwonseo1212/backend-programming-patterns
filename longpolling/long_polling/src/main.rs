
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use serde::Deserialize;
use tokio::time::{sleep, Duration};


async fn submit_job(data: web::Data<AppState>) -> impl Responder {
    let job_id = format!("job:{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    println!("JOb id ..{:?}", job_id.clone());
    data.jobs.lock().unwrap().insert(job_id.clone(), 0);
    tokio::spawn(update_job(data.jobs.clone(), job_id.clone()));
    HttpResponse::Ok().body(job_id)
}


async fn update_job(jobs: Arc<Mutex<HashMap<String, i32>>>, job_id: String) {
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


async fn check_status(query: web::Query<JobQuery>, data: web::Data<AppState>) -> impl Responder {
    loop {
        {let jobs = data.jobs.lock().unwrap();
        println!("{:?}", &query.job_id);
        
        if let progress = jobs.get(&query.job_id).unwrap_or(&-1) {
        
        if progress >= &100 {
            return HttpResponse::Ok().body("Job is done".to_string());
            }
        }
        
    
        else{
            HttpResponse::NotFound().finish();
            }
        }
        sleep(Duration::from_secs(1)).await;
    }
}
#[derive(Clone)]
struct AppState {
    jobs: Arc<Mutex<HashMap<String, i32>>>,
}

#[derive(Deserialize)]
struct JobQuery {
    job_id: String
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        jobs: Arc::new(Mutex::new(HashMap::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/submit", web::post().to(submit_job))
            .route("/checkstatus", web::get().to(check_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}








