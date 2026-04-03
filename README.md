# Qarwyn 🦅

**Qarwyn** is a simple Python library that uses AI to grade essays based on your specific rules. It handles the complicated parts of talking to OpenAI and ensures you always get back clean, organized data.

---

## 🚀 Installation

You can install Qarwyn directly using `pip`:

```bash
pip install qarwyn

```
> **Note:** You will need an OPENAI_API_KEY to use the grading features.
> 
## 📖 Tutorial: How to Grade an Essay
Grading an essay takes three simple steps: Define your **Criteria**, create your **Essay**, and run the **Pipeline**.
### 1. Setup your Rubric
Decide what you are grading and how many points each section is worth.
```python
import qarwyn

# Define what you are looking for
clarity = qarwyn.Criterion("Clarity", 10.0, "How clear is the main argument?")
grammar = qarwyn.Criterion("Grammar", 5.0, "Proper use of punctuation and spelling.")

# Combine them into a Rubric (Total score: 15.0)
my_rubric = qarwyn.Rubric("Classroom Grading", 15.0, [clarity, grammar])

```
### 2. Prepare the Essay
Create the essay object with a title and the actual text content.
```python
my_essay = qarwyn.Essay(
    title="My First AI Essay",
    content="This is an essay about how AI helps us code faster.",
    author="Jane Doe"
)

```
### 3. Run the Grader
Send everything to the pipeline. Qarwyn handles the AI communication for you.
```python
# Create the payload
payload = qarwyn.Payload(
    api_key="your-api-key-here",
    model="gpt-5.4-nano-2026-03-17",
    name="grading_job_01",
    system_prompt="You are a helpful teacher. Grade the essay based on the rubric.",
    rubric=my_rubric,
    essays=[my_essay]
)

# Get the results
report = qarwyn.run_grading_pipeline(payload)

# Your results come back as a standard Python dictionary
print(report.results)

```
## 🛠 Features
 * **Reliable Data**: You always get a clean Python dictionary back, never a messy string of text.
 * **Fast & Safe**: Built with a Rust core to ensure high performance and data validation.
 * **Automatic Math**: It checks if your rubric scores add up correctly before starting the job.
## 📄 License
MIT
```
