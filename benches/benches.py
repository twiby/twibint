import twibint as bi 

def bench_product(A, B, nb_samples):
	for i in range(nb_samples):
		print(str(i) + "/" + str(nb_samples))
		C = A * B
	print()

SIZE = 50000
NB_SAMPLES = 50

A_bi = bi.gen_random_biguint(SIZE)
B_bi = bi.gen_random_biguint(SIZE)
A_int = int(A_bi)
B_int = int(B_bi)

# bench_product(A_bi, B_bi, NB_SAMPLES)
bench_product(A_int, B_int, NB_SAMPLES)