import ctypes
from ctypes import CDLL, POINTER, Structure, c_double, c_int64, c_bool, c_char_p

import time

#Edit to access your personally built RTLola Monitors
drift_lib = ctypes.CDLL("/home/bitcraze/projects/crazyflie-lib-python-code/rtlola/waypoint_monitor/libmonitor.so")

# Define all BoundedBuffer structures
class BoundedBuffer_int64_t(ctypes.Structure):
    _fields_ = [("values", ctypes.c_int64 * 1),
                ("valid", ctypes.c_bool * 1),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

class BoundedBuffer_double(ctypes.Structure):
    _fields_ = [("values", ctypes.c_double * 1),
                ("valid", ctypes.c_bool * 1),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

class BoundedBuffer_bool(ctypes.Structure):
    _fields_ = [("values", ctypes.c_bool * 1),
                ("valid", ctypes.c_bool * 1),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

class BoundedBuffer_str(ctypes.Structure):
    _fields_ = [("values", ctypes.c_char_p * 1),
                ("valid", ctypes.c_bool * 1),
                ("current", ctypes.c_int),
                ("is_fresh", ctypes.c_bool)]

# Full Memory struct
class Memory(ctypes.Structure):
    _fields_ = [
        ("boundedbuffer_motor_pass", BoundedBuffer_int64_t),
        ("boundedbuffer_battery_pass", BoundedBuffer_int64_t),
        ("boundedbuffer_x_drift", BoundedBuffer_double),
        ("boundedbuffer_y_drift", BoundedBuffer_double),
        ("boundedbuffer_z_drift", BoundedBuffer_double),
        ("boundedbuffer_pitch", BoundedBuffer_double),
        ("boundedbuffer_roll", BoundedBuffer_double),
        ("boundedbuffer_yaw", BoundedBuffer_double),
        ("boundedbuffer_vbat", BoundedBuffer_double),
        ("boundedbuffer_abs_x_drift", BoundedBuffer_double),
        ("boundedbuffer_abs_y_drift", BoundedBuffer_double),
        ("boundedbuffer_abs_z_drift", BoundedBuffer_double),
        ("boundedbuffer_abs_pitch", BoundedBuffer_double),
        ("boundedbuffer_abs_roll", BoundedBuffer_double),
        ("boundedbuffer_abs_yaw", BoundedBuffer_double),
        ("boundedbuffer_motor_check", BoundedBuffer_int64_t),
        ("boundedbuffer_motor_failed", BoundedBuffer_bool),
        ("boundedbuffer_trigger_0", BoundedBuffer_str),
        ("boundedbuffer_battery_failed", BoundedBuffer_bool),
        ("boundedbuffer_trigger_1", BoundedBuffer_str),
        ("boundedbuffer_x_drift_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_2", BoundedBuffer_str),
        ("boundedbuffer_y_drift_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_3", BoundedBuffer_str),
        ("boundedbuffer_z_drift_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_4", BoundedBuffer_str),
        ("boundedbuffer_pitch_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_5", BoundedBuffer_str),
        ("boundedbuffer_roll_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_6", BoundedBuffer_str),
        ("boundedbuffer_yaw_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_7", BoundedBuffer_str),
        ("boundedbuffer_battery_low", BoundedBuffer_bool),
        ("boundedbuffer_trigger_8", BoundedBuffer_str),
        ("time", ctypes.c_double)
    ]

class InternalEvent(Structure):
    _fields_ = [
        ("motor_pass", c_int64),
        ("motor_pass_is_present", c_bool),
        ("battery_pass", c_int64),
        ("battery_pass_is_present", c_bool),
        ("x_drift", c_double),
        ("x_drift_is_present", c_bool),
        ("y_drift", c_double),
        ("y_drift_is_present", c_bool),
        ("z_drift", c_double),
        ("z_drift_is_present", c_bool),
        ("pitch", c_double),
        ("pitch_is_present", c_bool),
        ("roll", c_double),
        ("roll_is_present", c_bool),
        ("yaw", c_double),
        ("yaw_is_present", c_bool),
        ("vbat", c_double),
        ("vbat_is_present", c_bool),
        ("time", c_double),
    ]

class Verdict(Structure):
    _fields_ = [
        ("abs_x_drift", c_double),
        ("abs_x_drift_is_present", c_bool),
        ("abs_y_drift", c_double),
        ("abs_y_drift_is_present", c_bool),
        ("abs_z_drift", c_double),
        ("abs_z_drift_is_present", c_bool),
        ("abs_pitch", c_double),
        ("abs_pitch_is_present", c_bool),
        ("abs_roll", c_double),
        ("abs_roll_is_present", c_bool),
        ("abs_yaw", c_double),
        ("abs_yaw_is_present", c_bool),
        ("motor_check", c_int64),
        ("motor_check_is_present", c_bool),
        ("motor_failed", c_bool),
        ("motor_failed_is_present", c_bool),
        ("trigger_0", c_char_p),
        ("trigger_0_is_present", c_bool),
        ("battery_failed", c_bool),
        ("battery_failed_is_present", c_bool),
        ("trigger_1", c_char_p),
        ("trigger_1_is_present", c_bool),
        ("x_drift_exceeded", c_bool),
        ("x_drift_exceeded_is_present", c_bool),
        ("trigger_2", c_char_p),
        ("trigger_2_is_present", c_bool),
        ("y_drift_exceeded", c_bool),
        ("y_drift_exceeded_is_present", c_bool),
        ("trigger_3", c_char_p),
        ("trigger_3_is_present", c_bool),
        ("z_drift_exceeded", c_bool),
        ("z_drift_exceeded_is_present", c_bool),
        ("trigger_4", c_char_p),
        ("trigger_4_is_present", c_bool),
        ("pitch_exceeded", c_bool),
        ("pitch_exceeded_is_present", c_bool),
        ("trigger_5", c_char_p),
        ("trigger_5_is_present", c_bool),
        ("roll_exceeded", c_bool),
        ("roll_exceeded_is_present", c_bool),
        ("trigger_6", c_char_p),
        ("trigger_6_is_present", c_bool),
        ("yaw_exceeded", c_bool),
        ("yaw_exceeded_is_present", c_bool),
        ("trigger_7", c_char_p),
        ("trigger_7_is_present", c_bool),
        ("battery_low", c_bool),
        ("battery_low_is_present", c_bool),
        ("trigger_8", c_char_p),
        ("trigger_8_is_present", c_bool),
        ("time", c_double),
    ]

# Helper to build events
def create_event(motor_pass=None, battery_pass=None, x_drift=None, y_drift=None, z_drift=None,
                 pitch=None, roll=None, yaw=None, vbat=None, time_val=0.0):
    e = InternalEvent()

    def set_field(value, attr, present_attr):
        if value is not None:
            setattr(e, attr, value)
            setattr(e, present_attr, True)
        else:
            setattr(e, present_attr, False)

    set_field(motor_pass, 'motor_pass', 'motor_pass_is_present')
    set_field(battery_pass, 'battery_pass', 'battery_pass_is_present')
    set_field(x_drift, 'x_drift', 'x_drift_is_present')
    set_field(y_drift, 'y_drift', 'y_drift_is_present')
    set_field(z_drift, 'z_drift', 'z_drift_is_present')
    set_field(pitch, 'pitch', 'pitch_is_present')
    set_field(roll, 'roll', 'roll_is_present')
    set_field(yaw, 'yaw', 'yaw_is_present')
    set_field(vbat, 'vbat', 'vbat_is_present')
    e.time = time_val
    return e

# Trigger display helper
def display_verdict_triggers(verdict):
    for i in range(9):
        trigger_field = f"trigger_{i}"
        present_field = f"{trigger_field}_is_present"
        if getattr(verdict, present_field):
            msg_ptr = getattr(verdict, trigger_field)
            if msg_ptr:
                msg = ctypes.string_at(msg_ptr).decode()
                print(f"[TRIGGER] {trigger_field}: {msg}")

import logging
import sys
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

import random
import csv

URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')

DEFAULT_HEIGHT = 1.5
ERROR_PARAMETER = 0.2

deck_attached_event = Event()

logging.basicConfig(level=logging.ERROR)

memory_instance = Memory()  # Initialize once globally

def send_state_to_monitor(x_val, x0, y_val, y0, timestamp):
    x_drift_val = x_val - x0
    y_drift_val = y_val - y0

    event = create_event(
        x_drift=x_drift_val,
        y_drift=y_drift_val,
        time_val=timestamp / 1000.0  # Convert CF timestamp to seconds
    )

    verdict = drift_lib.cycle(ctypes.byref(memory_instance), event)
    display_verdict_triggers(verdict)

def take_off_simple(scf, lg_stab):
    print("Takeoff.")
    
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(10)
        mc.stop()

    print("Touchdown.")
    
#Waypoint flight
def waypoint_flight(scf, lg_stab, rnd):
    if rnd == True:
        x_pos = []
        y_pos = []
        z_pos = []

        for i in range(3):
            start = 0.6

            end_x = 2.6
            end_y = 2
            end_z = 1.8

            step = 0.2

            val_x = random.randint(0, int((end_x - start) / step))
            val_y = random.randint(0, int((end_y - start) / step))
            val_z = random.randint(0, int((end_z - start) / step))

            x_pos.append(start + val_x * step)
            y_pos.append(start + val_y * step)
            z_pos.append(start + val_z * step)

        print("Coordinate 1 (Takeoff): (0, 0, 1)")
        print("Coordinate 2: ({}, {}, {}) ".format(x_pos[0], y_pos[0], z_pos[0]))
        print("Coordinate 3: ({}, {}, {}) ".format(x_pos[1], y_pos[1], z_pos[1]))
        print("Coordinate 4: ({}, {}, {}) ".format(x_pos[2], y_pos[2], z_pos[2]))
        print("Coordinate 5 (RTH): (0, 0, 1)")
        print("Coordinate 6 (Landing): (0, 0, 0)")

        print("Takeoff.")
        with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
            time.sleep(5)
            mc.move_distance(x_pos[0], y_pos[0], z_pos[0] - 1, velocity=1.5)
            time.sleep(5)
            mc.move_distance(x_pos[1] - x_pos[0], y_pos[1] - y_pos[0], z_pos[1] - z_pos[0], velocity=1.5)
            time.sleep(5)
            mc.move_distance(x_pos[2] - x_pos[1], y_pos[2] - y_pos[1], z_pos[2] - z_pos[1], velocity=1.5)
            time.sleep(5)
            mc.move_distance(0 - x_pos[2], 0 - y_pos[2], 1 - z_pos[2], velocity=1.5)
            time.sleep(5)
            mc.stop()

        print("Touchdown.")

    else:
        print("Coordinate 1 (Takeoff): (0, 0, 1)")
        print("Coordinate 2: (0, 1, 1)")
        print("Coordinate 3: (-1, 1, 1.5)")
        print("Coordinate 4: (1, 0.5, 0.5)")
        print("Coordinate 5 (RTH): (0, 0, 1)")
        print("Coordinate 6 (Landing): (0, 0, 0)")

        print("Takeoff.")
        with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
            time.sleep(5)
            mc.move_distance(0, 1, 0, velocity=1.5)
            time.sleep(5)
            mc.move_distance(-2, 0, 0.5, velocity=1.5)
            time.sleep(5)
            mc.move_distance(2, -0.5, -1, velocity=1.5)
            time.sleep(5)
            mc.move_distance(-1, -0.5, 0.5, velocity=1.5)
            time.sleep(5)
            mc.stop()

        print("Touchdown.")

#Check if deck is attached to Crazyflie
def param_deck_flow(name, value_str):
    value = int(value_str)
    print(value)
    if value:
        deck_attached_event.set()
        print('Deck is attached!')
    else:
        print('Deck is NOT attached!')

def write_csv_log(full_csv_path, logging_dict):
    with open(full_csv_path, 'a', newline = '') as file:
        writer = csv.writer(file)

        field = ['Timestamp',
        'X-Coordinate',
        'Y-Coordinate',
        'Z-Coordinate'
        ]

        writer.writerow(field)

        for k, v in logging_dict.items():
            writer.writerow([k, v[0], v[1], v[2]])

#Log position data of drone
def drone_logging(scf, lg_stab):
    #Testing folder for logging
    project_directory = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/tests"
    full_csv_path = os.path.join(project_directory, "run9.csv")

    print(full_csv_path)

    first_time = True

    with SyncLogger(scf, lg_stab) as logger:
        end_time = time.time() + 35
        time.sleep(2)

        logging_dict = {}

        for log_entry in logger:
            if time.time() < end_time:
                print("Time: {}, Initial Time: {}".format(time.time(), end_time))
                print("(" + "Timestamp: " + str(log_entry[0]) + ", " + str(log_entry[1]['stateEstimate.x']) + ", " + str(log_entry[1]['stateEstimate.y']) + ", " + str(log_entry[1]['stateEstimate.z']) + ")")

                if first_time == True:
                    x0 = log_entry[1]['stateEstimate.x']
                    y0 = log_entry[1]['stateEstimate.y']

                    first_time = False

                send_state_to_monitor(log_entry[1]['stateEstimate.x'], x0, log_entry[1]['stateEstimate.y'], y0, log_entry[0])

                logging_dict[log_entry[0]] = (log_entry[1]['stateEstimate.x'], log_entry[1]['stateEstimate.y'], log_entry[1]['stateEstimate.z'])

            else:
                print("End time reached, stopping logging.")
                break

    write_csv_log(full_csv_path, logging_dict)

if __name__ == '__main__':
    cflib.crtp.init_drivers()

    # Initialize stream memory and memory
    drift_lib.init_stream_memory(ctypes.byref(stream_memory_instance))
    start_time = time.time()  # or use time.time() if timestamp is relative to wall clock
    drift_lib.memory_init(ctypes.byref(memory_instance), start_time)

    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache= './cache')) as scf:

        #Check if flow deck is attached
        scf.cf.param.add_update_callback(group='deck', name='bcFlow2', cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=1):
            print('No flow deck detected!')
            sys.exit(1)

        #Defining log variables
        #Drone Health
        lg_stab = LogConfig(name='Health', period_in_ms=100)
        lg_stab.add_variable('health.motorPass', 'float')
        lg_stab.add_variable('health.batteryPass', 'float')

        #Battery Life
        lg_stab = LogConfig(name='Battery', period_in_ms=100)   
        lg_stab.add_variable('pm.vbat', 'float')

        #Position
        lg_stab = LogConfig(name='Position', period_in_ms=100)
        lg_stab.add_variable('stateEstimate.x', 'float')
        lg_stab.add_variable('stateEstimate.y', 'float')
        lg_stab.add_variable('stateEstimate.z', 'float')

        #Rotation
        lg_stab = LogConfig(name='Rotation', period_in_ms=100)
        lg_stab.add_variable('stateEstimate.roll', 'float')
        lg_stab.add_variable('stateEstimate.pitch', 'float')
        lg_stab.add_variable('stateEstimate.yaw', 'float')

        t1 = threading.Thread(target=waypoint_flight, args=(scf, lg_stab, True))
        t2 = threading.Thread(target=drone_logging, args=(scf, lg_stab))

        t1.start()
        t2.start()

        t1.join()
        t2.join()

        print("Logging & flight completed.")