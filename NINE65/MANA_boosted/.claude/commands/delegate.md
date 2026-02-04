---
description: Smart delegation to appropriate local model based on task
---

Analyze the task: "{{prompt}}"

Based on the task type, call the appropriate local model:

**For code generation tasks** (keywords: implement, write, create, generate code):
```bash
ollama run deepseek-coder:33b "{{prompt}}"
```

**For mathematical reasoning** (keywords: prove, verify, calculate, theorem, proof):
```bash
ollama run qwen2.5:32b "{{prompt}}\n\nProvide rigorous mathematical reasoning."
```

**For code analysis** (keywords: analyze, review, bug, security, optimize):
```bash
ollama run gemma2:27b "Analyze this: {{prompt}}"
```

**For documentation** (keywords: document, explain, describe):
```bash
ollama run llama3.1:8b "Explain in detail: {{prompt}}"
```

If you cannot determine the task type, default to the most general model (llama3.1:8b).

After getting the response, present it to the user with a note about which model was used.
