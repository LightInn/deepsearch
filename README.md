# Deep Search CLI

**An AI-powered research assistant for your terminal.**

Deep Search is a command-line tool that uses local large language models (LLMs) to provide in-depth answers to complex questions. It breaks down your query, scours the web for relevant information, and synthesizes a comprehensive response, all within your terminal.

## Features

*   **AI-Powered Research:** Leverages local LLMs (via Ollama) to understand and research your questions.
*   **Step-by-Step Process:** Decomposes questions, searches multiple sources (Wikipedia, DuckDuckGo), filters for relevance, and summarizes findings.
*   **Local First:** Works with your own Ollama-hosted models, keeping your data private.
*   **Minimalist CLI:** A clean, focused interface for your research tasks.

## How It Works

The tool follows a structured research workflow:

```
         +---------------------+
         |   User Question     |
         +----------+----------+
                    |
                    v
+-------------------+-------------------+
|  Decompose into Sub-questions (LLM)   |
+-------------------+-------------------+
                    |
                    v
+-------------------+-------------------+
|      Search (Wikipedia / DuckDuckGo)  |
+-------------------+-------------------+
                    |
                    v
+-------------------+-------------------+
|   Filter Results (LLM)                |
+-------------------+-------------------+
                    |
                    v
+-------------------+-------------------+
|      Summarize Content (LLM)          |
+-------------------+-------------------+
                    |
                    v
+-------------------+-------------------+
|      Evaluate & Synthesize (LLM)      |
+-------------------+-------------------+
                    |
+-------------------+-------------------+
|  Final Answer     |
+-------------------+
```

1.  **Decompose:** The initial question is broken down into smaller, specific sub-questions.
2.  **Search:** Each sub-question is researched using Wikipedia or DuckDuckGo.
3.  **Filter:** The search results are filtered to identify the most relevant sources.
4.  **Summarize:** The content of each relevant page is summarized.
5.  **Evaluate:** The summaries are used to construct a final answer. If the answer is incomplete, the process can be iterated with new sub-questions.
6.  **Answer:** A final, synthesized answer is presented to the user.

## Installation

1.  **Install Rust:** If you don't have Rust, install it from [rust-lang.org](https://www.rust-lang.org/).
2.  **Install Ollama:** You need a running Ollama instance. See the [Ollama website](https://ollama.ai/) for installation instructions.
3.  **Clone the repository:**
    ```bash
    git clone https://github.com/LightInn/deepsearch.git
    cd deepsearch
    ```
4.  **Build the project:**
    ```bash
    cargo build --release
    ```
5.  **Run the tool:**
    ```bash
    ./target/release/deepsearch "Your research question"
    ```

## Development

To build and run the project in development mode:

```bash
cargo run -- "Your research question"
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.
