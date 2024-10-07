import csv
import pandas as pd
import matplotlib.pyplot as plt

crazyflie2_1 = True

#df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1_flight/in_place_flight/net_drift_logger.csv")

#df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/in_place_flight/net_drift_logger.csv")
#df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/forward/net_drift_logger.csv")
#df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/backward/net_drift_logger.csv")
#df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/right/net_drift_logger.csv")
df1 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/left/net_drift_logger.csv")

#df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1_flight/in_place_flight/net_drift_manual.csv")

#df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/in_place_flight/net_drift_manual.csv")
#df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/forward/net_drift_manual.csv")
#df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/backward/net_drift_manual.csv")
#df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/right/net_drift_manual.csv")
df2 = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/left/net_drift_manual.csv")


x1 = []
y1 = []

x2 = []
y2 = []

x_sum_logger = 0
y_sum_logger = 0

x_sum_manual = 0
y_sum_manual = 0

for i in range(len(df1)):
    if crazyflie2_1 == True:
        if i == 25 or i == 35:
            continue

    x1.append(100*df1["Y-Net"][i]*(-1))
    y1.append(100*df1["X-Net"][i])
    x2.append(2.54*df2["X-Net"][i])
    y2.append(2.54*df2["Y-Net"][i])

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
plt.scatter(0, 0, color='0', marker='X', s=100)

for i in range(len(x1)):
    x = [x1[i], x2[i]]
    y = [y1[i], y2[i]]
    plt.plot(x, y, color="black", linewidth=1, alpha=0.225)

plt.legend(loc = "upper left")#bbox_to_anchor=(1.05, 1), loc = "upper left")

plt.title("Net Drone Drift for In Place Flight")

plt.xlabel("Horizontal (X) Drift (Centimeters)")
plt.ylabel("Vertical (Y) Drift (Centimeters)")

plt.grid()

plt.show()