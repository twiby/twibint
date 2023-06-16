import bigint as bi

A = 123456789101112131415161718
B = 987654321919293949596979899

def test_add():
	assert bi.BigUint(A) + bi.BigUint(B) == bi.BigUint(A + B)
def test_eq():
	assert bi.BigUint(A) + bi.BigUint(B) == bi.BigUint(A + B)
def test_le():
	assert bi.BigUint(A) + bi.BigUint(B) <= bi.BigUint(A + B)
def test_ge():
	assert bi.BigUint(A) + bi.BigUint(B) >= bi.BigUint(A + B)
def test_lt():
	assert bi.BigUint(A) < bi.BigUint(A + B)
def test_gt():
	assert bi.BigUint(A + B) > bi.BigUint(A)
def test_bool():
	assert bool(bi.BigUint(1)) == bool(1)
	assert bool(bi.BigUint(0)) == bool(0)