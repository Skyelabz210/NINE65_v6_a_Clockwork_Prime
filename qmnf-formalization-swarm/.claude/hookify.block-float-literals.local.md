---
name: block-float-literals
enabled: true
event: file
action: block
pattern: (?<![a-zA-Z_])\d+\.\d+(?![a-zA-Z_])
---

🚫 **FLOAT LITERAL DETECTED - BLOCKED**

You are attempting to use a floating-point literal (e.g., `3.14159`).

**QMNF Integer-Only Mandate:**
All computations must use exact integer/rational arithmetic. Float literals are forbidden.

**Instead of:**
```python
x = 3.14159
y = 2.5
```

**Use:**
```python
from qmnf.api import QMNFRational
x = QMNFRational(314159, 100000)
y = QMNFRational(5, 2)
```

**Why:** Every float operation is a precision loss. At scale, this compounds into garbage.
