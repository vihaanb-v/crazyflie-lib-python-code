import ctypes
from ctypes import CDLL, POINTER, Structure, c_double, c_int64, c_bool, c_char_p, string_at
from ctypes import *


import time


drift_lib = ctypes.CDLL("/home/bitcraze/projects/crazyflie-lib-python-code/rtlola/waypoint_monitor_enforcement/libmonitor.so")


#Define all BoundedBuffer structures
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
  
class Memory(Structure):
   _fields_ = [
       ("boundedbuffer_x_drift", BoundedBuffer_double),
       ("boundedbuffer_y_drift", BoundedBuffer_double),
       ("boundedbuffer_z_drift", BoundedBuffer_double),
       ("boundedbuffer_pitch", BoundedBuffer_double),
       ("boundedbuffer_roll", BoundedBuffer_double),
       ("boundedbuffer_yaw", BoundedBuffer_double),
       ("boundedbuffer_multi_ranger_x_drift", BoundedBuffer_double),
       ("boundedbuffer_multi_ranger_y_drift", BoundedBuffer_double),
       ("boundedbuffer_multi_ranger_z_drift", BoundedBuffer_double),
       ("boundedbuffer_abs_pitch", BoundedBuffer_double),
       ("boundedbuffer_abs_roll",  BoundedBuffer_double),
       ("boundedbuffer_abs_yaw",   BoundedBuffer_double),
       ("boundedbuffer_x_drift_pos_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_0", BoundedBuffer_str),
       ("boundedbuffer_x_drift_neg_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_1", BoundedBuffer_str),
       ("boundedbuffer_y_drift_pos_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_2", BoundedBuffer_str),
       ("boundedbuffer_y_drift_neg_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_3", BoundedBuffer_str),
       ("boundedbuffer_z_drift_pos_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_4", BoundedBuffer_str),
       ("boundedbuffer_z_drift_neg_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_5", BoundedBuffer_str),
       ("boundedbuffer_multi_ranger_x_drift_pos_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_6", BoundedBuffer_str),
       ("boundedbuffer_multi_ranger_x_drift_neg_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_7", BoundedBuffer_str),
       ("boundedbuffer_multi_ranger_y_drift_pos_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_8", BoundedBuffer_str),
       ("boundedbuffer_multi_ranger_y_drift_neg_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_9", BoundedBuffer_str),
       ("boundedbuffer_multi_ranger_z_drift_pos_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_10", BoundedBuffer_str),
       ("boundedbuffer_multi_ranger_z_drift_neg_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_11", BoundedBuffer_str),
       ("boundedbuffer_pitch_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_12", BoundedBuffer_str),
       ("boundedbuffer_roll_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_13", BoundedBuffer_str),
       ("boundedbuffer_yaw_exceeded", BoundedBuffer_bool),
       ("boundedbuffer_trigger_14", BoundedBuffer_str),
       ("time", c_double),
   ]


class InternalEvent(Structure):
   _fields_ = [
       ("x_drift", c_double), ("x_drift_is_present", c_bool),
       ("y_drift", c_double), ("y_drift_is_present", c_bool),
       ("z_drift", c_double), ("z_drift_is_present", c_bool),
       ("pitch", c_double), ("pitch_is_present", c_bool),
       ("roll", c_double), ("roll_is_present", c_bool),
       ("yaw", c_double), ("yaw_is_present", c_bool),
       ("multi_ranger_x_drift", c_double), ("multi_ranger_x_drift_is_present", c_bool),
       ("multi_ranger_y_drift", c_double), ("multi_ranger_y_drift_is_present", c_bool),
       ("multi_ranger_z_drift", c_double), ("multi_ranger_z_drift_is_present", c_bool),
       ("time", c_double),
   ]


class Verdict(Structure):
   _fields_ = [
       ("abs_pitch", c_double), ("abs_pitch_is_present", c_bool),
       ("abs_roll",  c_double), ("abs_roll_is_present",  c_bool),
       ("abs_yaw",   c_double), ("abs_yaw_is_present",   c_bool),
       ("x_drift_pos_exceeded", c_bool), ("x_drift_pos_exceeded_is_present", c_bool),
       ("trigger_0", c_char_p), ("trigger_0_is_present", c_bool),
       ("x_drift_neg_exceeded", c_bool), ("x_drift_neg_exceeded_is_present", c_bool),
       ("trigger_1", c_char_p), ("trigger_1_is_present", c_bool),
       ("y_drift_pos_exceeded", c_bool), ("y_drift_pos_exceeded_is_present", c_bool),
       ("trigger_2", c_char_p), ("trigger_2_is_present", c_bool),
       ("y_drift_neg_exceeded", c_bool), ("y_drift_neg_exceeded_is_present", c_bool),
       ("trigger_3", c_char_p), ("trigger_3_is_present", c_bool),
       ("z_drift_pos_exceeded", c_bool), ("z_drift_pos_exceeded_is_present", c_bool),
       ("trigger_4", c_char_p), ("trigger_4_is_present", c_bool),
       ("z_drift_neg_exceeded", c_bool), ("z_drift_neg_exceeded_is_present", c_bool),
       ("trigger_5", c_char_p), ("trigger_5_is_present", c_bool),
       ("multi_ranger_x_drift_pos_exceeded", c_bool), ("multi_ranger_x_drift_pos_exceeded_is_present", c_bool),
       ("trigger_6", c_char_p), ("trigger_6_is_present", c_bool),
       ("multi_ranger_x_drift_neg_exceeded", c_bool), ("multi_ranger_x_drift_neg_exceeded_is_present", c_bool),
       ("trigger_7", c_char_p), ("trigger_7_is_present", c_bool),
       ("multi_ranger_y_drift_pos_exceeded", c_bool), ("multi_ranger_y_drift_pos_exceeded_is_present", c_bool),
       ("trigger_8", c_char_p), ("trigger_8_is_present", c_bool),
       ("multi_ranger_y_drift_neg_exceeded", c_bool), ("multi_ranger_y_drift_neg_exceeded_is_present", c_bool),
       ("trigger_9", c_char_p), ("trigger_9_is_present", c_bool),
       ("multi_ranger_z_drift_pos_exceeded", c_bool), ("multi_ranger_z_drift_pos_exceeded_is_present", c_bool),
       ("trigger_10", c_char_p), ("trigger_10_is_present", c_bool),
       ("multi_ranger_z_drift_neg_exceeded", c_bool), ("multi_ranger_z_drift_neg_exceeded_is_present", c_bool),
       ("trigger_11", c_char_p), ("trigger_11_is_present", c_bool),
       ("pitch_exceeded", c_bool), ("pitch_exceeded_is_present", c_bool),
       ("trigger_12", c_char_p), ("trigger_12_is_present", c_bool),
       ("roll_exceeded", c_bool), ("roll_exceeded_is_present", c_bool),
       ("trigger_13", c_char_p), ("trigger_13_is_present", c_bool),
       ("yaw_exceeded", c_bool), ("yaw_exceeded_is_present", c_bool),
       ("trigger_14", c_char_p), ("trigger_14_is_present", c_bool),
       ("time", c_double),
   ]


# Binding function signature
drift_lib.cycle.restype = Verdict
drift_lib.cycle.argtypes = [POINTER(Memory), InternalEvent]


