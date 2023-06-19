from bigint import BigInt

A = 123456789101112131415161718
B = -987654321919293949596979899

def test_add():
	assert BigInt(A) + BigInt(B) == BigInt(A + B)
def test_sub():
	assert BigInt(B) - BigInt(A) == BigInt(B - A)
def test_mul():
	assert BigInt(B) * BigInt(A) == BigInt(B * A)
def test_div():
	assert BigInt(B) // BigInt(A) == BigInt(B // A)
def test_mod():
	assert BigInt(B) % BigInt(A) == BigInt(B % A)
def test_divmod():
	print(BigInt(B), BigInt(A))
	print(divmod(BigInt(B), BigInt(A))[0], divmod(BigInt(B), BigInt(A))[1])
	print(divmod(B, A))
	a, b = divmod(B, A)
	assert divmod(BigInt(B), BigInt(A)) == (BigInt(a), BigInt(b))
def test_truediv():
	assert BigInt(A) / BigInt(B) == A / B
	assert BigInt(B) / BigInt(A) == B / A
def test_pow():
	assert BigInt(A) ** 2 == BigInt(A ** 2)
	assert BigInt(B) ** 2 == BigInt(B ** 2)

# def test_shr():
# 	assert (BigInt(A) >> 10 == BigInt(A >> 10))
# def test_shl():
# 	assert (BigInt(A) << 10 == BigInt(A << 10))

# def test_and():
# 	assert (BigInt(A) & BigInt(B)) == BigInt(A & B)
# def test_xor():
# 	assert (BigInt(A) ^ BigInt(B)) == BigInt(A ^ B)
# def test_or():
# 	assert (BigInt(A) | BigInt(B)) == BigInt(A | B)

def test_float():
	assert float(BigInt(A)) == float(A)
def test_from_float():
	assert BigInt.from_f64(float(A)) == BigInt(int(float(A)))

def test_eq():
	assert BigInt(A) + BigInt(B) == BigInt(A + B)
def test_le():
	assert BigInt(A) + BigInt(B) <= BigInt(A + B)
def test_ge():
	assert BigInt(A) + BigInt(B) >= BigInt(A + B)
def test_lt():
	assert abs(BigInt(A)) < BigInt(abs(A) + abs(B))
def test_gt():
	assert BigInt(abs(A) + abs(B)) > abs(BigInt(A))

def test_bool():
	assert bool(BigInt(1)) == bool(1)
	assert bool(BigInt(0)) == bool(0)