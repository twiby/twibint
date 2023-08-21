from math import log
from twibint import PyBigInt, PyBigUint, gen_random_pybiguint

A = 123456789101112131415161718
B = 987654321919293949596979899

def test_len():
	assert len(PyBigUint(A)) == int(log(A, 2**64)) + 1
	assert len(PyBigUint(B)) == int(log(B, 2**64)) + 1
	assert len(PyBigUint(2**128)) == int(log(2**128, 2**64)) + 1
def test_constructor():
	assert PyBigUint(A) == PyBigUint(str(A))
	assert PyBigUint(A) == A
	assert PyBigUint(float(A)) == int(float(A))
	assert PyBigUint(PyBigInt(A)) == PyBigUint(A)
	assert PyBigUint(PyBigUint(A)) == PyBigUint(A)

def test_add():
	assert PyBigUint(A) + PyBigUint(B) == PyBigUint(A + B)
	assert PyBigUint(A) + B == PyBigUint(A + B)
	assert A + PyBigUint(B) == PyBigUint(A + B)
	assert str(A) + PyBigUint(B) == PyBigUint(A + B)
	assert float(150) + PyBigUint(B) == 150 + B
def test_sub():
	assert PyBigUint(B) - PyBigUint(A) == PyBigUint(B - A)
	assert PyBigUint(B) - A == PyBigUint(B - A)
	assert B - PyBigUint(A) == PyBigUint(B - A)
def test_mul():
	assert PyBigUint(B) * PyBigUint(A) == PyBigUint(B * A)
	assert B * PyBigUint(A) == PyBigUint(B * A)
	assert PyBigUint(B) * A == PyBigUint(B * A)
def test_div():
	assert PyBigUint(B) // PyBigUint(A) == PyBigUint(B // A)
	assert B // PyBigUint(A) == PyBigUint(B // A)
	assert PyBigUint(B) // A == PyBigUint(B // A)
def test_mod():
	assert PyBigUint(B) % PyBigUint(A) == PyBigUint(B % A)
	assert B % PyBigUint(A) == PyBigUint(B % A)
	assert PyBigUint(B) % A == PyBigUint(B % A)
def test_divmod():
	a, b = divmod(B, A)
	assert divmod(PyBigUint(B), PyBigUint(A)) == (PyBigUint(a), PyBigUint(b))
	assert divmod(B, PyBigUint(A)) == (PyBigUint(a), PyBigUint(b))
	assert divmod(PyBigUint(B), A) == (PyBigUint(a), PyBigUint(b))
def test_truediv():
	assert PyBigUint(A) / PyBigUint(B) == A / B
	assert PyBigUint(B) / PyBigUint(A) == B / A
	assert B / PyBigUint(A) == B / A
	assert PyBigUint(B) / A == B / A
def test_pow():
	assert PyBigUint(A) ** 2 == PyBigUint(A ** 2)
	assert PyBigUint(B) ** 2 == PyBigUint(B ** 2)

def test_shr():
	assert (PyBigUint(A) >> 10 == PyBigUint(A >> 10))
def test_shl():
	assert (PyBigUint(A) << 10 == PyBigUint(A << 10))

def test_and():
	assert (PyBigUint(A) & PyBigUint(B)) == PyBigUint(A & B)
	assert (A & PyBigUint(B)) == PyBigUint(A & B)
	assert (PyBigUint(A) & B) == PyBigUint(A & B)
	a = PyBigUint(A)
	a &= B
	assert a == A & B
def test_xor():
	assert (PyBigUint(A) ^ PyBigUint(B)) == PyBigUint(A ^ B)
	assert (A ^ PyBigUint(B)) == PyBigUint(A ^ B)
	assert (PyBigUint(A) ^ B) == PyBigUint(A ^ B)
	a = PyBigUint(A)
	a ^= B
	assert a == A ^ B
def test_or():
	assert (PyBigUint(A) | PyBigUint(B)) == PyBigUint(A | B)
	assert (A | PyBigUint(B)) == PyBigUint(A | B)
	assert (PyBigUint(A) | B) == PyBigUint(A | B)
	a = PyBigUint(A)
	a |= B
	assert a == A | B

def test_float():
	assert float(PyBigUint(A)) == float(A)
	assert float(PyBigUint(B)) == float(B)
def test_int():
	assert int(PyBigUint(A)) == A
	assert int(PyBigUint(B)) == B
def test_from_float():
	assert PyBigUint(float(A)) == PyBigUint(int(float(A)))

def test_eq():
	assert PyBigUint(A) + PyBigUint(B) == PyBigUint(A + B)
	assert PyBigUint(A) + PyBigUint(B) == A + B
def test_le():
	assert PyBigUint(A) + PyBigUint(B) <= PyBigUint(A + B)
	assert PyBigUint(A) + PyBigUint(B) <= A + B
def test_ge():
	assert PyBigUint(A) + PyBigUint(B) >= PyBigUint(A + B)
	assert PyBigUint(A) + PyBigUint(B) >= A + B
def test_lt():
	assert PyBigUint(A) < PyBigUint(A + B)
	assert A < PyBigUint(A + B)
	assert PyBigUint(A) < A + B
def test_gt():
	assert PyBigUint(A + B) > PyBigUint(A)
	assert PyBigUint(A + B) > A

def test_bool():
	assert bool(PyBigUint(5)) == bool(1)
	assert bool(PyBigUint(1)) == bool(1)
	assert bool(PyBigUint(0)) == bool(0)

def test_rand():
	C = gen_random_pybiguint(100)
	assert True