# Event constructor
def create_event(x_drift=None, y_drift=None, z_drift=None,
                pitch=None, roll=None, yaw=None,
                multi_ranger_x_drift=None, multi_ranger_y_drift=None, multi_ranger_z_drift=None,
                time_val=0.0):
   e = InternalEvent()


   def set_field(value, name):
       if value is None:
           setattr(e, name, 0.0)                       # safe default
           setattr(e, f"{name}_is_present", False)
       else:
           setattr(e, name, float(value))
           setattr(e, f"{name}_is_present", True)


   set_field(x_drift, 'x_drift')
   set_field(y_drift, 'y_drift')
   set_field(z_drift, 'z_drift')
   set_field(pitch, 'pitch')
   set_field(roll, 'roll')
   set_field(yaw, 'yaw')
   set_field(multi_ranger_x_drift, 'multi_ranger_x_drift')
   set_field(multi_ranger_y_drift, 'multi_ranger_y_drift')
   set_field(multi_ranger_z_drift, 'multi_ranger_z_drift')


   e.time = float(time_val)
   return e


def display_verdict_triggers(verdict):
   TRIGGER_COUNT = 15
   for i in range(TRIGGER_COUNT):
       trigger_field = f"trigger_{i}"
       present_field = f"{trigger_field}_is_present"
       if getattr(verdict, present_field, False):
           ptr = getattr(verdict, trigger_field, None)
           if ptr:
               msg = string_at(ptr).decode(errors="replace")
               print(f"[TRIGGER] {msg}")


import logging
import sys
import threading
from threading import Timer
from threading import Event
from threading import Lock
from queue import Queue


import cflib.crtp
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie
from cflib.crazyflie.syncLogger import SyncLogger
from cflib.positioning.motion_commander import MotionCommander
from cflib.crazyflie.log import LogConfig
from cflib.utils import uri_helper


import numpy as np
import random
import os
import csv
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from queue import Queue, Empty


URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')


DEFAULT_HEIGHT = 1.5


flow_deck_event = Event()
multiranger_deck_event = Event()


logging.basicConfig(level=logging.ERROR)


memory_instance = Memory()


takeoff_started = threading.Event()
takeoff_ended = threading.Event()


position_ready = threading.Event()


mx_right_ready = threading.Event()
mx_left_ready = threading.Event()


#my_front_ready = threading.Event()
my_back_ready = threading.Event()


hover_established = threading.Event()


autocorrect_position_x_neg = threading.Event()
autocorrect_position_x_pos = threading.Event()


autocorrect_position_y_neg = threading.Event()
autocorrect_position_y_pos = threading.Event()


autocorrect_position_z_neg = threading.Event()
autocorrect_position_z_pos = threading.Event()


corrections_q = Queue()


def send_state_to_monitor(x_val, x0, y_val, y0, mx_val, mx0, my_val, my0, mz_val, mz0, timestamp):
   x_drift_val = x_val - x0
   y_drift_val = y_val - y0
   mx_drift_val = mx_val - mx0
   my_drift_val = my_val - my0
   mz_drift_val = mz_val - mz0


   event = create_event(
       x_drift=x_drift_val,
       y_drift=y_drift_val,
       z_drift=mz_drift_val,
       multi_ranger_x_drift=mx_drift_val,
       multi_ranger_y_drift=my_drift_val,
       multi_ranger_z_drift=mz_drift_val,
       time_val=timestamp / 1000.0
   )


   verdict = drift_lib.cycle(ctypes.byref(memory_instance), event)
   display_verdict_triggers(verdict)


def param_deck_flow(name, value_str):
   value = int(value_str)
   print(f"[Flow Deck Parameter] {name} = {value}")
   if value:
       flow_deck_event.set()
   else:
       print("[EXIT] Flow deck v2 not detected. Aborting.")


def param_deck_multi_ranger(name, value_str):
   value = int(value_str)
   print(f"[Multiranger Parameter] {name} = {value}")
   if value:
       multiranger_deck_event.set()
       print('Multi-ranger deck detected!')
   else:
       print("[EXIT] Multi-ranger deck not detected. Aborting.")


def take_off_simple(scf):
   print("Takeoff.")
  
   with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
       time.sleep(10)
       mc.stop()


   print("Touchdown.")
  
#Waypoint flight
def waypoint_flight(scf, rnd):
   if rnd == True:
       x_pos = []
       y_pos = []
       z_pos = []


       for i in range(3):
           start = 0.4


           end_x = 2.6
           end_y = 2
           end_z = 1.2


           step = 0.2


           val_x = random.randint(0, int((end_x - start) / step))
           val_y = random.randint(0, int((end_y - start) / step))
           val_z = random.randint(0, int((end_z - start) / step))


           x_pos.append(start + val_x * step)
           y_pos.append(start + val_y * step)
           z_pos.append(start + val_z * step)


       print("Coordinate 1 (Takeoff): (0, 0, 0)")
       print("Coordinate 2 (Hover): (0, 0, 1)")
       print("Coordinate 3: ({}, {}, {}) ".format(x_pos[0], y_pos[0], z_pos[0]))
       print("Coordinate 4: ({}, {}, {}) ".format(x_pos[1], y_pos[1], z_pos[1]))
       print("Coordinate 5: ({}, {}, {}) ".format(x_pos[2], y_pos[2], z_pos[2]))
       print("Coordinate 6 (RTH): (0, 0, 1)")
       print("Coordinate 7 (Landing): (0, 0, 0)")


       print("Takeoff.")
      
       takeoff_started.set()
      
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


       return [(0, 0, 0), (0, 0, 1)] + list(zip(x_pos, y_pos, z_pos))


   else:
       print("Coordinate 1 (Takeoff): (0, 0, 0)")
       print("Coordinate 2 (Hover): (0, 0, 1.5)")
       print("Coordinate 3: (-0.6, 0.6, 1.2)")
       print("Coordinate 4: (0.8, -0.4, 0.6)")
       print("Coordinate 5: (0.3, 0.5, 1.0)")
       print("Coordinate 6 (RTH): (0, 0, 1.5)")
       print("Coordinate 7 (Landing): (0, 0, 0)")


       print("Takeoff.")


       takeoff_started.set()


       with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
           time.sleep(5)
           mc.move_distance(-0.6, 0.6, -0.3, velocity=1.0)
           time.sleep(5)
           mc.move_distance(1.4, -1.0, -0.6, velocity=1.0)
           time.sleep(5)
           mc.move_distance(-0.5, 0.9, 0.4, velocity=1.0)
           time.sleep(5)
           mc.move_distance(-0.3, -0.5, 0.5, velocity=1.0)
           time.sleep(5)
           mc.stop()


       print("Touchdown.")


def square_turns_starting_at_corner(scf, velocity):
   print("Coordinate 1 (Takeoff): (0, 0, 0)")
   print("Coordinate 2 (Hover): (0, 0, 1.5)")
   print("Coordinate 3: (0, 1.2, 1.5)")
   print("Coordinate 4: (1.2, 1.2, 1.5)")
   print("Coordinate 5: (1.2, 0, 1.5)")
   print("Coordinate 6 (RTH): (0, 0, 1.5)")
   print("Coordinate 7 (Landing): (0, 0, 0)")


   print("Takeoff.")


   takeoff_started.set()
  
   with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
       #Code using turns from bottom right corner of square
       time.sleep(3)
       mc.move_distance(1.2, 0, 0, velocity=velocity)
       time.sleep(3)
       mc.move_distance(0, -1.2, 0, velocity=velocity)
       time.sleep(3)
       mc.move_distance(-1.2, 0, 0, velocity=velocity)
       time.sleep(3)
       mc.move_distance(0, 1.2, 0, velocity=velocity)
       time.sleep(3)
       mc.stop()
      
   print("Touchdown.")


