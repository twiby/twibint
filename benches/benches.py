import time
import twibint as bi 

def bench_product(A, B, nb_samples):
	for i in range(nb_samples):
		print(str(i) + "/" + str(nb_samples))
		C = A * B
	print()

SIZE = 50000
NB_SAMPLES = 50

A_bi = bi.gen_random_pybiguint(SIZE)
B_bi = bi.gen_random_pybiguint(SIZE)
A_int = int(A_bi)
B_int = int(B_bi)

start = time.time()
bench_product(A_bi, B_bi, NB_SAMPLES)
end = time.time()
print("elapsed:", end - start)

start = time.time()
bench_product(A_int, B_int, NB_SAMPLES)
end = time.time()
print("elapsed:", end - start)

def bench_add(A, B, nb_samples):
	for i in range(nb_samples):
		print(str(i) + "/" + str(nb_samples))
		for j in range(500):
			C = A + B
	print()

SIZE = 50000
NB_SAMPLES = 50

A_bi = bi.gen_random_pybiguint(SIZE)
B_bi = bi.gen_random_pybiguint(SIZE)
A_int = int(A_bi)
B_int = int(B_bi)

start = time.time()
bench_add(A_bi, B_bi, NB_SAMPLES)
end = time.time()
print("elapsed:", end - start)

start = time.time()
bench_add(A_int, B_int, NB_SAMPLES)
end = time.time()
print("elapsed:", end - start)
