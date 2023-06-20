import os
import json
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt

walker = os.walk("./target/criterion")
folders = next(walker)[1]

data = {
	name: {
		"new": {"x": [], "low_y": [], "up_y": []},
		"base": {"x": [], "low_y": [], "up_y": []},
	} for name in ["add", "mul", "sub"]
}

for f in folders:
	if f  == "report":
		continue

	try :
		with open("./target/criterion/" + f + "/"+"new"+"/estimates.json", "r") as file:
			new_data = json.load(file)
		lower_bound = new_data["mean"]["confidence_interval"]["lower_bound"]
		upper_bound = new_data["mean"]["confidence_interval"]["upper_bound"]

		name, x = f.split(" ")
		if name in data.keys():
			data[name]["new"]["x"].append(int(x));
			data[name]["new"]["low_y"].append(lower_bound);
			data[name]["new"]["up_y"].append(upper_bound);
	except FileNotFoundError:
		pass


for name, points in data.items():
	fig = plt.figure()
	fig.suptitle(name)

	indices = np.argsort(points["new"]["x"])
	X = np.array(points["new"]["x"])[indices]
	Y1 = np.array(points["new"]["low_y"])[indices]
	Y2 = np.array(points["new"]["up_y"])[indices]

	m,b = np.polyfit(
		np.log(X), 
		np.log(Y1), 
		1)
	plt.plot(X, Y1, "x", label="lower bound")
	plt.plot(X, Y2, "x", label="upper bound")
	plt.plot(X, np.exp(b)*(X**m), color="red", label="linear fit "+str(m))

	plt.xlabel("time in ns")
	plt.ylabel("number of digits (in base 2³²)")
	plt.yscale("log")
	plt.xscale("log")
	plt.legend()

plt.show()