def combined_flight(scf, position_lock, shared_position):
    print("Coordinate 1 (Takeoff): (0, 0, 0)")
    print("Coordinate 2 (Hover): (0, 0, 1.5)")
    print("Coordinate 3: (0, 1.2, 1.5)")
    print("Coordinate 4: (1.2, 1.2, 1.5)")
    print("Coordinate 5: (1.2, 0, 1.5)")
    print("Coordinate 6 (RTH): (0, 0, 1.5)")
    print("Coordinate 7 (Landing): (0, 0, 0)")

    print("Takeoff.")
    takeoff_started.set()

    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        position_ready.wait()
        
        time.sleep(3)

        with position_lock:
            mx = shared_position["mx"]
            my = shared_position["my"]

        if mx > 0.2:
            print("[INIT-AUTO-X] Initial X displacement too large. Correcting to 0.")
            correction_time = abs(mx / 0.2) + 0.7
            mc.start_linear_motion(0.0, -0.2, 0.0)
            time.sleep(correction_time)
            mc.stop()
            time.sleep(1)

        elif mx < -0.2:
            print("[INIT-AUTO-X] Initial X displacement too large. Correcting to 0.")
            correction_time = abs(mx / 0.2) + 0.7
            mc.start_linear_motion(0.0, 0.2, 0.0)
            time.sleep(correction_time)
            mc.stop()
            time.sleep(1)

        if my > 0.2:
            print("[INIT-AUTO-Y] Initial Y displacement too large. Correcting to 0.")
            correction_time = abs(my / 0.2) + 0.7
            mc.start_linear_motion(-0.2, 0.0, 0.0)
            time.sleep(correction_time)
            mc.stop()
            time.sleep(1)

        elif my < -0.2:
            print("[INIT-AUTO-Y] Initial Y displacement too large. Correcting to 0.")
            correction_time = abs(my / 0.2) + 0.7
            mc.start_linear_motion(0.2, 0.0, 0.0)
            time.sleep(correction_time)
            mc.stop()
            time.sleep(1)
        
        # SEGMENT 1
        mc.start_linear_motion(0.2, 0.0, 0.0)
        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()
        while True:
            if autocorrect_position_x_neg.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(0.2, 0.0, 0.0)

            elif autocorrect_position_x_pos.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(0.2, 0.0, 0.0)

            elif autocorrect_position_y_neg.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(0.2, 0.0, 0.0)

            elif autocorrect_position_y_pos.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(0.2, 0.0, 0.0)

            with position_lock:
                my = shared_position["my"]
            if my >= 1.18:
                mc.stop()
                time.sleep(3)
                break
            time.sleep(0.01)

        mx_right_ready.set()
        mc.stop()

        time_temp = time.time()
        time_end = time_temp + 2

        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()
        while time.time() < time_end:
            if autocorrect_position_x_neg.is_set():
                time_end += 2.1
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_x_pos.is_set():
                time_end += 2.1
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_y_neg.is_set():
                time_end += 2.1
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_y_pos.is_set():
                time_end += 2.1
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time_end += 2.1
                time.sleep(2)

            time.sleep(0.01)

        # SEGMENT 2
        mc.start_linear_motion(0.0, -0.2, 0.0)
        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()
        while True:
            if autocorrect_position_x_neg.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, -0.2, 0.0)

            elif autocorrect_position_x_pos.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, -0.2, 0.0)

            elif autocorrect_position_y_neg.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, -0.2, 0.0)

            elif autocorrect_position_y_pos.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, -0.2, 0.0)

            with position_lock:
                mx = shared_position["mx"]
            if mx >= 1.18:
                mc.stop()
                time.sleep(3)
                break
            time.sleep(0.01)

        my_back_ready.set()
        mc.stop()
        
        time_temp = time.time()
        time_end = time_temp + 2

        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()

        while time.time() < time_end:
            if autocorrect_position_x_neg.is_set():
                time_end += 2.1
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_x_pos.is_set():
                time_end += 2.1
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_y_neg.is_set():
                time_end += 2.1
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_y_pos.is_set():
                time_end += 2.1
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time_end += 2.1
                time.sleep(2)

            time.sleep(0.01)

        # SEGMENT 3
        mc.start_linear_motion(-0.2, 0.0, 0.0)
        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()
        while True:
            if autocorrect_position_x_neg.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(-0.2, 0.0, 0.0)

            elif autocorrect_position_x_pos.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(-0.2, 0.0, 0.0)

            elif autocorrect_position_y_neg.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(-0.2, 0.0, 0.0)

            elif autocorrect_position_y_pos.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(-0.2, 0.0, 0.0)

            with position_lock:
                my = shared_position["my"]
            if my <= 0.02:
                mc.stop()
                time.sleep(3)
                break
            time.sleep(0.01)

        mx_left_ready.set()
        mc.stop()
        
        time_temp = time.time()
        time_end = time_temp + 2

        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()

        while time.time() < time_end:
            if autocorrect_position_x_neg.is_set():
                time_end += 2.1
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_x_pos.is_set():
                time_end += 2.1
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_y_neg.is_set():
                time_end += 2.1
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time_end += 2.1
                time.sleep(2)

            elif autocorrect_position_y_pos.is_set():
                time_end += 2.1
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time_end += correction_time + 0.1
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time_end += 2.1
                time.sleep(2)

            time.sleep(0.01)

        # SEGMENT 4
        mc.start_linear_motion(0.0, 0.2, 0.0)
        autocorrect_position_x_neg.clear()
        autocorrect_position_x_pos.clear()
        autocorrect_position_y_neg.clear()
        autocorrect_position_y_pos.clear()
        while True:
            if autocorrect_position_x_neg.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, -0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, 0.2, 0.0)

            elif autocorrect_position_x_pos.is_set():
                print("[AUTO-X] X drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmx = shared_position["dmx"]
                correction_time = abs(dmx / 0.2) + 0.7
                mc.start_linear_motion(0.0, 0.2, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_x_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, 0.2, 0.0)

            elif autocorrect_position_y_neg.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_neg.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, 0.2, 0.0)

            elif autocorrect_position_y_pos.is_set():
                print("[AUTO-Y] Y drift trigger fired. Pausing flight.")
                mc.stop()
                time.sleep(2)
                with position_lock:
                    dmy = shared_position["dmy"]
                correction_time = abs(dmy / 0.2) + 0.7
                mc.start_linear_motion(-0.2, 0.0, 0.0)
                time.sleep(correction_time)
                mc.stop()
                autocorrect_position_y_pos.clear()
                time.sleep(2)
                mc.start_linear_motion(0.0, 0.2, 0.0)

            with position_lock:
                mx = shared_position["mx"]
            if mx <= 0.02:
                mc.stop()
                time.sleep(3)
                break
            time.sleep(0.01)

        takeoff_ended.set()

    print("Touchdown.")


