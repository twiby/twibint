from bigint import BigUint

A = 123456789101112131415161718
B = 987654321919293949596979899

def test_add():
	assert BigUint(A) + BigUint(B) == BigUint(A + B)
def test_sub():
	assert BigUint(B) - BigUint(A) == BigUint(B - A)
def test_mul():
	assert BigUint(B) * BigUint(A) == BigUint(B * A)
def test_div():
	assert BigUint(B) // BigUint(A) == BigUint(B // A)
def test_mod():
	assert BigUint(B) % BigUint(A) == BigUint(B % A)
def test_divmod():
	a, b = divmod(B, A)
	assert divmod(BigUint(B), BigUint(A)) == (BigUint(a), BigUint(b))

def test_shr():
	assert (BigUint(A) >> 10 == BigUint(A >> 10))
def test_shl():
	assert (BigUint(A) << 10 == BigUint(A << 10))

def test_and():
	assert (BigUint(A) & BigUint(B)) == BigUint(A & B)
def test_xor():
	assert (BigUint(A) ^ BigUint(B)) == BigUint(A ^ B)
def test_or():
	assert (BigUint(A) | BigUint(B)) == BigUint(A | B)

def test_float():
	assert float(BigUint(A)) == float(A)
def test_from_float():
	assert BigUint.from_f64(float(A)) == BigUint(int(float(A)))

def test_eq():
	assert BigUint(A) + BigUint(B) == BigUint(A + B)
def test_le():
	assert BigUint(A) + BigUint(B) <= BigUint(A + B)
def test_ge():
	assert BigUint(A) + BigUint(B) >= BigUint(A + B)
def test_lt():
	assert BigUint(A) < BigUint(A + B)
def test_gt():
	assert BigUint(A + B) > BigUint(A)

def test_bool():
	assert bool(BigUint(1)) == bool(1)
	assert bool(BigUint(0)) == bool(0)