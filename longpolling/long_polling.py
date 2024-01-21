import uvicorn
import threading
import time
from pydantic import BaseModel
from fastapi import FastAPI , Request
import asyncio



app = FastAPI()
jobs ={}

class JobStatus(BaseModel):
    status: str


async def update_job(job_id, progress):
    jobs[job_id] = progress
    print(f"updated {job_id} to {progress}")
    if progress < 100:
        for i in range(10):
            jobs[job_id] += 10
            print(f"progress --> {jobs[job_id]}")
            await asyncio.sleep(3)


@app.post("/submit")
async def submit_job():
    job_id = f"job:{int(time.time() * 1000)}"
    jobs[job_id] = 0  
    asyncio.create_task(update_job(job_id, 0))
    return {"job_id": job_id}


@app.get("/checkstatus", response_model=JobStatus)
async def check_status(request: Request):
    jobId =  request.query_params.get('jobId')
    print(f"checking status job_id... {jobId}")

    job_id = jobs.get(jobId, None)
    if not job_id:
        return {"status": "no such id"}
    while not await check_is_done(jobId):
        await asyncio.sleep(1)
    return {"status": "job is done"}

async def check_is_done(job_id):
    if jobs[job_id] < 100:
        return False
    else:
        return True

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)