def write_state_csv_log(full_csv_path_log, log_dict):
   headers = [
       "timestamp", "x", "y", "z",
       "x_drift", "y_drift", "z_drift",
       "roll", "pitch", "yaw",
       "roll_drift", "pitch_drift", "yaw_drift",
       "x_drift_pos_exceeded", "x_drift_pos_trigger",
       "x_drift_neg_exceeded", "x_drift_neg_trigger",
       "y_drift_pos_exceeded", "y_drift_pos_trigger",
       "y_drift_neg_exceeded", "y_drift_neg_trigger",
       "z_drift_pos_exceeded", "z_drift_pos_trigger",
       "z_drift_neg_exceeded", "z_drift_neg_trigger",
       "pitch_exceeded", "pitch_trigger",
       "roll_exceeded", "roll_trigger",
       "yaw_exceeded", "yaw_trigger",
   ]


   with open(full_csv_path_log, 'w', newline='') as f:
       writer = csv.writer(f)
       writer.writerow(headers)
       for ts in sorted(log_dict.keys()):
           row = [log_dict[ts].get(col, "") for col in headers]
           writer.writerow(row)


   print(f"[State CSV] Saved to: {full_csv_path_log}")


def write_ranger_csv_log(full_csv_path_log, log_dict):
   headers = [
       "timestamp", "mx", "my", "mz",
       "multi_ranger_x_drift", "multi_ranger_y_drift", "multi_ranger_z_drift",
       "multi_ranger_x_drift_pos_exceeded", "multi_ranger_x_drift_pos_trigger",
       "multi_ranger_x_drift_neg_exceeded", "multi_ranger_x_drift_neg_trigger",
       "multi_ranger_y_drift_pos_exceeded", "multi_ranger_y_drift_pos_trigger",
       "multi_ranger_y_drift_neg_exceeded", "multi_ranger_y_drift_neg_trigger",
       "multi_ranger_z_drift_pos_exceeded", "multi_ranger_z_drift_pos_trigger",
       "multi_ranger_z_drift_neg_exceeded", "multi_ranger_z_drift_neg_trigger",
   ]


   with open(full_csv_path_log, 'w', newline='') as f:
       writer = csv.writer(f)
       writer.writerow(headers)
       for ts in sorted(log_dict.keys()):
           row = [log_dict[ts].get(col, "") for col in headers]
           writer.writerow(row)


   print(f"[Multi-Ranger CSV] Saved to: {full_csv_path_log}")


def graph_state_estimate_drift(project_directory_plot, logging_rows, run_id):
   import matplotlib.pyplot as plt
   import os


   timestamps = [row["timestamp"] for row in logging_rows]
   time_seconds = [t / 1000.0 for t in timestamps]


   x_drift = [row['x_drift'] for row in logging_rows]
   y_drift = [row['y_drift'] for row in logging_rows]
   z_drift = [row['z_drift'] for row in logging_rows]


   roll_drift = [row['roll_drift'] for row in logging_rows]
   pitch_drift = [row['pitch_drift'] for row in logging_rows]
   yaw_drift = [row['yaw_drift'] for row in logging_rows]


   # Positional Drift Plot
   plt.figure(figsize=(12, 6))
   plt.plot(time_seconds, x_drift, label='X Drift', linewidth=2, color='blue')
   plt.plot(time_seconds, y_drift, label='Y Drift', linewidth=2, color='red')
   plt.plot(time_seconds, z_drift, label='Z Drift', linewidth=2, color='green')
   plt.title('Drone State Estimate Positional Drift vs. Time')
   plt.xlabel('Time (seconds)')
   plt.ylabel('Drift (meters)')
   plt.legend()
   plt.grid(True)
   plt.tight_layout()
   pos_path = os.path.join(project_directory_plot, f"{run_id}_position_drift.png")
   plt.savefig(pos_path, dpi=300)
   plt.close()


   # Orientation Drift Plot
   plt.figure(figsize=(12, 6))
   plt.plot(time_seconds, roll_drift, label='Roll Drift', linewidth=2, color='blue')
   plt.plot(time_seconds, pitch_drift, label='Pitch Drift', linewidth=2, color='red')
   plt.plot(time_seconds, yaw_drift, label='Yaw Drift', linewidth=2, color='green')
   plt.title('Drone Orientation Drift vs. Time')
   plt.xlabel('Time (seconds)')
   plt.ylabel('Drift (radians)')
   plt.legend()
   plt.grid(True)
   plt.tight_layout()
   orient_path = os.path.join(project_directory_plot, f"{run_id}_orientation_drift.png")
   plt.savefig(orient_path, dpi=300)
   plt.close()


   print(f"Saved state estimate plots to:\n{pos_path}\n{orient_path}")


def graph_multiranger_drift(project_directory_plot, ranger_rows, run_id):
   import matplotlib.pyplot as plt
   import os


   timestamps = [row["timestamp"] for row in ranger_rows]
   time_seconds = [t / 1000.0 for t in timestamps]


   mx_drift = [row['multi_ranger_x_drift'] for row in ranger_rows]
   my_drift = [row['multi_ranger_y_drift'] for row in ranger_rows]
   mz_drift = [row['multi_ranger_z_drift'] for row in ranger_rows]


   plt.figure(figsize=(12, 6))
   plt.plot(time_seconds, mx_drift, label='X Drift', linewidth=2, color='blue')
   plt.plot(time_seconds, my_drift, label='Y Drift', linewidth=2, color='red')
   plt.plot(time_seconds, mz_drift, label='Z Drift', linewidth=2, color='green')
   plt.title('Drone Multi-Ranger Positional Drift vs. Time')
   plt.xlabel('Time (seconds)')
   plt.ylabel('Drift (meters)')
   plt.legend()
   plt.grid(True)
   plt.tight_layout()
   path = os.path.join(project_directory_plot, f"{run_id}_multi_ranger_drift.png")
   plt.savefig(path, dpi=300)
   plt.close()


   print(f"Saved multiranger drift plot to:\n{path}")


# For random waypoint flights & square turns flight from corners
#'''
def graph_3d_state_estimate_vs_ideal(project_directory_plot, logging_rows, ideal_coords, run_id):
   # Convert stateEstimate logs to real-world coordinates
   x_actual = [-row["y"] for row in logging_rows]  # State y → -x (real)
   y_actual = [row["x"] for row in logging_rows]   # State x → +y (real)
   z_actual = [row["z"] for row in logging_rows]   # Z stays the same


   # Ideal coordinates are already in real-world space
   x_ideal, y_ideal, z_ideal = zip(*ideal_coords[:5])
   print({"x_ideal": x_ideal, "y_ideal": y_ideal, "z_ideal": z_ideal})


   # Close the loop for ideal path visualization
   x_ideal_loop = list(x_ideal) + [x_ideal[1]]
   y_ideal_loop = list(y_ideal) + [y_ideal[1]]
   z_ideal_loop = list(z_ideal) + [z_ideal[1]]


   fig = plt.figure(figsize=(18, 14))
   ax = fig.add_subplot(111, projection='3d')


   ax.plot(x_actual, y_actual, z_actual, label='State Estimate Flight Path', color='blue', linewidth=2)
   ax.plot(x_ideal_loop, y_ideal_loop, z_ideal_loop, label='Ideal Waypoints Path', color='green', linestyle='--', linewidth=2)
   ax.scatter(x_ideal, y_ideal, z_ideal, color='black', s=80, marker='x')


   labels = ["Takeoff / Landing", "Hover / Return-to-Home", "Coord 1", "Coord 2", "Coord 3"]
   for i, label in enumerate(labels):
       ax.text(x_ideal[i], y_ideal[i], z_ideal[i] + 0.08, label, fontsize=14, color='black', weight='bold')


   ax.set_title("3D Flight Visualization: State Estimate Flight Path vs. Ideal", fontsize=18)
   ax.set_xlabel("X (m)", fontsize=14)
   ax.set_ylabel("Y (m)", fontsize=14)
   ax.set_zlabel("Z (m)", fontsize=14)
   ax.legend(fontsize=12)
   ax.grid(True)
   ax.view_init(elev=25, azim=135)


   ax.set_box_aspect([1, 1, 1])  # Equal scaling for x, y, z
   ax.set_zlim(0, 2)  # Expand vertical axis to ensure Z=1.5 is correctly rendered


   path = os.path.join(project_directory_plot, f"{run_id}_3d_state_estimate_vs_ideal.svg")
   plt.tight_layout()
   plt.savefig(path, format='svg')
   plt.close()
   print(f"Saved state estimate vs. ideal trajectory plot to: {path}")
