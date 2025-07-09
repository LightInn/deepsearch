Alright. Here’s a fresh, *dense & actionable* blueprint describing your improved **CLI Rust research assistant**, taking into account all your new constraints and logic. Cynical curiosity included.

---

# 📝 **Blueprint: Modular AI-Powered Research CLI (Rust)**

## 🎯 **Project Scope & Principles**

* Single binary CLI app, Unix-friendly.
* External dependencies:

  * Local LLM (Ollama, JSON output mode, `ollama-rs` crate).
  * Search via Wikipedia API & DuckDuckGo HTML scraping.
* Orchestrates **step-by-step research**:

  * Decompose → Search → Filter → Summarize → Evaluate → Iterate → Answer.
* Prioritize **validity & coherence** of data (names, dates, etc.).
* Minimal CLI UI — no `ratatui`, just `clap`, `colored` and some Unicode flair.
* User chooses Ollama model at startup (from detected list).
* Avoid unnecessary Ollama calls — reduce context bloat & chat loops.

---

## 🪜 **Logic: Research Workflow**

```
🔍 Question → 🤔 Decompose → 🧭 Select tools → 🌐 Search → 🗂️ Filter results 
→ 📄 Scrape & Summarize → 🧠 Evaluate → 🧩 (Iterate if needed) → ✅ Answer
```

### Step-by-Step

1️⃣ **Decompose the question**

* Send main question to Ollama:

  > *"Break down this complex question into concrete, specific sub-questions needed to answer it step by step. Reply in JSON list of strings."*
* Parse JSON list.

2️⃣ **Decide per sub-question: search tool**

* For each sub-question, decide:

  * Wikipedia if clearly factual or well-defined topic.
  * DuckDuckGo otherwise.
* Query Ollama to rank tools if ambiguous (optional).

3️⃣ **Search & collect results**

* Use:

  * Wikipedia API (structured JSON)
  * DuckDuckGo (HTML scrape + parse).
* Discard irrelevant sources: closed platforms (Facebook, Twitter, etc).

4️⃣ **Filter URLs for relevance**

* Prompt Ollama:

  > *"Given these URLs and the sub-question, which ones are most relevant, trustworthy, and coherent with the question’s constraints (names, dates, etc.)? Return JSON of relevant URLs & titles."*

5️⃣ **Scrape & summarize each page**

* Fetch content.
* Parse into readable text.
* Chunk if too long.
* Summarize with Ollama (keep it scoped to sub-question & avoid overloading context).

6️⃣ **Evaluate**

* Aggregate summaries into `global_context`.
* Ask Ollama:

  > *"Based on this global context, can you answer the main question? If not, suggest refined sub-questions."*
* If yes → Output synthetic answer.
* If no → Loop with new sub-questions.

7️⃣ **Final checks**

* Ensure coherence of output (e.g., names, dates).
* Clean, concise markdown-style output.

---

## 🧩 **CLI UI**

* User selects Ollama model at startup (list fetched via `ollama-rs`).
* Command example:

  ```
  deepsearch "How did the 2008 financial crisis impact European banking regulation?" --max-depth 3 --model llama3
  ```
* Output: minimal, readable, with section headings & emojis:

  ```
  🧩 Decomposed sub-questions:
  1. …
  2. …
  …

  🔍 Search results:
  …

  📄 Summaries:
  …

  ✅ Final Answer:
  …
  ```

---

## 🗃️ **Architecture**

```
src/
├── main.rs
├── cli.rs           // clap CLI & UI output helpers
├── orchestrator.rs  // workflow logic & loop
├── ollama.rs        // ollama-rs interface & JSON parsing
├── search.rs        // Wikipedia + DuckDuckGo
├── parser.rs        // HTML scraping & text extraction
├── context.rs       // global context management & validation
```

---

## 📈 **Optimizations**

* Always request `format=json` in Ollama prompts.
* Limit Ollama input: only pass minimal context & sub-question.
* Cache Ollama calls & fetched pages if running iteratively.
* Cap number of search results & page size early.
* Avoid full conversational history — stateless calls where possible.

---

## 🪵 **MVP Milestones**

| Milestone | What to implement                    |
| --------- | ------------------------------------ |
| ✅ 1       | CLI with question + model selection  |
| ✅ 2       | Ollama JSON call: decompose question |
| ✅ 3       | Wikipedia & DDG search               |
| ✅ 4       | Ollama JSON call: filter URLs        |
| ✅ 5       | Page scrape & summary                |
| ✅ 6       | Evaluation loop                      |
| ✅ 7       | Final answer output                  |

---

## 😈 **Brutal Advice**

* Ollama context windows are brutal — keep each call scoped, and pre-trim content.
* Don’t trust DDG HTML structure to stay stable — build a resilient parser, or consider Bing if this fails.
* Wikipedia API is your friend — prefer it whenever possible.
* Be skeptical about Ollama JSON output — validate & fallback.
* Never concatenate full-page texts into Ollama input — chunk & summarize progressively.

---

## 🗂️ **Next Steps**

✅ Bootstrap project & skeleton
✅ Implement `ollama.rs` with model listing & JSON calls
✅ Implement Wikipedia search module
🔜 Build orchestrator loop step by step
🔜 Focus on per-step validation & defensive coding

---

If you want, I can also draft:

✅ a `main.rs` skeleton with CLI + orchestrator hook
✅ `ollama.rs` JSON query/response parser example
✅ example `orchestrator.rs` with a hardcoded 1-iteration loop

Say the word.
