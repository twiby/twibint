import bigint as bi

def test_biguint():
	a = bi.BigUint(4294967295)
	b = bi.BigUint(4294967295)
	assert str(a + b) == "8589934590"