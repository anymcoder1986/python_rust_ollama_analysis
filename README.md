# 🦀🧠 Python–Rust Comparative Analysis for Localized LLM Inference via Ollama on Raspberry Pi 4B

This repository provides a **novel comparative performance analysis framework** using both **Python** and **Rust** to interact with a **locally running LLM via Ollama API** on a **Raspberry Pi 4B**.

Both languages execute a set of 20 diverse prompts under controlled temperature and seed conditions, and log runtime and model evaluation metrics into CSV files. The goal is to analyze latency, throughput, and robustness of the RESTful API interface.

---

## 📌 Project Structure



## Project Structure

```

pythonrust/
│
├── main.rs                # Rust source (Ollama API call and CSV logging)
├── Cargo.toml             # Rust dependencies
│
├── python\_ollama\_api.py   # Python source (Ollama API call and CSV logging)
├── requirements.txt       # Python dependencies
│
├── rust\_ollama\_log.csv    # Output: Rust logs (CSV)
├── python\_ollama\_log.csv  # Output: Python logs (CSV)
│
└── ... (virtualenv, target/, src/, etc.)

````

---

## Motivation & Research Value

- **Why compare?**  
  Python is the default language for AI research, but Rust offers superior performance and safety for edge devices like the Raspberry Pi 4B.
- **What is measured?**  
  This repo enables direct, reproducible comparison of request latency, throughput, and output quality when using Rust vs Python for the exact same LLM inference tasks.
- **Applications**  
  Useful for research in edge AI, benchmarking, reproducible science, and deployment choices for LLM-powered IoT.

---

## Hardware/Software Requirements

- **Raspberry Pi 4B** (4GB/8GB recommended)
- **Ollama** running locally (`ollama serve`)
- **Python 3.8+** (tested with 3.9+)
- **Rust 1.73+**
- **Virtual environment recommended for Python**

---

## Setup Instructions

### 1. Clone and Enter the Project Folder

```sh
git clone <your_repo_url>
cd pythonrust
````

### 2. Setup Rust Environment

```sh
# If not installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cd pythonrust
cargo build
```

### 3. Setup Python Environment

```sh
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

The **requirements.txt** contains:

```python
requests
```

### 4. Start Ollama Server (in another terminal)

```sh
ollama serve
# Or ensure Ollama is running on http://localhost:11434/
```

### 5. Run Rust and Python Experiments

**Rust:**

```sh
cargo run
```

**Python:**

```sh
python python_ollama_api.py
```

*(Replace with the actual script filename as needed)*

---

**requirements.txt**
```sh
requests
````

**Cargo.toml**

```toml
[package]
name = "rustpython"
version = "0.1.0"
edition = "2024"

[dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
````
## Output

Each run produces a detailed `.csv` log:

* `rust_ollama_log.csv`
* `python_ollama_log.csv`

Both contain:

| timestamp | model | prompt | temperature | seed | response | total\_duration | load\_duration | prompt\_eval\_count | prompt\_eval\_duration | eval\_count | eval\_duration | tokens\_per\_sec |
| --------- | ----- | ------ | ----------- | ---- | -------- | --------------- | -------------- | ------------------- | ---------------------- | ----------- | -------------- | ---------------- |

---

## Example Comparative Analysis

With both CSVs, you can:

* Compare average response latency, throughput, or token/sec between Rust and Python.
* Analyze output accuracy and model consistency for the same prompt and temperature.
* Identify network/API bottlenecks or error frequency.

*You can use pandas, Excel, or your favorite tool for plotting and statistical analysis.*

---

## Notes & Best Practices

* Always run both clients against the same Ollama server and model version for a fair comparison.
* Add new prompt pairs as needed for your study.
* Adjust timeouts (`600s` by default) if using larger models.

---