#'''


# For square turns flights from center
'''
def graph_3d_state_estimate_vs_ideal(project_directory_plot, logging_rows, ideal_coords, run_id):
   x_actual = [row["x"] for row in logging_rows]
   y_actual = [row["y"] for row in logging_rows]
   z_actual = [row["z"] for row in logging_rows]


   # Unpack full ideal coordinates
   x_ideal, y_ideal, z_ideal = zip(*ideal_coords)


   # Create looped ideal path: go back to RTH and then land
   x_ideal_loop = list(x_ideal) + [x_ideal[1], x_ideal[0]]
   y_ideal_loop = list(y_ideal) + [y_ideal[1], y_ideal[0]]
   z_ideal_loop = list(z_ideal) + [z_ideal[1], z_ideal[0]]


   fig = plt.figure(figsize=(18, 14))
   ax = fig.add_subplot(111, projection='3d')


   # Plot state estimate flight path
   ax.plot(x_actual, y_actual, z_actual, label='State Estimate Flight Path', color='blue', linewidth=2)


   # Plot ideal waypoint path
   ax.plot(x_ideal_loop, y_ideal_loop, z_ideal_loop, label='Ideal Waypoints Path', color='green', linestyle='--', linewidth=2)


   # Scatter and annotate ideal coordinates
   ax.scatter(x_ideal, y_ideal, z_ideal, color='black', s=80, marker='x')


   labels = [
       "Start / Land (0,0,0)",
       "Hover / RTH (0,0,1.5)",
       "Coord 1 (0,0.6,1.5)",
       "Coord 2 (-0.6,0.6,1.5)",
       "Coord 3 (-0.6,-0.6,1.5)",
       "Coord 4 (0.6,-0.6,1.5)",
       "Coord 5 (0.6,0.6,1.5)"
   ]


   for i, label in enumerate(labels):
       ax.text(x_ideal[i], y_ideal[i], z_ideal[i] + 0.08, label, fontsize=12, color='black', weight='bold')


   ax.set_title("3D Flight Visualization: State Estimate Flight Path vs. Ideal", fontsize=18)
   ax.set_xlabel("X (m)", fontsize=14)
   ax.set_ylabel("Y (m)", fontsize=14)
   ax.set_zlabel("Z (m)", fontsize=14)
   ax.legend(fontsize=12)
   ax.grid(True)
   ax.view_init(elev=25, azim=135)


   path = os.path.join(project_directory_plot, f"{run_id}_3d_state_estimate_vs_ideal.svg")
   plt.tight_layout()
   plt.savefig(path, format='svg')
   plt.close()
   print(f"Saved state estimate vs. ideal trajectory plot to: {path}")
'''


# For random waypoint flights & square turns flight from corners
# '''
def graph_3d_multiranger_vs_ideal(project_directory_plot, multiranger_rows, ideal_coords, run_id):
   x_multi = [row["mx"] for row in multiranger_rows]
   y_multi = [row["my"] for row in multiranger_rows]
   z_multi = [row["mz"] for row in multiranger_rows]


   x_ideal, y_ideal, z_ideal = zip(*ideal_coords[:5])


   x_ideal_loop = list(x_ideal) + [x_ideal[1]]
   y_ideal_loop = list(y_ideal) + [y_ideal[1]]
   z_ideal_loop = list(z_ideal) + [z_ideal[1]]


   fig = plt.figure(figsize=(18, 14))
   ax = fig.add_subplot(111, projection='3d')


   ax.plot(x_multi, y_multi, z_multi, label='Multi-Ranger Flight Path', color='red', linewidth=2)
   ax.plot(x_ideal_loop, y_ideal_loop, z_ideal_loop, label='Ideal Waypoints Path', color='green', linestyle='--', linewidth=2)
   ax.scatter(x_ideal, y_ideal, z_ideal, color='black', s=80, marker='x')


   labels = ["Takeoff / Landing", "Hover / Return-to-Home", "Coordinate 1", "Coordinate 2", "Coordinate 3"]
   for i, label in enumerate(labels):
       ax.text(x_ideal[i], y_ideal[i], z_ideal[i] + 0.08, label, fontsize=14, color='black', weight='bold')


   ax.set_title("3D Visualization: Multi-Ranger Flight Path vs. Ideal", fontsize=18)
   ax.set_xlabel("X (m)", fontsize=14)
   ax.set_ylabel("Y (m)", fontsize=14)
   ax.set_zlabel("Z (m)", fontsize=14)
   ax.legend(fontsize=12)
   ax.grid(True)
   ax.view_init(elev=25, azim=135)


   path = os.path.join(project_directory_plot, f"{run_id}_3d_multiranger_vs_ideal.svg")
   plt.tight_layout()
   plt.savefig(path, format='svg')
   plt.close()
   print(f"Saved multiranger vs. ideal trajectory plot to: {path}")
# '''


# For square turns flights from center
'''
def graph_3d_multiranger_vs_ideal(project_directory_plot, multiranger_rows, ideal_coords, run_id):
   import matplotlib.pyplot as plt
   from mpl_toolkits.mplot3d import Axes3D
   import os


   x_multi = [row["mx"] for row in multiranger_rows]
   y_multi = [row["my"] for row in multiranger_rows]
   z_multi = [row["mz"] for row in multiranger_rows]


   # Unpack all ideal coordinates
   x_ideal, y_ideal, z_ideal = zip(*ideal_coords)
  
   # Loop the ideal path visually by adding the first hover again and finally landing
   x_ideal_loop = list(x_ideal) + [x_ideal[1], x_ideal[0]]
   y_ideal_loop = list(y_ideal) + [y_ideal[1], y_ideal[0]]
   z_ideal_loop = list(z_ideal) + [z_ideal[1], z_ideal[0]]


   fig = plt.figure(figsize=(18, 14))
   ax = fig.add_subplot(111, projection='3d')


   # Plot multiranger-estimated path
   ax.plot(x_multi, y_multi, z_multi, label='Multi-Ranger Flight Path', color='red', linewidth=2)


   # Plot ideal waypoint path
   ax.plot(x_ideal_loop, y_ideal_loop, z_ideal_loop, label='Ideal Waypoints Path', color='green', linestyle='--', linewidth=2)


   # Mark each ideal coordinate
   ax.scatter(x_ideal, y_ideal, z_ideal, color='black', s=80, marker='x')


   labels = [
       "Start / Land (0,0,0)",
       "Hover / RTH (0,0,1.5)",
       "Coord 1 (0,0.6,1.5)",
       "Coord 2 (-0.6,0.6,1.5)",
       "Coord 3 (-0.6,-0.6,1.5)",
       "Coord 4 (0.6,-0.6,1.5)",
       "Coord 5 (0.6,0.6,1.5)"
   ]
  
   for i, label in enumerate(labels):
       ax.text(x_ideal[i], y_ideal[i], z_ideal[i] + 0.08, label, fontsize=12, color='black', weight='bold')


   ax.set_title("3D Visualization: Multi-Ranger Flight Path vs. Ideal", fontsize=18)
   ax.set_xlabel("X (m)", fontsize=14)
   ax.set_ylabel("Y (m)", fontsize=14)
   ax.set_zlabel("Z (m)", fontsize=14)
   ax.legend(fontsize=12)
   ax.grid(True)
   ax.view_init(elev=25, azim=135)


   path = os.path.join(project_directory_plot, f"{run_id}_3d_multiranger_vs_ideal.svg")
   plt.tight_layout()
   plt.savefig(path, format='svg')
   plt.close()
   print(f"Saved multiranger vs. ideal trajectory plot to: {path}")
'''


