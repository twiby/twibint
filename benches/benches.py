import time
import twibint as bi 

def bench_product(A, B, nb_samples):
	for i in range(nb_samples):
		C = A * B

print("BENCHING PRODUCT")
SIZE = 50000
NB_SAMPLES = 50

A_bi = bi.gen_random_biguint(SIZE)
B_bi = bi.gen_random_biguint(SIZE)
A_int = int(A_bi)
B_int = int(B_bi)

start = time.time()
bench_product(A_bi, B_bi, NB_SAMPLES)
end = time.time()
print("[twibint] elapsed:", end - start)

start = time.time()
bench_product(A_int, B_int, NB_SAMPLES)
end = time.time()
print("[Python integers] elapsed:", end - start)
print()
print()

def bench_div(A, B, nb_samples):
	for i in range(nb_samples):
		C = A // B

print("BENCHING DIV")
SIZE = 5000
NB_SAMPLES = 50

A_bi = bi.gen_random_biguint(SIZE)
B_bi = bi.gen_random_biguint(SIZE//3)
A_int = int(A_bi)
B_int = int(B_bi)

start = time.time()
bench_div(A_bi, B_bi, NB_SAMPLES)
end = time.time()
print("[twibint] elapsed:", end - start)

start = time.time()
bench_div(A_int, B_int, NB_SAMPLES)
end = time.time()
print("[Python integers] elapsed:", end - start)
print()
print()

def bench_add(A, B, nb_samples):
	for i in range(nb_samples):
		for j in range(500):
			C = A + B

print("BENCHING ADD")
SIZE = 50000
NB_SAMPLES = 50

A_bi = bi.gen_random_biguint(SIZE)
B_bi = bi.gen_random_biguint(SIZE)
A_int = int(A_bi)
B_int = int(B_bi)

start = time.time()
bench_add(A_bi, B_bi, NB_SAMPLES)
end = time.time()
print("[twibint] elapsed:", end - start)

start = time.time()
bench_add(A_int, B_int, NB_SAMPLES)
end = time.time()
print("[Python integers] elapsed:", end - start)
