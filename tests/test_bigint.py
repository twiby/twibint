from math import log
from twibint import BigInt, BigUint, read_from_file

A = 123456789101112131415161718
B = -987654321919293949596979899

def test_len():
	assert len(BigInt(A)) == int(log(A, 2**64)) + 1
	assert len(BigInt(B)) == int(log(-B, 2**64)) + 1
	assert len(BigInt(2**128)) == int(log(2**128, 2**64)) + 1
def test_constructor():
	assert BigInt(A) == BigInt(str(A))
	assert BigInt(A) == A
	assert BigInt(float(A)) == int(float(A))
	assert BigInt(BigInt(A)) == BigInt(A)
	assert BigInt(BigUint(A)) == BigInt(A)

def test_add():
	assert BigInt(A) + BigInt(B) == BigInt(A + B)
	assert BigInt(A) + B == BigInt(A + B)
	assert A + BigInt(B) == BigInt(A + B)
	assert float(150) + BigInt(B) == 150 + B
def test_sub():
	assert BigInt(B) - BigInt(A) == BigInt(B - A)
	assert B - BigInt(A) == BigInt(B - A)
	assert BigInt(B) - A == BigInt(B - A)
def test_mul():
	assert BigInt(B) * BigInt(A) == BigInt(B * A)
	assert B * BigInt(A) == BigInt(B * A)
	assert BigInt(B) * A == BigInt(B * A)
def test_div():
	assert BigInt(B) // BigInt(A) == BigInt(B // A)
	assert BigInt(B) // A == BigInt(B // A)
	assert B // BigInt(A) == BigInt(B // A)
def test_mod():
	assert BigInt(B) % BigInt(A) == BigInt(B % A)
	assert B % BigInt(A) == BigInt(B % A)
	assert BigInt(B) % A == BigInt(B % A)
def test_divmod():
	a, b = divmod(B, A)
	assert divmod(BigInt(B), BigInt(A)) == (BigInt(a), BigInt(b))
	assert divmod(B, BigInt(A)) == (BigInt(a), BigInt(b))
	assert divmod(BigInt(B), A) == (BigInt(a), BigInt(b))
def test_truediv():
	assert BigInt(A) / BigInt(B) == A / B
	assert BigInt(B) / BigInt(A) == B / A
	assert BigInt(B) / A == B / A
	assert B / BigInt(A) == B / A
def test_pow():
	assert BigInt(A) ** 2 == BigInt(A ** 2)
	assert BigInt(B) ** 2 == BigInt(B ** 2)

def test_shr():
	assert (BigInt(A) >> 10 == BigInt(A >> 10))
	assert (BigInt(B) >> 10 == BigInt(B >> 10))
def test_shl():
	assert (BigInt(A) << 10 == BigInt(A << 10))
	assert (BigInt(B) << 10 == BigInt(B << 10))

def test_xor():
	assert (BigInt(A) ^ BigInt(B)) == BigInt(A ^ B)
	assert (BigInt(-A) ^ BigInt(B)) == BigInt((-A) ^ B)
	assert (BigInt(A) ^ BigInt(-B)) == BigInt(A ^ (-B))
	assert (BigInt(-A) ^ BigInt(-B)) == BigInt((-A) ^ (-B))
def test_or():
	assert (BigInt(A) | BigInt(B)) == BigInt(A | B)
	assert (BigInt(-A) | BigInt(B)) == BigInt((-A) | B)
	assert (BigInt(A) | BigInt(-B)) == BigInt(A | (-B))
	assert (BigInt(-A) | BigInt(-B)) == BigInt((-A) | (-B))
def test_and():
	assert (BigInt(A) & BigInt(B)) == BigInt(A & B)
	assert (BigInt(-A) & BigInt(B)) == BigInt((-A) & B)
	assert (BigInt(A) & BigInt(-B)) == BigInt(A & (-B))
	assert (BigInt(-A) & BigInt(-B)) == BigInt((-A) & (-B))
def test_not():
	assert ~BigInt(A) == ~A
	assert ~BigInt(B) == ~B

def test_float():
	assert float(BigInt(A)) == float(A)
	assert float(BigInt(B)) == float(B)
def test_from_float():
	assert BigInt(float(A)) == BigInt(int(float(A)))
	assert BigInt(float(B)) == BigInt(int(float(B)))
def test_int():
	assert int(BigInt(A)) == A
	assert int(BigInt(B)) == B

def test_eq():
	assert BigInt(A) + BigInt(B) == BigInt(A + B)
	assert BigInt(A) + BigInt(B) == A + B
def test_le():
	assert BigInt(A) + BigInt(B) <= BigInt(A + B)
	assert BigInt(A) + BigInt(B) <= A + B
def test_ge():
	assert BigInt(A) + BigInt(B) >= BigInt(A + B)
	assert BigInt(A) + BigInt(B) >= A + B
def test_lt():
	assert abs(BigInt(A)) < BigInt(abs(A) + abs(B))
	assert abs(A) < BigInt(abs(A) + abs(B))
	assert abs(BigInt(A)) < abs(A) + abs(B)
def test_gt():
	assert BigInt(abs(A) + abs(B)) > abs(BigInt(A))
	assert BigInt(abs(A) + abs(B)) > abs(A)

def test_bool():
	assert bool(BigInt(1)) == bool(1)
	assert bool(BigInt(0)) == bool(0)

def test_read_write():
	C = BigInt(B)
	C.write_to_file("test_file_py_bigint")
	D = read_from_file("test_file_py_bigint")
	assert C == D

	C = BigInt(A)
	C.write_to_file("test_file_py_bigint")
	D = read_from_file("test_file_py_bigint")
	assert C == D
