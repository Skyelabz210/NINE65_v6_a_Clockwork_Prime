---
name: block-numpy-floats
enabled: true
event: file
action: block
pattern: np\.(float|float32|float64|array\s*\(\s*\[.*\d+\.\d+)
---

🚫 **NUMPY FLOAT OPERATION DETECTED - BLOCKED**

You are attempting to use NumPy with floating-point types or float arrays.

**QMNF Integer-Only Mandate:**
Float arrays violate the integer-only requirement. Use integer tensors via hcvlang.

**Instead of:**
```python
import numpy as np
arr = np.array([1.5, 2.5, 3.5])
x = np.float64(3.14)
```

**Use:**
```python
import hcvlang
# Use integer tensors or QMNFRational arrays
arr = [QMNFRational(3, 2), QMNFRational(5, 2), QMNFRational(7, 2)]
```

**Why:** NumPy float operations accumulate error. QMNF provides exact integer arithmetic.
