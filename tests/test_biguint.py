import bigint as bi

def test_biguint():
	a = 123456789101112131415161718
	b = 987654321919293949596979899

	assert bi.BigUint(a) + bi.BigUint(b) == bi.BigUint(a + b)