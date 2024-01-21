import uvicorn
import threading
import time
from pydantic import BaseModel
from fastapi import FastAPI


app = FastAPI()

jobs = {}

class JobStatus(BaseModel):
    status: str


def update_job(job_id, progress):
    jobs[job_id] = progress
    print(f"updated {job_id} to {progress}")
    if progress < 100:
        threading.Timer(3, update_job, args=(job_id, progress + 10)).start()

@app.post("/submit")
def submit_job():
    job_id = f"job:{int(time.time() * 1000)}"
    jobs[job_id] = 0
    update_job(job_id, 0)
    return {"job_id": job_id}

@app.get("/checkstatus", response_model=JobStatus)
def check_status(jobId: str):
    progress = jobs.get(jobId, "No such job")
    return {"status": f"{progress}%"}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)

