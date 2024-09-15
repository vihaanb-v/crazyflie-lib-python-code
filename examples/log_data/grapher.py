import csv
import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/net_drift_logger.csv")

x = []
y = []

for i in range(len(df)):
    x.append(df["X-Net"][i])
    y.append(df["Y-Net"][i])

plt.scatter(x, y)

plt.legend("Drone Landing Positions")

plt.title("Drone Drift Calculate Through Crazyflie Logging System")

plt.show()