def plot_state_estimate_xyz_vs_time(state_rows, run_id, save_path):
   times = [row["timestamp"] / 1000.0 for row in state_rows]
   x_vals = [row["x"] for row in state_rows]
   y_vals = [row["y"] for row in state_rows]
   z_vals = [row["z"] for row in state_rows]


   plt.figure(figsize=(14, 8))
   plt.plot(times, x_vals, label='X (mx)', color='red', linewidth=2)
   plt.plot(times, y_vals, label='Y (my)', color='green', linewidth=2)
   plt.plot(times, z_vals, label='Z (mz)', color='blue', linewidth=2)


   plt.title(f'State Estimate X/Y/Z vs. Time — {run_id}', fontsize=16)
   plt.xlabel('Time (s)', fontsize=14)
   plt.ylabel('Distance (m)', fontsize=14)
   plt.legend()
   plt.grid(True)
   plt.tight_layout()


   plt.savefig(save_path, dpi=300)


   print(f"Saved plot to {save_path}")


def plot_multiranger_xyz_vs_time(ranger_rows, run_id, save_path):
   times = [row["timestamp"] / 1000.0 for row in ranger_rows]
   x_vals = [row["mx"] for row in ranger_rows]
   y_vals = [row["my"] for row in ranger_rows]
   z_vals = [row["mz"] for row in ranger_rows]


   plt.figure(figsize=(14, 8))
   plt.plot(times, x_vals, label='X (mx)', color='red', linewidth=2)
   plt.plot(times, y_vals, label='Y (my)', color='green', linewidth=2)
   plt.plot(times, z_vals, label='Z (mz)', color='blue', linewidth=2)


   plt.title(f'MultiRanger X/Y/Z vs. Time — {run_id}', fontsize=16)
   plt.xlabel('Time (s)', fontsize=14)
   plt.ylabel('Distance (m)', fontsize=14)
   plt.legend()
   plt.grid(True)
   plt.tight_layout()


   plt.savefig(save_path, dpi=300)


   print(f"Saved plot to {save_path}")


def plot_muliranger_raw_xyz_vs_time(ranger_rows, run_id, save_path):
   times = [row["timestamp"] / 1000.0 for row in ranger_rows]
   x_vals = [row["right"] for row in ranger_rows]
   y_vals = [row["front"] for row in ranger_rows]
   z_vals = [row["up"] for row in ranger_rows]


   plt.figure(figsize=(14, 8))
   plt.plot(times, x_vals, label='X (mx)', color='red', linewidth=2)
   plt.plot(times, y_vals, label='Y (my)', color='green', linewidth=2)
   plt.plot(times, z_vals, label='Z (mz)', color='blue', linewidth=2)


   plt.title(f'MultiRanger X/Y/Z vs. Time — {run_id}', fontsize=16)
   plt.xlabel('Time (s)', fontsize=14)
   plt.ylabel('Distance (m)', fontsize=14)
   plt.legend()
   plt.grid(True)
   plt.tight_layout()


   plt.savefig(save_path, dpi=300)


   print(f"Saved plot to {save_path}")


def closest_point_on_segment(p, a, b):
   p = np.array(p)
   a = np.array(a)
   b = np.array(b)
   ab = b - a
   ap = p - a
   ab_len_sq = np.dot(ab, ab)


   if ab_len_sq == 0:
       return a  # Segment is a point
   t = np.dot(ap, ab) / ab_len_sq
   t = max(0, min(1, t))  # Clamp to segment
   return a + t * ab


def compute_drift_from_path(pos, waypoints):
   # Return drift vector (dx, dy, dz) from pos to nearest path segment.
   min_dist = float('inf')
   best_drift = (0.0, 0.0, 0.0)


   for i in range(len(waypoints) - 1):
       a = waypoints[i]
       b = waypoints[i + 1]
       closest = closest_point_on_segment(pos, a, b)
       drift_vec = np.array(pos) - closest
       dist = np.linalg.norm(drift_vec)
       if dist < min_dist:
           min_dist = dist
           best_drift = tuple(drift_vec)


   return best_drift


def drone_logging_position_state_estimate(scf, log_state_estimate, log_dict_state, waypoints):
   takeoff_started.wait()


   first_time = True
   roll0 = pitch0 = yaw0 = None


   with SyncLogger(scf, log_state_estimate) as logger:


       time.sleep(2)


       for log_entry in logger:
           if takeoff_ended.is_set():
               break


           timestamp = log_entry[0]
           data = log_entry[1]


           x = data['stateEstimate.x']
           y = data['stateEstimate.y']
           z = data['stateEstimate.z']


           roll = data['stateEstimate.roll']
           pitch = data['stateEstimate.pitch']
           yaw = data['stateEstimate.yaw']


           if first_time:
               roll0, pitch0, yaw0 = roll, pitch, yaw
               first_time = False


           # Compute drift
           dx, dy, dz = compute_drift_from_path((x, y, z), waypoints)
           droll = roll - roll0
           dpitch = pitch - pitch0
           dyaw = yaw - yaw0


           event = create_event(
               x_drift=dx, y_drift=dy, z_drift=dz,
               roll=droll, pitch=dpitch, yaw=dyaw,
               time_val=timestamp / 1000.0
           )


           verdict = drift_lib.cycle(ctypes.byref(memory_instance), event)


           def get_val(name):
               return getattr(verdict, name)


           def get_trigger(name):
               present = getattr(verdict, f"{name}_is_present")
               val = getattr(verdict, name)
               return ctypes.string_at(val).decode() if present and val else ""


           log_dict_state[timestamp] = {
               "timestamp": timestamp,
               "x": x, "y": y, "z": z,
               "x_drift": dx, "y_drift": dy, "z_drift": dz,
               "roll": roll, "pitch": pitch, "yaw": yaw,
               "roll_drift": droll, "pitch_drift": dpitch, "yaw_drift": dyaw,


               "x_drift_pos_exceeded": int(get_val("x_drift_pos_exceeded")),
               "x_drift_pos_trigger":  get_trigger("trigger_0"),
               "x_drift_neg_exceeded": int(get_val("x_drift_neg_exceeded")),
               "x_drift_neg_trigger":  get_trigger("trigger_1"),


               "y_drift_pos_exceeded": int(get_val("y_drift_pos_exceeded")),
               "y_drift_pos_trigger":  get_trigger("trigger_2"),
               "y_drift_neg_exceeded": int(get_val("y_drift_neg_exceeded")),
               "y_drift_neg_trigger":  get_trigger("trigger_3"),


               "z_drift_pos_exceeded": int(get_val("z_drift_pos_exceeded")),
               "z_drift_pos_trigger":  get_trigger("trigger_4"),
               "z_drift_neg_exceeded": int(get_val("z_drift_neg_exceeded")),
               "z_drift_neg_trigger":  get_trigger("trigger_5"),


               "pitch_exceeded": int(get_val("pitch_exceeded")),
               "pitch_trigger":  get_trigger("trigger_12"),
               "roll_exceeded":  int(get_val("roll_exceeded")),
               "roll_trigger":   get_trigger("trigger_13"),
               "yaw_exceeded":   int(get_val("yaw_exceeded")),
               "yaw_trigger":    get_trigger("trigger_14"),
           }


           '''
           print(f"[{timestamp}] Pos: ({x:.2f}, {y:.2f}, {z:.2f}) | Drift: (dx = {dx:.2f}, dy = {dy:.2f}, dz = {dz:.2f}) | "
                 f"Orientation: (roll = {roll:.2f}, pitch = {pitch:.2f}, yaw = {yaw:.2f}) | "
                 f"Orient Drift: (droll = {droll:.2f}, dpitch = {dpitch:.2f}, dyaw = {dyaw:.2f})")
          
           display_verdict_triggers(verdict)
           '''


