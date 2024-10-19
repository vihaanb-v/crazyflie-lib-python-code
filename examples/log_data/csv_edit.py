import csv
import pandas as pd

run_num = 25

with open("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1_flight_new_props/in_place_flight/net_drift_logger.csv", 'a', newline = '') as file:
    writer = csv.writer(file)

    '''
    field = ['Run',
            'Timestamp_Init',
             'Timestamp_Final',
            'X-Net',
            'Y-Net',
            'Z-Net'
            ]
    
    writer.writerow(field)
    '''

    df = pd.read_csv("/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1_flight_new_props/in_place_flight/run" + str(run_num) + ".csv")
    #print(len(df))

    timestamp_init = df['Timestamp'][0]
    x_init = df['X-Coordinate'][1]
    y_init = df['Y-Coordinate'][1]
    z_init = df['Z-Coordinate'][1]

    timestamp_final = df['Timestamp'][len(df) - 1]
    x_final = df['X-Coordinate'][len(df) - 1]
    y_final = df['Y-Coordinate'][len(df) - 1]
    z_final = df['Z-Coordinate'][len(df) - 1]

    x_net = x_final - x_init
    y_net = y_final - y_init
    z_net = z_final - z_init

    writer.writerow(
        [run_num,
         timestamp_init,
         timestamp_final,
         x_net,
         y_net,
         z_net]
    )