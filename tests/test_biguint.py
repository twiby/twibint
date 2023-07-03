from math import log
from bigint import BigInt, BigUint

A = 123456789101112131415161718
B = 987654321919293949596979899

def test_len():
	assert len(BigUint(A)) == int(log(A, 2**32)) + 1
	assert len(BigUint(B)) == int(log(B, 2**32)) + 1
	assert len(BigUint(2**64)) == int(log(2**64, 2**32)) + 1
def test_constructor():
	assert BigUint(A) == BigUint(str(A))
	assert BigUint(A) == A
	assert BigUint(float(A)) == int(float(A))
	assert BigUint(BigInt(A)) == BigUint(A)
	assert BigUint(BigUint(A)) == BigUint(A)

def test_add():
	assert BigUint(A) + BigUint(B) == BigUint(A + B)
	assert BigUint(A) + B == BigUint(A + B)
	assert A + BigUint(B) == BigUint(A + B)
	assert str(A) + BigUint(B) == BigUint(A + B)
	assert float(150) + BigUint(B) == 150 + B
def test_sub():
	assert BigUint(B) - BigUint(A) == BigUint(B - A)
	assert BigUint(B) - A == BigUint(B - A)
	assert B - BigUint(A) == BigUint(B - A)
def test_mul():
	assert BigUint(B) * BigUint(A) == BigUint(B * A)
	assert B * BigUint(A) == BigUint(B * A)
	assert BigUint(B) * A == BigUint(B * A)
def test_div():
	assert BigUint(B) // BigUint(A) == BigUint(B // A)
	assert B // BigUint(A) == BigUint(B // A)
	assert BigUint(B) // A == BigUint(B // A)
def test_mod():
	assert BigUint(B) % BigUint(A) == BigUint(B % A)
	assert B % BigUint(A) == BigUint(B % A)
	assert BigUint(B) % A == BigUint(B % A)
def test_divmod():
	a, b = divmod(B, A)
	assert divmod(BigUint(B), BigUint(A)) == (BigUint(a), BigUint(b))
	assert divmod(B, BigUint(A)) == (BigUint(a), BigUint(b))
	assert divmod(BigUint(B), A) == (BigUint(a), BigUint(b))
def test_truediv():
	assert BigUint(A) / BigUint(B) == A / B
	assert BigUint(B) / BigUint(A) == B / A
	assert B / BigUint(A) == B / A
	assert BigUint(B) / A == B / A
def test_pow():
	assert BigUint(A) ** 2 == BigUint(A ** 2)
	assert BigUint(B) ** 2 == BigUint(B ** 2)

def test_shr():
	assert (BigUint(A) >> 10 == BigUint(A >> 10))
def test_shl():
	assert (BigUint(A) << 10 == BigUint(A << 10))

def test_and():
	assert (BigUint(A) & BigUint(B)) == BigUint(A & B)
	assert (A & BigUint(B)) == BigUint(A & B)
	assert (BigUint(A) & B) == BigUint(A & B)
def test_xor():
	assert (BigUint(A) ^ BigUint(B)) == BigUint(A ^ B)
	assert (A ^ BigUint(B)) == BigUint(A ^ B)
	assert (BigUint(A) ^ B) == BigUint(A ^ B)
def test_or():
	assert (BigUint(A) | BigUint(B)) == BigUint(A | B)
	assert (A | BigUint(B)) == BigUint(A | B)
	assert (BigUint(A) | B) == BigUint(A | B)

def test_float():
	assert float(BigUint(A)) == float(A)
def test_from_float():
	assert BigUint(float(A)) == BigUint(int(float(A)))

def test_eq():
	assert BigUint(A) + BigUint(B) == BigUint(A + B)
	assert BigUint(A) + BigUint(B) == A + B
def test_le():
	assert BigUint(A) + BigUint(B) <= BigUint(A + B)
	assert BigUint(A) + BigUint(B) <= A + B
def test_ge():
	assert BigUint(A) + BigUint(B) >= BigUint(A + B)
	assert BigUint(A) + BigUint(B) >= A + B
def test_lt():
	assert BigUint(A) < BigUint(A + B)
	assert A < BigUint(A + B)
	assert BigUint(A) < A + B
def test_gt():
	assert BigUint(A + B) > BigUint(A)
	assert BigUint(A + B) > A

def test_bool():
	assert bool(BigUint(1)) == bool(1)
	assert bool(BigUint(0)) == bool(0)