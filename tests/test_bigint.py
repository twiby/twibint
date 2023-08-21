from math import log
from twibint import PyBigInt, PyBigUint

A = 123456789101112131415161718
B = -987654321919293949596979899

def test_len():
	assert len(PyBigInt(A)) == int(log(A, 2**64)) + 1
	assert len(PyBigInt(B)) == int(log(-B, 2**64)) + 1
	assert len(PyBigInt(2**128)) == int(log(2**128, 2**64)) + 1
def test_constructor():
	assert PyBigInt(A) == PyBigInt(str(A))
	assert PyBigInt(A) == A
	assert PyBigInt(float(A)) == int(float(A))
	assert PyBigInt(PyBigInt(A)) == PyBigInt(A)
	assert PyBigInt(PyBigUint(A)) == PyBigInt(A)

def test_add():
	assert PyBigInt(A) + PyBigInt(B) == PyBigInt(A + B)
	assert PyBigInt(A) + B == PyBigInt(A + B)
	assert A + PyBigInt(B) == PyBigInt(A + B)
	assert float(150) + PyBigInt(B) == 150 + B
def test_sub():
	assert PyBigInt(B) - PyBigInt(A) == PyBigInt(B - A)
	assert B - PyBigInt(A) == PyBigInt(B - A)
	assert PyBigInt(B) - A == PyBigInt(B - A)
def test_mul():
	assert PyBigInt(B) * PyBigInt(A) == PyBigInt(B * A)
	assert B * PyBigInt(A) == PyBigInt(B * A)
	assert PyBigInt(B) * A == PyBigInt(B * A)
def test_div():
	assert PyBigInt(B) // PyBigInt(A) == PyBigInt(B // A)
	assert PyBigInt(B) // A == PyBigInt(B // A)
	assert B // PyBigInt(A) == PyBigInt(B // A)
def test_mod():
	assert PyBigInt(B) % PyBigInt(A) == PyBigInt(B % A)
	assert B % PyBigInt(A) == PyBigInt(B % A)
	assert PyBigInt(B) % A == PyBigInt(B % A)
def test_divmod():
	a, b = divmod(B, A)
	assert divmod(PyBigInt(B), PyBigInt(A)) == (PyBigInt(a), PyBigInt(b))
	assert divmod(B, PyBigInt(A)) == (PyBigInt(a), PyBigInt(b))
	assert divmod(PyBigInt(B), A) == (PyBigInt(a), PyBigInt(b))
def test_truediv():
	assert PyBigInt(A) / PyBigInt(B) == A / B
	assert PyBigInt(B) / PyBigInt(A) == B / A
	assert PyBigInt(B) / A == B / A
	assert B / PyBigInt(A) == B / A
def test_pow():
	assert PyBigInt(A) ** 2 == PyBigInt(A ** 2)
	assert PyBigInt(B) ** 2 == PyBigInt(B ** 2)

def test_shr():
	assert (PyBigInt(A) >> 10 == PyBigInt(A >> 10))
	assert (PyBigInt(B) >> 10 == PyBigInt(B >> 10))
def test_shl():
	assert (PyBigInt(A) << 10 == PyBigInt(A << 10))
	assert (PyBigInt(B) << 10 == PyBigInt(B << 10))

def test_xor():
	assert (PyBigInt(A) ^ PyBigInt(B)) == PyBigInt(A ^ B)
	assert (PyBigInt(-A) ^ PyBigInt(B)) == PyBigInt((-A) ^ B)
	assert (PyBigInt(A) ^ PyBigInt(-B)) == PyBigInt(A ^ (-B))
	assert (PyBigInt(-A) ^ PyBigInt(-B)) == PyBigInt((-A) ^ (-B))
def test_or():
	assert (PyBigInt(A) | PyBigInt(B)) == PyBigInt(A | B)
	assert (PyBigInt(-A) | PyBigInt(B)) == PyBigInt((-A) | B)
	assert (PyBigInt(A) | PyBigInt(-B)) == PyBigInt(A | (-B))
	assert (PyBigInt(-A) | PyBigInt(-B)) == PyBigInt((-A) | (-B))
def test_and():
	assert (PyBigInt(A) & PyBigInt(B)) == PyBigInt(A & B)
	assert (PyBigInt(-A) & PyBigInt(B)) == PyBigInt((-A) & B)
	assert (PyBigInt(A) & PyBigInt(-B)) == PyBigInt(A & (-B))
	assert (PyBigInt(-A) & PyBigInt(-B)) == PyBigInt((-A) & (-B))
def test_not():
	assert ~PyBigInt(A) == ~A
	assert ~PyBigInt(B) == ~B

def test_float():
	assert float(PyBigInt(A)) == float(A)
	assert float(PyBigInt(B)) == float(B)
def test_from_float():
	assert PyBigInt(float(A)) == PyBigInt(int(float(A)))
	assert PyBigInt(float(B)) == PyBigInt(int(float(B)))
def test_int():
	assert int(PyBigInt(A)) == A
	assert int(PyBigInt(B)) == B

def test_eq():
	assert PyBigInt(A) + PyBigInt(B) == PyBigInt(A + B)
	assert PyBigInt(A) + PyBigInt(B) == A + B
def test_le():
	assert PyBigInt(A) + PyBigInt(B) <= PyBigInt(A + B)
	assert PyBigInt(A) + PyBigInt(B) <= A + B
def test_ge():
	assert PyBigInt(A) + PyBigInt(B) >= PyBigInt(A + B)
	assert PyBigInt(A) + PyBigInt(B) >= A + B
def test_lt():
	assert abs(PyBigInt(A)) < PyBigInt(abs(A) + abs(B))
	assert abs(A) < PyBigInt(abs(A) + abs(B))
	assert abs(PyBigInt(A)) < abs(A) + abs(B)
def test_gt():
	assert PyBigInt(abs(A) + abs(B)) > abs(PyBigInt(A))
	assert PyBigInt(abs(A) + abs(B)) > abs(A)

def test_bool():
	assert bool(PyBigInt(1)) == bool(1)
	assert bool(PyBigInt(0)) == bool(0)