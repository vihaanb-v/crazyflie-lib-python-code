import logging
import sys
import time
import os
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

DEFAULT_HEIGHT = 1

deck_attached_event = Event()

logging.basicConfig(level=logging.ERROR)

def take_off_simple(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(10)
        mc.stop()

    print("Touchdown.")

def straight_line(scf, lg_stab, direction):
    print("Takeoff.")

    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code to fly 2 meters forward
        for i in range(1):
            time.sleep(3)

            if direction == 'f':
                mc.forward(2, velocity=1.5)
            elif direction == 'b':
                mc.back(2, velocity=1.5)
            elif direction == 'r':
                mc.right(2, velocity=1.5)
            elif direction == 'l':
                mc.left(2, velocity=1.5)
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
def drone_logging(scf, lg_stab):
    with SyncLogger(scf, lg_stab) as logger:
            curr = time.time()
            # Iterate the logger to get the values
            count = 0
            for log_entry in logger:
                print(log_entry[1]['stateEstimate.x'])
                # Do useful stuff
                count += 1
                if (time.time() > curr + 15):
                    # The logging will continue until you exit the loop
                    break

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
        lg_stab = LogConfig(name='Stabilizer', period_in_ms=100)
        lg_stab.add_variable('stateEstimate.x', 'float')
        lg_stab.add_variable('stateEstimate.y', 'float')
        lg_stab.add_variable('stateEstimate.z', 'float')

        #drone_logging(scf, lg_stab)

        #Conducting multi-threading of flight and logging
        t1 = threading.Thread(target=take_off_simple, args=(scf, lg_stab))
        #t1 = threading.Thread(target=straight_line, args=(scf, lg_stab, 'f'))
        t2 = threading.Thread(target=drone_logging, args=(scf, lg_stab))

        t1.start()
        t2.start()

        t1.join()
        t2.join()

        print("Logging & Flight done!")