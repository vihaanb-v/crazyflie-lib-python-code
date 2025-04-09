import ctypes

#Edit to access your personally built RTLola Monitors
drift_lib = ctypes.CDLL('/home/bitcraze/projects/rtlola/rtlola_spec/in_place_monitor/libmonitor.so')

# Define the structure for Memory_x
class Memory_x(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.ARRAY(ctypes.c_double, 1)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_y
class Memory_y(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.ARRAY(ctypes.c_double, 1)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_x0
class Memory_x0(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.ARRAY(ctypes.c_double, 1)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_y0
class Memory_y0(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.ARRAY(ctypes.c_double, 1)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_x_drift
class Memory_x_drift(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.ARRAY(ctypes.c_double, 1)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_y_drift
class Memory_y_drift(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.ARRAY(ctypes.c_double, 1)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_trigger_0
class Memory_trigger_0(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.POINTER(ctypes.c_char_p)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for Memory_trigger_1
class Memory_trigger_1(ctypes.Structure):
    _fields_ = [("valid", ctypes.ARRAY(ctypes.c_bool, 1)),
                ("values", ctypes.POINTER(ctypes.c_char_p)),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Define the structure for StreamMemory
class StreamMemory(ctypes.Structure):
    _fields_ = [("x", Memory_x),
                ("y", Memory_y),
                ("x0", Memory_x0),
                ("y0", Memory_y0),
                ("x_drift", Memory_x_drift),
                ("y_drift", Memory_y_drift),
                ("trigger_0", Memory_trigger_0),
                ("trigger_1", Memory_trigger_1)]

# Define the structure for Memory
class Memory(ctypes.Structure):
    _fields_ = [("stream_memory", StreamMemory),
                ("time", ctypes.c_double)]

# Define the structure for Event
class Event(ctypes.Structure):
    _fields_ = [("has_x", ctypes.c_bool),
                ("x", ctypes.c_double),
                ("has_y", ctypes.c_bool),
                ("y", ctypes.c_double)]

# Define the structure for InternalEvent
class InternalEvent(ctypes.Structure):
    _fields_ = [("has_x", ctypes.c_bool),
                ("x", ctypes.c_double),
                ("has_y", ctypes.c_bool),
                ("y", ctypes.c_double),
                ("time", ctypes.c_double)]

# Define the structure for Verdict
class Verdict(ctypes.Structure):
    _fields_ = [("has_trigger_0", ctypes.c_bool),
                ("trigger_0", ctypes.POINTER(ctypes.c_char_p)),
                ("has_trigger_1", ctypes.c_bool),
                ("trigger_1", ctypes.POINTER(ctypes.c_char_p)),
                ("time", ctypes.c_double)]

# Function to initialize StreamMemory
drift_lib.init_stream_memory.argtypes = [ctypes.POINTER(StreamMemory)]
drift_lib.init_stream_memory.restype = None  # void function

# Function to initialize Memory with start_time as double
drift_lib.memory_init.argtypes = [ctypes.POINTER(Memory), ctypes.c_double]
drift_lib.memory_init.restype = None  # void function

# Function to accept an event (returns a Verdict)
drift_lib.accept_event.argtypes = [ctypes.POINTER(Memory), Event, ctypes.c_double]
drift_lib.accept_event.restype = Verdict

# Additional functions for the various structures such as Memory_x, Memory_y, etc.
# Memory_x functions (similar for other memory structures like Memory_y, Memory_x_drift, etc.)
drift_lib.memory_get_x.argtypes = [ctypes.POINTER(Memory_x), ctypes.c_uint]
drift_lib.memory_get_x.restype = ctypes.c_double

drift_lib.memory_shift_x.argtypes = [ctypes.POINTER(Memory_x)]
drift_lib.memory_shift_x.restype = None  # void function

drift_lib.memory_update_x.argtypes = [ctypes.POINTER(Memory_x), ctypes.c_double]
drift_lib.memory_update_x.restype = None  # void function

# Same pattern for Memory_y, Memory_x0, Memory_y0, etc.
drift_lib.memory_get_y.argtypes = [ctypes.POINTER(Memory_y), ctypes.c_uint]
drift_lib.memory_get_y.restype = ctypes.c_double

drift_lib.memory_shift_y.argtypes = [ctypes.POINTER(Memory_y)]
drift_lib.memory_shift_y.restype = None  # void function

drift_lib.memory_update_y.argtypes = [ctypes.POINTER(Memory_y), ctypes.c_double]
drift_lib.memory_update_y.restype = None  # void function

# For trigger_0, trigger_1, etc. (same pattern for other trigger types)
drift_lib.memory_get_trigger_0.argtypes = [ctypes.POINTER(Memory_trigger_0), ctypes.c_uint]
drift_lib.memory_get_trigger_0.restype = ctypes.c_char_p

drift_lib.memory_shift_trigger_0.argtypes = [ctypes.POINTER(Memory_trigger_0)]
drift_lib.memory_shift_trigger_0.restype = None  # void function

drift_lib.memory_update_trigger_0.argtypes = [ctypes.POINTER(Memory_trigger_0), ctypes.c_char_p]
drift_lib.memory_update_trigger_0.restype = None  # void function

# For other trigger types (trigger_1, trigger_2, etc.)
drift_lib.memory_get_trigger_1.argtypes = [ctypes.POINTER(Memory_trigger_1), ctypes.c_uint]
drift_lib.memory_get_trigger_1.restype = ctypes.c_char_p

drift_lib.memory_shift_trigger_1.argtypes = [ctypes.POINTER(Memory_trigger_1)]
drift_lib.memory_shift_trigger_1.restype = None  # void function

drift_lib.memory_update_trigger_1.argtypes = [ctypes.POINTER(Memory_trigger_1), ctypes.c_char_p]
drift_lib.memory_update_trigger_1.restype = None  # void function

memory_x_instance = Memory_x()
memory_y_instance = Memory_y()
memory_x0_instance = Memory_x0()
memory_y0_instance = Memory_y0()
memory_x_drift_instance = Memory_x_drift()
memory_y_drift_instance = Memory_y_drift()
memory_trigger_0_instance = Memory_trigger_0()
memory_trigger_1_instance = Memory_trigger_1()

# Instantiate StreamMemory with all the instances of the other structures
stream_memory_instance = StreamMemory(
    x=memory_x_instance,
    y=memory_y_instance,
    x0=memory_x0_instance,
    y0=memory_y0_instance,
    x_drift=memory_x_drift_instance,
    y_drift=memory_y_drift_instance,
    trigger_0=memory_trigger_0_instance,
    trigger_1=memory_trigger_1_instance
)

# Instantiate Memory with the stream_memory_instance and a start time
memory_instance = Memory(
    stream_memory=stream_memory_instance,
    time=0.0  # you can replace this with any starting time value
)

# Instantiate Event and InternalEvent
event_instance = Event(
    has_x=True,
    x=1.0,  # example x value
    has_y=True,
    y=2.0  # example y value
)

internal_event_instance = InternalEvent(
    has_x=True,
    x=1.0,  # example x value
    has_y=True,
    y=2.0,  # example y value
    time=0.0  # example time value
)

# Instantiate Verdict
verdict_instance = Verdict(
    has_trigger_0=True,
    trigger_0=None,  # Set this to the appropriate value if needed
    has_trigger_1=True,
    trigger_1=None,  # Set this to the appropriate value if needed
    time=0.0  # example time value
)

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

def fly_left(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(2)
        mc.left(1.5, velocity=1.5)
        time.sleep(2)
        mc.stop()

    print("Touchdown.")

def fly_right(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(2)
        mc.right(1.5, velocity=1.5)
        time.sleep(2)
        mc.stop()

    print("Touchdown.")

def fly_forward(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(2)
        mc.forward(1.5, velocity=1.5)
        time.sleep(2)
        mc.stop()

    print("Touchdown.")

def fly_backward(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(2)
        mc.back(1.5, velocity=1.5)
        time.sleep(2)
        mc.stop()

    print("Touchdown.")

def straight_line(scf, lg_stab, direction):
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
        
        #Directional drift test Crazyflie 2.1
        #project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1_flight/path_flight/forward/"
        
        #In place drift test Crazyflie 2.1
        #project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1_flight/in_place_flight"
        
        #Patterned floor test Crazyflie 2.1+
        #project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/patterned_floor/in_place_flight"

        #In place drift test Crazyflie 2.1+
        #project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/in_place_flight"
       
        #Directional drift test Crazyflie 2.1+
        #project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/2.1+_flight/path_flight/left"

        #Testing folder for logging
        project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/tests"

        print(project_directory)

        full_csv_path = os.path.join(project_directory, "run2.csv")

        first_time = True

        with SyncLogger(scf, lg_stab) as logger:
            end_time = time.time() + 35
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
        scf.cf.param.add_update_callback(group='deck', name='bcFlow2', cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=1):
            print('No flow deck detected!')
            sys.exit(1)

        #Defining log variables
        lg_stab = LogConfig(name='Position', period_in_ms=100)
        lg_stab.add_variable('stateEstimate.x', 'float')
        lg_stab.add_variable('stateEstimate.y', 'float')
        lg_stab.add_variable('stateEstimate.z', 'float')

        #take_off_simple(scf, lg_stab)
        #straight_line(scf, lg_stab, 'f')

        t1 = threading.Thread(target=take_off_simple, args=(scf, lg_stab))
        #t1 = threading.Thread(target=fly_right, args=(scf, lg_stab))
        #t1 = threading.Thread(target=straight_line, args=(scf, lg_stab, 'l'))
        t2 = threading.Thread(target=drone_logging, args=(scf, lg_stab, "entire_flight"))

        t1.start()
        t2.start()

        t1.join()
        t2.join()

        print("Logging & flight completed.")