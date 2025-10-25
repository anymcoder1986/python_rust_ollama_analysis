# Python code Ollama API analysis
# 30 June, 2025

import requests
import datetime
import csv
import os
import time

OLLAMA_URL = "http://localhost:11434/api/generate"
MODEL_NAME = "qwen2.5:0.5b" # llama3.2:1b, gemma3:1b, granite3.1-moe:1b, qwen2.5:0.5b             

SEED = 42
CSV_FILE = "python_ollama_log.csv"

PROMPTS = [
    ("Name any one river in India.", 0.2),
    ("Who wrote the Indian National Anthem?", 0.2),
    ("Translate 'peace' to French.", 0.3),
    ("Suggest a healthy snack for children.", 0.7),
    ("Write a Python function to add two numbers.", 0.5),
    ("Summarize the water cycle in one sentence.", 0.4),
    ("What is the capital of Canada?", 0.2),
    ("Name a fruit that is yellow.", 0.3),
    ("Who is known as the father of computers?", 0.2),
    ("Give me a random English word.", 0.8),
    ("What is the square root of 144?", 0.2),
    ("Suggest a nickname for a friendly dog.", 0.8),
    ("Explain gravity to a child.", 0.5),
    ("Which planet is called the Red Planet?", 0.2),
    ("List any one prime number between 10 and 20.", 0.2),
    ("What comes next in the sequence: 2, 4, 8, 16, ...?", 0.3),
    ("Translate 'thank you' to Spanish.", 0.3),
    ("Tell me a short joke.", 0.9),
    ("Who is the current UN Secretary-General?", 0.2),
    ("Complete: To be, or not to be, ...", 0.4),
]

def log_to_csv(data):
    file_exists = os.path.isfile(CSV_FILE)
    with open(CSV_FILE, "a", newline="", encoding="utf-8") as csvfile:
        writer = csv.writer(csvfile)
        if not file_exists:
            writer.writerow([
                "timestamp", "model", "prompt", "temperature", "seed", "response",
                "total_duration", "load_duration", "prompt_eval_count",
                "prompt_eval_duration", "eval_count", "eval_duration", "tokens_per_sec"
            ])
        writer.writerow(data)

for prompt, temp in PROMPTS:
    payload = {
        "model": MODEL_NAME,
        "prompt": prompt,
        "stream": False,
        "options": {
            "temperature": temp,
            "seed": SEED
        }
    }
    try:
        t_start = time.time()
        response = requests.post(OLLAMA_URL, json=payload, timeout=600)
        elapsed = time.time() - t_start
        data = response.json()
        eval_count = data.get('eval_count', 0) or 0
        eval_duration = data.get('eval_duration', 1) or 1
        tokens_per_sec = float(eval_count) / float(eval_duration) * 1e9 if eval_duration > 0 else 0.0
        log_to_csv([
            datetime.datetime.now().isoformat(),
            data.get('model', ''),
            prompt,
            temp,
            SEED,
            data.get('response', '').replace(",", " ").replace("\n", " "),
            data.get('total_duration', 0),
            data.get('load_duration', 0),
            data.get('prompt_eval_count', 0),
            data.get('prompt_eval_duration', 0),
            eval_count,
            eval_duration,
            f"{tokens_per_sec:.2f}"
        ])
        print(f"[API] Prompt: {prompt}\nResponse: {data.get('response','').strip()}\n---")
    except Exception as e:
        print(f"[API] Error with prompt '{prompt}': {e}")
    time.sleep(2)
