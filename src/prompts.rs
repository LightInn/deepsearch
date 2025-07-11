pub fn filter_search_results_prompt(sub_question: &str, results_str: &str, json_example: &str) -> String {
    format!(r#"You are a search result filter. Your task is to identify relevant search results for a given topic. Below is the topic and a list of search results. Each search result is formatted as 'Title: [title] URL: [url]'.

Topic: '{0}'

Search Results:
{1}

Respond with a JSON object containing a key 'results' which is an array of objects, where each object has 'title' and 'url' keys.
Example:
{{
  "results": [
    {{
      "title": "Example Example",
      "url": "https://example.com"
    }}
  ]
}}"#
, sub_question, results_str)
}

pub fn summarize_text_prompt(sub_question: &str, text: &str) -> String {
    format!(r#"Summarize the following text in relation to the question: '{0}'.

Text:
{1}"#
, sub_question, text)
}

pub fn evaluate_completeness_and_answer_prompt(main_question: &str, global_summary: &str) -> String {
    format!(r#"Synthesize the information in the research summary to directly answer the following question: '{0}'.

Research Summary:
{1}"#
, main_question, global_summary)
}

pub fn decide_search_tool_prompt(sub_question: &str) -> String {
    format!(r#"Given the sub-question: '{0}', decide whether to use Wikipedia or DuckDuckGo. Use Wikipedia for factual, well-defined topics (e.g., historical events, scientific concepts, biographies). Use DuckDuckGo for broader, more current, less structured, or time-sensitive queries (e.g., 'latest news', 'current events', 'how-to guides', 'opinions', 'troubleshooting'). Respond with 'wikipedia' or 'duckduckgo'."#
, sub_question)
}

pub fn decompose_question_prompt_initial(question: &str) -> String {
    format!(r#"Break down this complex question into concrete, specific sub-questions needed to answer it step by step. Reply in JSON list of strings.

Question: '{0}'"#
, question)
}

pub fn decompose_question_prompt_iterative(main_question: &str, context: &str) -> String {
    format!(r#"You are a meticulous research assistant. Based on the main question and the research context provided below, generate the next single, specific sub-question to continue the research.
Pay strict attention to the details provided in the main question and context, such as dates, names, and locations. Do not invent or change these details.
The sub-question should be a focused query suitable for a web search.
Respond with a JSON object containing a single key "question".

Main Question: '{0}'

Research Context:
{1}"#
, main_question, context)
}

pub fn check_if_answer_is_complete_prompt(global_summary: &str, main_question: &str) -> String {
    format!(r#"Based on the research summary so far:
{0}

Can you now provide a comprehensive answer to the main question: '{1}'? Respond with a JSON object containing a single key "decision" with value "yes" or "no"."#
, global_summary, main_question)
}