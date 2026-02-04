---
name: warn-time-based-logic
enabled: true
event: file
action: warn
pattern: time\.(time|sleep|perf_counter|monotonic)|std::time::(Instant|Duration)
---

⚠️ **TIME-BASED LOGIC DETECTED**

You are using time-based operations which can break determinism.

**QMNF Determinism Requirement:**
Use operation counts instead of time-based thresholds. Time varies across:
- Different CPU speeds
- Different system loads
- Different platforms

**Instead of:**
```python
import time
start = time.time()
# ... work ...
if time.time() - start > threshold:
    promote_tier()
```

**Use:**
```python
operation_count += 1
if operation_count > 2048:  # Deterministic threshold
    promote_tier()
    operation_count = 0
```

**Why:** Time-based thresholds break reproducibility. Same code, same input → different behavior.
