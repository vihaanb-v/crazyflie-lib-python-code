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

#Logger Data
x1 = []
y1 = []

#Manual Data
x2 = []
y2 = []

#Averages
mean_x_logger = []
mean_y_logger = []

mean_x_manual = []
mean_y_manual = []

#Sequential List of Number of Tests
count_list = []

for i in range(len(df1)):
    if crazyflie2_1 == True:
        if i == 25 or i == 35:
            continue

    x1.append(100*df1["Y-Net"][i]*(-1))
    y1.append(100*df1["X-Net"][i])
    x2.append(2.54*df2["X-Net"][i])
    y2.append(2.54*df2["Y-Net"][i])

for i in range(len(x1)):
    count_list.append(i+1)

def calc_means():
    #For Average Calculation
    count = 0
    running_sum_x = 0
    running_sum_y = 0

    for i in range(len(x1)):
        count += 1
        running_sum_x += x1[i]
        running_sum_y += y1[i]
        mean_x_logger.append(running_sum_x/count)
        mean_y_logger.append(running_sum_y/count)

    running_sum_x = 0
    running_sum_y = 0
    
    for i in range(len(x2)):
        count += 1
        running_sum_x += x2[i]
        running_sum_y += y2[i]
        mean_x_manual.append(running_sum_x/count)
        mean_y_manual.append(running_sum_y/count)

    running_sum_x = 0
    running_sum_y = 0

if __name__ == "__main__":
    calc_means()
    #print(count_list)
    #print(mean_x_logger)
    #print(mean_y_logger)

    #Line Plot of Net Drift values for Logger Data
    #plt.scatter(count_list, mean_x_logger, label="Average X-Drift Logger")
    #plt.scatter(count_list, mean_y_logger, label="Average Y-Drift Logger")
    
    #Line Plot of Net Drift values for Manual Data 
    plt.scatter(count_list, mean_x_manual, label="Average X-Drift Manual")
    plt.scatter(count_list, mean_y_manual, label="Average Y-Drift Manual")

    '''
    for i in range(len(count_list) - 1):
        x = [count_list[i], count_list[i+1]]
        y = [mean_x_logger[i], mean_x_logger[i+1]]
        plt.plot(x, y, color="black", linewidth=1, alpha=0.5)

        x = [count_list[i], count_list[i+1]]
        y = [mean_y_logger[i], mean_y_logger[i+1]]
        plt.plot(x, y, color="black", linewidth=1, alpha=0.5)
    '''

    for i in range(len(count_list) - 1):
        x = [count_list[i], count_list[i+1]]
        y = [mean_x_manual[i], mean_x_manual[i+1]]
        plt.plot(x, y, color="black", linewidth=1, alpha=0.5)

        x = [count_list[i], count_list[i+1]]
        y = [mean_y_manual[i], mean_y_manual[i+1]]
        plt.plot(x, y, color="black", linewidth=1, alpha=0.5)

    plt.legend(loc = "upper right")#bbox_to_anchor=(1.05, 1), loc = "upper left")

    plt.title("Net Drone Drift Averages for Flight")

    plt.xlabel("Number of Tests Conducted (#)")
    plt.ylabel("Net Drift (Centimeters)")

    plt.grid()

    plt.show()
