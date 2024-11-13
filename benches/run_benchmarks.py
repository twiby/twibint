import os
import sys
import json
import platform
import subprocess
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt

VERSIONS = {
	"num-bigint": "0.4.6",
	"twibint": "0.3.2",
}

def run_benchmark(crate):
	print("Running benchmarks for crate \"" + crate + "\"")
	working_directory = os.getcwd()
	p = subprocess.Popen(
		["cargo", "clean"], 
		cwd = working_directory + "/bencher")
	p.wait()
	p = subprocess.Popen(
		["cargo", "bench", "--features=" + crate], 
		cwd = working_directory + "/bencher")
	p.wait()

def split_operands(s):
	if 'x' in s:
		return [int(n) for n in s.split('x')]
	elif '+' in s:
		return [int(n) for n in s.split('+')]
	elif '-' in s:
		return [int(n) for n in s.split('-')]
	elif '_' in s:
		return [int(n) for n in s.split('_')]
	else:
		raise ValueError("op not found in " + s)

def get_run_results(f):
	with open("./bencher/target/criterion/" + f + "/"+"new"+"/estimates.json", "r") as file:
		new_data = json.load(file)
		lower_bound = new_data["mean"]["confidence_interval"]["lower_bound"]
		upper_bound = new_data["mean"]["confidence_interval"]["upper_bound"]
	return lower_bound, upper_bound

def get_benchmark_results():
	walker = os.walk("./bencher/target/criterion")
	folders = next(walker)[1]

	data = {}
	for f in folders:
		if f  == "report":
			continue

		op, size  = f.split(" ")
		mean_size = sum(split_operands(size)) / 2
		lower, upper = get_run_results(f)

		if not op in data.keys():
			data[op] = {}

		data[op][mean_size] = {"lower": lower, "upper": upper}
	return data

def get_figure(op, figures):
	if not op in figures.keys():
		f = plt.figure()
		f.suptitle(op + "\n" + platform.processor())
		figures[op] = f
	return figures[op]

def draw_benchmark_results(data, figures):
	for op, op_data in data.items():
		plt.figure(get_figure(op, figures))

		X = sorted(list(op_data.keys()))
		Y1 = np.array([op_data[x]["lower"] for x in X])
		Y2 = np.array([op_data[x]["upper"] for x in X])
		Y = (Y1 + Y2) / 2

		plt.plot(X, Y, label = crate + " " + VERSIONS[crate])
		plt.ylabel("time in ns")
		plt.xlabel("number of bits")
		plt.yscale("log")
		plt.xscale("log")
		plt.legend()


#################
#################
#################
#################

figures = {}
for crate in ["twibint", "num-bigint"]:
	run_benchmark(crate)
	data = get_benchmark_results()
	draw_benchmark_results(data, figures)

plt.show()
