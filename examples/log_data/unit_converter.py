import csv
import pandas as pd

df = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/net_drift_logger.csv")

coords = []

for i in range(len(df)):
    x_val = 39.3701*df["X-Net"][i]
    y_val = 39.3701*df["Y-Net"][i]

    coords.append(str(x_val) + ", " + str(y_val))

print(coords)