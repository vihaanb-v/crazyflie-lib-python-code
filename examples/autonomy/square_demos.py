import logging
import sys
import time
import os
import csv
import threading
from threading import Timer
from threading import Event

import cflib.crtp
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie
from cflib.crazyflie.syncLogger import SyncLogger
from cflib.positioning.motion_commander import MotionCommander
from cflib.crazyflie.log import LogConfig
from cflib.utils import uri_helper

import csv

URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')

DEFAULT_HEIGHT = 1.5
ERROR_PARAMETER = 0.2

deck_attached_event = Event()

logging.basicConfig(level=logging.ERROR)

def take_off_simple(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(10)
        mc.stop()

    print("Touchdown.")


def straight_line(scf, direction):
    print("Takeoff.")

    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(3)

        if direction == 'f':
            mc.forward(1.5, velocity=1.5)
        elif direction == 'b':
            mc.back(1.5, velocity=1.5)
        elif direction == 'r':
            mc.right(1.5, velocity=1.5)
        elif direction == 'l':
            mc.left(1.5, velocity=1.5)
            
        time.sleep(3)
        mc.stop()

    print("Touchdown.")

def square_turns(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code using turns from bottom right corner of square
        for i in range(1):
            time.sleep(3)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.turn_left(90)
            mc.stop()

def square_no_turns(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code without turns from bottom right corner of square
        for i in range(1):
            time.sleep(3)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.left(2, velocity=1.5)
            time.sleep(3)
            mc.back(2, velocity=1.5)
            time.sleep(3)
            mc.right(2, velocity=1.5)
            time.sleep(3)
            mc.stop()

def square_turns_no_stops(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code using turns without stops from bottom right corner of square
        for i in range(1):
            time.sleep(3)
            mc.forward(2, velocity=1.5)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            mc.turn_left(90)
            time.sleep(3)
            mc.stop()

def square_no_turns_no_stops(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code without turns or stops from bottom right corner of square
        for i in range(1):
            time.sleep(3)
            mc.forward(2, velocity=1.5)
            mc.left(2, velocity=1.5)
            mc.back(2, velocity=1.5)
            mc.right(2, velocity=1.5)
            time.sleep(3)
            mc.stop()

#Check if deck is attached to Crazyflie
def param_deck_flow(name, value_str):
    value = int(value_str)
    print(value)
    if value:
        deck_attached_event.set()
        print('Deck is attached!')
    else:
        print('Deck is NOT attached!')

#Log position data of drone
def drone_logging(scf, lg_stab, mode):

    if mode == "stationary":
        with SyncLogger(scf, lg_stab) as logger:
            # Iterate the logger to get the values
            count = 0
            for log_entry in logger:
                print("(" + "Timestamp: " + str(log_entry[0]) + ", " + str(log_entry[1]['stateEstimate.x']) + ", " + str(log_entry[1]['stateEstimate.y']) + ", " + str(log_entry[1]['stateEstimate.z']) + ")")
                count += 1
                x_pos_total += log_entry[1]['stateEstimate.x']
                y_pos_total += log_entry[1]['stateEstimate.y']
                z_pos_total += log_entry[1]['stateEstimate.z']
                if (count > 10):
                    # The logging will continue until you exit the loop
                    break

    elif mode == "moving":
        with SyncLogger(scf, lg_stab) as logger:
            while log_entry[1]['stateEstimate.z'] > 0.97:
                # Iterate the logger to get the values
                count = 0
                for log_entry in logger:
                    print("(" + "Timestamp: " + str(log_entry[0]) + ", " + str(log_entry[1]['stateEstimate.x']) + ", " + str(log_entry[1]['stateEstimate.y']) + ", " + str(log_entry[1]['stateEstimate.z']) + ")")
                    
                    count += 1
                    z_pos_total += log_entry[1]['stateEstimate.z']
                    if (count > 10):
                        # The logging will continue until you exit the loop
                        break

    elif mode == "hover":
        with SyncLogger(scf, lg_stab) as logger:
            while log_entry[1]['stateEstimate.z'] > 0.97:
                # Iterate the logger to get the values
                count = 0
                for log_entry in logger:
                    print("(" + "Timestamp: " + str(log_entry[0]) + ", " + str(log_entry[1]['stateEstimate.x']) + ", " + str(log_entry[1]['stateEstimate.y']) + ", " + str(log_entry[1]['stateEstimate.z']) + ")")
                    
                    x_pos_total += log_entry[1]['stateEstimate.x']
                    y_pos_total += log_entry[1]['stateEstimate.y']
                    z_pos_total += log_entry[1]['stateEstimate.z']
                    count += 1

    elif mode == "entire_flight":
        #Have to change your file path
        project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/path_flight/forward/"
        print(project_directory)

        full_csv_path = os.path.join(project_directory, "run1.csv")

        first_time = True
        inFlight = False

        with SyncLogger(scf, lg_stab) as logger:
            if inFlight == True:
                end_time = time.time() + 35
                time.sleep(5)
            else:
                end_time = time.time() + 29
                time.sleep(5)

            for log_entry in logger:
                if time.time() < end_time:
                    print("Time: {}, Initial Time: {}".format(time.time(), end_time))
                    print("(" + "Timestamp: " + str(log_entry[0]) + ", " + str(log_entry[1]['stateEstimate.x']) + ", " + str(log_entry[1]['stateEstimate.y']) + ", " + str(log_entry[1]['stateEstimate.z']) + ")")

                    with open(full_csv_path, 'a', newline = '') as file:
                        writer = csv.writer(file)

                        if first_time == True:
                            field = ['Timestamp',
                            'X-Coordinate',
                            'Y-Coordinate',
                            'Z-Coordinate'
                            ]
                
                            writer.writerow(field)

                            first_time = False

                        else:
                            writer.writerow(
                            [log_entry[0],
                            log_entry[1]['stateEstimate.x'],
                            log_entry[1]['stateEstimate.y'],
                            log_entry[1]['stateEstimate.z']
                            ])


if __name__ == '__main__':
    cflib.crtp.init_drivers()

    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache= './cache')) as scf:

        #Check if flow deck is attached
        scf.cf.param.add_update_callback(group='deck', name='bcFlow2',
                                         cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=1):
            print('No flow deck detected!')
            sys.exit(1)

        #Defining log variables
        lg_stab = LogConfig(name='Position', period_in_ms=100)
        lg_stab.add_variable('stateEstimate.x', 'float')
        lg_stab.add_variable('stateEstimate.y', 'float')
        lg_stab.add_variable('stateEstimate.z', 'float')

        #t1 = threading.Thread(target=take_off_simple, args=(scf, lg_stab))
        t1 = threading.Thread(target=straight_line, args=(scf, 'f'))
        t2 = threading.Thread(target=drone_logging, args=(scf, lg_stab, "entire_flight"))

        t1.start()
        t2.start()

        t1.join()
        t2.join()

        print("Logging & flight completed.")