def drone_logging_position_multi_ranger(scf, log_multi_ranger, log_dict_ranger, waypoints, position_lock, shared_position):
   takeoff_started.wait()


   # Offset if using square-from-corner flight
   offset = 0.64
   RIGHT_BOUND = 2.19 + offset
   FRONT_BOUND = 2.57 + offset
   LEFT_BOUND = 2.19 - offset
   BACK_BOUND = 2.57 - offset 
   TOP_BOUND = 3.24


   x_tolerance = 2.25
   y_tolerance = 2.25
   z_tolerance = 2.25


   with SyncLogger(scf, log_multi_ranger) as logger:
       time.sleep(2)


       for log_entry in logger:
           if takeoff_ended.is_set():
               break


           timestamp = log_entry[0]
           data = log_entry[1]


           try:
               front = data.get('range.front') / 1000.0
               back = data.get('range.back') / 1000.0
               left = data.get('range.left') / 1000.0
               right = data.get('range.right') / 1000.0
               up = data.get('range.up') / 1000.0
               '''
               front = float(f"{data.get('range.front') / 1000.0:.2f}")
               back = float(f"{data.get('range.back') / 1000.0:.2f}")
               left = float(f"{data.get('range.left') / 1000.0:.2f}")
               right = float(f"{data.get('range.right') / 1000.0:.2f}")
               up = float(f"{data.get('range.up') / 1000.0:.2f}")
               '''
           except (TypeError, ValueError):
               print(f"[{timestamp}] Type error in range data, skipping.")
               continue


           # Infer position within known cube bounds
           # mx = float(f"{(RIGHT_BOUND - right):.2f}")
           # my = float(f"{(FRONT_BOUND - front):.2f}")
           # mz = float(f"{(TOP_BOUND - up):.2f}")
           # print("Right: {f}, Left: {f}, Back: {f}.format(mx_right_ready=mx_right_ready, mx_left_ready=mx_left_ready, my_back_ready=my_back_ready)}")


           if not mx_right_ready.is_set():
               mx = float(f"{(left - LEFT_BOUND):.2f}")
           elif mx_right_ready.is_set() and not mx_left_ready.is_set():
               mx = float(f"{(RIGHT_BOUND - right):.2f}")
           elif mx_left_ready.is_set():
               mx = float(f"{(left - LEFT_BOUND):.2f}")


           if not my_back_ready.is_set():
               my = float(f"{(FRONT_BOUND - front):.2f}")
           elif my_back_ready.is_set():
               my = float(f"{(back - BACK_BOUND):.2f}")


           mz = float(f"{(TOP_BOUND - up):.2f}")


           #print(f"(Multiranger Raw Position: {mx:.2f}, {my:.2f}, {mz:.2f})")


          
           if abs(mx) > x_tolerance or abs(my) > y_tolerance or abs(mz) > z_tolerance:
               print(f"[{timestamp}] Invalid position ({mx:.2f}, {my:.2f}, {mz:.2f}) detected, skipping logging.")
               with position_lock:
                   if abs(mx) <= x_tolerance:
                       shared_position["mx"] = mx
                   if abs(my) <= y_tolerance:
                       shared_position["my"] = my
                   if abs(mz) <= z_tolerance:
                       shared_position["mz"] = mz
                   if not position_ready.is_set():
                       position_ready.set()
          
               continue
          
           dmx, dmy, dmz = compute_drift_from_path((mx, my, mz), waypoints)


           with position_lock:
               shared_position["mx"] = mx
               shared_position["my"] = my
               shared_position["mz"] = mz
               shared_position["dmx"] = dmx
               shared_position["dmy"] = dmy
               shared_position["dmz"] = dmz
               if not position_ready.is_set():
                   position_ready.set()


           event = create_event(
               multi_ranger_x_drift=dmx,
               multi_ranger_y_drift=dmy,
               multi_ranger_z_drift=dmz,
               time_val=timestamp / 1000.0
           )


           verdict = drift_lib.cycle(ctypes.byref(memory_instance), event)


           def get_val(name):
               return getattr(verdict, name)


           def get_trigger(name):
               present = getattr(verdict, f"{name}_is_present")
               val = getattr(verdict, name)
               return ctypes.string_at(val).decode() if present and val else ""


           log_dict_ranger[timestamp] = {
               "timestamp": timestamp,
               "mx": mx, "my": my, "mz": mz,
               "multi_ranger_x_drift": dmx,
               "multi_ranger_y_drift": dmy,
               "multi_ranger_z_drift": dmz,
               "multi_ranger_x_drift_pos_exceeded": int(get_val("multi_ranger_x_drift_pos_exceeded")),
               "multi_ranger_x_drift_pos_trigger": get_trigger("trigger_6"),
               "multi_ranger_x_drift_neg_exceeded": int(get_val("multi_ranger_x_drift_neg_exceeded")),
               "multi_ranger_x_drift_neg_trigger": get_trigger("trigger_7"),
               "multi_ranger_y_drift_pos_exceeded": int(get_val("multi_ranger_y_drift_pos_exceeded")),
               "multi_ranger_y_drift_pos_trigger": get_trigger("trigger_8"),
               "multi_ranger_y_drift_neg_exceeded": int(get_val("multi_ranger_y_drift_neg_exceeded")),
               "multi_ranger_y_drift_neg_trigger": get_trigger("trigger_9"),
               "multi_ranger_z_drift_pos_exceeded": int(get_val("multi_ranger_z_drift_pos_exceeded")),
               "multi_ranger_z_drift_pos_trigger": get_trigger("trigger_10"),
               "multi_ranger_z_drift_neg_exceeded": int(get_val("multi_ranger_z_drift_neg_exceeded")),
               "multi_ranger_z_drift_neg_trigger": get_trigger("trigger_11"),
               "right": right, "front": front, "up": up,
           }


           print(f"[{timestamp}] MultiRanger Pos: ({mx:.2f}, {my:.2f}, {mz:.2f}) | "
                 f"Drift: (dmx = {dmx:.2f}, dmy = {dmy:.2f}, dmz = {dmz:.2f})")


           display_verdict_triggers(verdict)


           if verdict.multi_ranger_x_drift_neg_exceeded and not autocorrect_position_x_neg.is_set():
               autocorrect_position_x_neg.set()
               with position_lock:
                   shared_position["mx"] = mx


           if verdict.multi_ranger_x_drift_pos_exceeded and not autocorrect_position_x_pos.is_set():
               autocorrect_position_x_pos.set()
               with position_lock:
                   shared_position["mx"] = mx


           if verdict.multi_ranger_y_drift_neg_exceeded and not autocorrect_position_y_neg.is_set():
               autocorrect_position_y_neg.set()
               with position_lock:
                   shared_position["my"] = my


           if verdict.multi_ranger_y_drift_pos_exceeded and not autocorrect_position_y_pos.is_set():
               autocorrect_position_y_pos.set()
               with position_lock:
                   shared_position["my"] = my


           if verdict.multi_ranger_z_drift_neg_exceeded and not autocorrect_position_z_neg.is_set():
               autocorrect_position_z_neg.set()
               with position_lock:
                   shared_position["mz"] = mz


           if verdict.multi_ranger_z_drift_pos_exceeded and not autocorrect_position_z_pos.is_set():
               autocorrect_position_z_pos.set()
               with position_lock:
                   shared_position["mz"] = mz




