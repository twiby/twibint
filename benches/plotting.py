import os
import json
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt

walker = os.walk("./target/criterion")
folders = next(walker)[1]

data = {
	name: {"x": [], "low_y": [], "up_y": []} for name in ["add", "mul"]
}

for f in folders:
	if f  == "report":
		continue
	with open("./target/criterion/" + f + "/new/estimates.json", "r") as file:
		new_data = json.load(file)
	lower_bound = new_data["mean"]["confidence_interval"]["lower_bound"]
	upper_bound = new_data["mean"]["confidence_interval"]["upper_bound"]

	name, x = f.split(" ")
	if name in data.keys():
		data[name]["x"].append(int(x));
		data[name]["low_y"].append(lower_bound);
		data[name]["up_y"].append(upper_bound);


for name, points in data.items():
	indices = np.argsort(points["x"])
	X = np.array(points["x"])[indices]
	Y1 = np.array(points["low_y"])[indices]
	Y2 = np.array(points["up_y"])[indices]

	fig = plt.figure()
	fig.suptitle(name)
	m,b = np.polyfit(
		np.log(X), 
		np.log(Y1), 
		1)
	plt.plot(X, Y1, label="lower bound")
	plt.plot(X, Y2, label="upper bound")
	plt.plot(X, np.exp(b)*(X**m), color="red", label="linear fit "+str(m))
	plt.yscale("log")
	plt.xscale("log")
	plt.legend()

plt.show()