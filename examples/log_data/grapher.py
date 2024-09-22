import csv
import pandas as pd
import matplotlib.pyplot as plt

df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/in_place_flight/net_drift_logger.csv")

df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/in_place_flight/net_drift_manual.csv")

x1 = []
y1 = []

x2 = []
y2 = []

x_sum_logger = 0
y_sum_logger = 0

x_sum_manual = 0
y_sum_manual = 0

for i in range(len(df1)):
    if i == 25 or i == 35:
        continue
    else:
        x1.append(39.3701*df1["X-Net"][i])
        y1.append(39.3701*df1["Y-Net"][i])

for i in range(len(df2)):
    x2.append(df2["X-Net"][i])
    y2.append(df2["Y-Net"][i])

for i in range(len(x1)):
    x_sum_logger += x1[i]
    y_sum_logger += y1[i]

for i in range(len(x2)):
    x_sum_manual += x2[i]
    y_sum_manual += y2[i]

#Plotting all data runs excluding outliers for logger data
plt.scatter(x1, y1, label="Logger Recorded Drift")
plt.scatter(x2, y2, label="Manually Recorded Drift")

#Plotting average points
plt.scatter(x_sum_logger/len(x1), y_sum_logger/len(x1), label="Average Point for Logger Data", marker='X', s=90)
plt.scatter(x_sum_manual/len(x2), y_sum_manual/len(x2), label="Average Point for Manual Data", marker='X', s=90)

plt.legend(loc = "upper left")

plt.title("Net Drone Drift for In Place Flight")

plt.xlabel("Horizontal (X) Drift (Inches)")
plt.ylabel("Vertical (Y) Drift (Inches)")

plt.grid()

plt.show()