if __name__ == '__main__':
   cflib.crtp.init_drivers()


   run_id = "run15"


   log_dict_state = {}
   log_dict_ranger = {}
   ideal_coords_holder = {}


   start_time = time.time()


   with SyncCrazyflie(URI, cf=Crazyflie(rw_cache='./cache')) as scf:


       # Flow deck check
       scf.cf.param.add_update_callback(group='deck', name='bcFlow2', cb=param_deck_flow)
       scf.cf.param.add_update_callback(group='deck', name='bcMultiranger', cb=param_deck_multi_ranger)


       time.sleep(1)
       if not flow_deck_event.wait(timeout=1):
           sys.exit(1)


       if not multiranger_deck_event.wait(timeout=2):
           sys.exit(1)


       # Logging state estimate parameters
       log_state_estimate = LogConfig(name='Position', period_in_ms=100)


       log_state_estimate.add_variable('stateEstimate.x', 'float')
       log_state_estimate.add_variable('stateEstimate.y', 'float')
       log_state_estimate.add_variable('stateEstimate.z', 'float')


       log_state_estimate.add_variable('stateEstimate.roll', 'float')
       log_state_estimate.add_variable('stateEstimate.pitch', 'float')
       log_state_estimate.add_variable('stateEstimate.yaw', 'float')


       # Logging multi-ranger parameters
       log_multi_ranger = LogConfig(name='Multi-Ranger', period_in_ms=100)


       log_multi_ranger.add_variable('range.front', 'float')
       log_multi_ranger.add_variable('range.back', 'float')
       log_multi_ranger.add_variable('range.up', 'float')
       log_multi_ranger.add_variable('range.left', 'float')
       log_multi_ranger.add_variable('range.right', 'float')


       shared_position = {"mx": 0.0, "my": 0.0, "mz": 0.0}
       position_lock = Lock()


       def flight_wrapper():
           #ideal_coords_holder["coords"] = waypoint_flight(scf, False)
           #square_turns_starting_at_center(scf, 0.3)
           #square_turns_starting_at_corner(scf, 0.8)
           combined_flight(scf, position_lock, shared_position)


       flight_thread = threading.Thread(target=flight_wrapper)


       project_dir_log = f"/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/rtlola_second_spec_runs/v1_logs/{run_id}"
       os.makedirs(project_dir_log, exist_ok=True)


       path_state_csv = os.path.join(project_dir_log, f"{run_id}_state.csv")
       path_ranger_csv = os.path.join(project_dir_log, f"{run_id}_ranger.csv")


       project_dir_plot = f"/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/rtlola_second_spec_runs/v1_plots/{run_id}"
       os.makedirs(project_dir_plot, exist_ok=True)


       print(f"Logging at: {project_dir_log}")
       print(f"Plotting at: {project_dir_plot}")


       # For waypoint flight
       '''
       ideal_coords = [
           (0, 0, 0),
           (0, 0, 1.5),
           (-0.6, 0.6, 1.2),
           (0.8, -0.4, 0.6),
           (0.3, 0.7, 1.0)
       ]
       '''


       # For square turns flight from center
       '''
       ideal_coords = [
           (0, 0, 0),
           (0, 0, 1.5),
           (0, 0.6, 1.5),
           (-0.6, 0.6, 1.5),
           (-0.6, -0.6, 1.5),
           (0.6, -0.6, 1.5),
           (0.6, 0.6, 1.5),
           (0, 0.6, 1.5),
       ]
       '''


       # For square turns flight from corners
       # '''
       ideal_coords_state = [
           (0, 0, 0),
           (0, 0, 1.5),
           (0, 1.2, 1.5),
           (1.2, 1.2, 1.5),
           (1.2, 0, 1.5)
       ]


       ideal_coords_ranger = [
           (0, 0, 0),
           (0, 0, 1.5),
           (0, 1.2, 1.5),
           (1.2, 1.2, 1.5),
           (1.2, 0, 1.5)
       ]
       # '''


       state_estimate_thread = threading.Thread(
           target=drone_logging_position_state_estimate,
           args=(scf, log_state_estimate, log_dict_state, ideal_coords_state)
       )


       multi_ranger_thread = threading.Thread(
           target=drone_logging_position_multi_ranger,
           args=(scf, log_multi_ranger, log_dict_ranger, ideal_coords_ranger, position_lock, shared_position)
       )


       multi_ranger_thread.start()
       flight_thread.start()
       state_estimate_thread.start()


       multi_ranger_thread.join()
       flight_thread.join()
       state_estimate_thread.join()


       state_rows = list(log_dict_state.values())
       ranger_rows = list(log_dict_ranger.values())


       write_state_csv_log(path_state_csv, log_dict_state)
       write_ranger_csv_log(path_ranger_csv, log_dict_ranger)


       graph_state_estimate_drift(project_dir_plot, state_rows, run_id)
       graph_multiranger_drift(project_dir_plot, ranger_rows, run_id)


       graph_3d_state_estimate_vs_ideal(project_dir_plot, state_rows, ideal_coords_state, run_id)
       graph_3d_multiranger_vs_ideal(project_dir_plot, ranger_rows, ideal_coords_ranger, run_id)


       plot_state_estimate_xyz_vs_time(state_rows, run_id, os.path.join(project_dir_plot, f"{run_id}_state_xyz_vs_time.png"))
       plot_multiranger_xyz_vs_time(ranger_rows, run_id, os.path.join(project_dir_plot, f"{run_id}_multi_ranger_xyz_vs_time.png"))


       plot_muliranger_raw_xyz_vs_time(ranger_rows, run_id, os.path.join(project_dir_plot, f"{run_id}_multi_ranger_raw_xyz_vs_time.png"))


       print("Logging & flight completed.")