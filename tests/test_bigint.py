from math import log
from bigint import BigInt, BigUint

A = 123456789101112131415161718
B = -987654321919293949596979899

def test_len():
	assert len(BigInt(A)) == int(log(A, 2**32)) + 1
	assert len(BigInt(B)) == int(log(-B, 2**32)) + 1
	assert len(BigInt(2**64)) == int(log(2**64, 2**32)) + 1
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

# def test_shr():
# 	assert (BigInt(A) >> 10 == BigInt(A >> 10))
# def test_shl():
# 	assert (BigInt(A) << 10 == BigInt(A << 10))

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
def test_from_float():
	assert BigInt(float(A)) == BigInt(int(float(A)))

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