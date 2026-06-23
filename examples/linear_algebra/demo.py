import sys
sys.path.insert(0, '.')
import linear_algebra as la
import numpy as np

# dot product
a = np.array([1.0, 2.0, 3.0])
b = np.array([4.0, 5.0, 6.0])
print(f"dot product: {la.dot_product(a, b)}")     # 32.0

# norm
print(f"norm:        {la.norm(a):.4f}")            # 3.7417

# scale
print(f"scaled:      {la.scale(a, 2.0)}")          # [2. 4. 6.]

# matrix determinant
m = la.Matrix2x2(a=1.0, b=2.0, c=3.0, d=4.0)
print(f"determinant: {la.determinant(m)}")         # -2.0
