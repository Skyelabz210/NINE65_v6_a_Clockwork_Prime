---
name: block-math-module
enabled: true
event: file
action: block
pattern: math\.(sqrt|sin|cos|tan|log|exp|pow|floor|ceil|pi|e)
---

🚫 **MATH MODULE FUNCTION DETECTED - BLOCKED**

You are attempting to use Python's `math` module which internally uses floating-point.

**QMNF Integer-Only Mandate:**
Standard library math functions are forbidden. They produce inexact results.

**Instead of:**
```python
import math
x = math.sqrt(2)
y = math.pi
```

**Use:**
```python
from qmnf.api import QMNFRational
x = QMNFRational(2, 1).sqrt()  # Exact or error-bounded
y = QMNFRational.pi()          # Exact rational approximation
```

**Why:** `math.sqrt(2)` returns 1.4142135623730951 (truncated). QMNF gives exact representation.
