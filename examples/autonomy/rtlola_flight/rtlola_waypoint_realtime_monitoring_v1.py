import ctypes
from ctypes import CDLL, POINTER, Structure, c_double, c_int64, c_bool, c_char_p

import time

#Edit to access your personally built RTLola Monitors
drift_lib = ctypes.CDLL("/home/bitcraze/projects/crazyflie-lib-python-code/rtlola/waypoint_monitor/libmonitor.so")

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

#Full Memory struct
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

#Helper to build events
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

#Trigger display helper
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
import os
import csv
import matplotlib.pyplot as plt

URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')

DEFAULT_HEIGHT = 1.5
ERROR_PARAMETER = 0.2

deck_attached_event = Event()

logging.basicConfig(level=logging.ERROR)

memory_instance = Memory()

def send_state_to_monitor(x_val, x0, y_val, y0, timestamp):
    x_drift_val = x_val - x0
    y_drift_val = y_val - y0

    event = create_event(
        x_drift=x_drift_val,
        y_drift=y_drift_val,
        time_val=timestamp / 1000.0  #Convert timestamp to seconds
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
def waypoint_flight(scf, log_drone, rnd):
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

def write_csv_log(full_csv_path_log, logging_rows):
    headers = [
        "Timestamp",
        "x", "y", "z",
        "x_drift", "y_drift", "z_drift",
        "roll", "pitch", "yaw",
        "roll_drift", "pitch_drift", "yaw_drift",
        "vbat",
        "motor_pass", "battery_pass",
        "motor_failed", "trigger_0",
        "battery_failed", "trigger_1",
        "x_drift_exceeded", "trigger_2",
        "y_drift_exceeded", "trigger_3",
        "z_drift_exceeded", "trigger_4",
        "pitch_exceeded", "trigger_5",
        "roll_exceeded", "trigger_6",
        "yaw_exceeded", "trigger_7",
        "battery_low", "trigger_8"
    ]

    with open(full_csv_path_log, 'w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow(headers)
        writer.writerows(logging_rows)

    print(f"CSV saved to: {full_csv_path_log}")

def graph_data(project_directory_plot, logging_rows):
    #Extract from logging_rows
    timestamps = [row[0] for row in logging_rows]
    time_seconds = [t / 1000.0 for t in timestamps]

    x_drift = [row[4] for row in logging_rows]
    y_drift = [row[5] for row in logging_rows]
    z_drift = [row[6] for row in logging_rows]

    roll_drift = [row[10] for row in logging_rows]
    pitch_drift = [row[11] for row in logging_rows]
    yaw_drift = [row[12] for row in logging_rows]

    #Plot position drift
    plt.figure(figsize=(12, 6))
    plt.plot(time_seconds, x_drift, label='X Drift', linewidth=2, color='blue')
    plt.plot(time_seconds, y_drift, label='Y Drift', linewidth=2, color='red')
    plt.plot(time_seconds, z_drift, label='Z Drift', linewidth=2, color='green')

    plt.title('Drone Positional Drift vs. Time')
    plt.xlabel('Time (seconds)')
    plt.ylabel('Drift (meters)')

    plt.legend()
    plt.grid(True)
    plt.tight_layout()

    position_plot_path = os.path.join(project_directory_plot, "run1_position_drift.pdf")
    plt.savefig(position_plot_path, dpi=300)
    plt.close()

    #Plot orientation drift
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

    orientation_plot_path = os.path.join(project_directory_plot, "run1_orientation_drift.pdf")
    plt.savefig(orientation_plot_path, dpi=300)
    plt.close()

    print(f"Saved plots to:\n{position_plot_path}\n{orientation_plot_path}")


def drone_logging(scf, log_drone):
    project_directory_log = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/rtlola/v1_logs"
    full_csv_path_log = os.path.join(project_directory_log, "run1.csv")

    project_directory_plot = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/rtlola/v1_plots/run1"

    #Create directories
    os.makedirs(project_directory_log, exist_ok=True)
    os.makedirs(project_directory_plot, exist_ok=True)

    print(f"Logging to: {full_csv_path_log}")
    print(f"Plotting at: {project_directory_plot}")

    first_time = True
    logging_rows = []

    with SyncLogger(scf, log_drone) as logger:
        end_time = time.time() + 35
        time.sleep(2)

        for log_entry in logger:
            if time.time() >= end_time:
                print("End time reached, stopping logging.")
                break

            timestamp = log_entry[0]
            data = log_entry[1]

            #Raw sensor values
            x = data['stateEstimate.x']
            y = data['stateEstimate.y']
            z = data['stateEstimate.z']

            roll = data['stateEstimate.roll']
            pitch = data['stateEstimate.pitch']
            yaw = data['stateEstimate.yaw']

            vbat = data['pm.vbat']

            motor_pass = int(data['health.motorPass'])
            battery_pass = int(data['health.batteryPass'])

            if first_time:
                x0, y0, z0 = x, y, z
                roll0, pitch0, yaw0 = roll, pitch, yaw

                first_time = False

            #Compute drift
            dx = x - x0
            dy = y - y0
            dz = z - z0

            droll = roll - roll0
            dpitch = pitch - pitch0
            dyaw = yaw - yaw0

            #Send to RTLola monitor
            event = create_event(
                motor_pass=motor_pass,
                battery_pass=battery_pass,
                x_drift=dx,
                y_drift=dy,
                z_drift=dz,
                pitch=dpitch,
                roll=droll,
                yaw=dyaw,
                vbat=vbat,
                time_val=timestamp / 1000.0
            )
            
            verdict = drift_lib.cycle(ctypes.byref(memory_instance), event)

            #Extract triggers and verdict flags
            def get_val(name):
                return getattr(verdict, name)

            def get_trigger(name):
                present = getattr(verdict, f"{name}_is_present")
                val = getattr(verdict, name)
                return ctypes.string_at(val).decode() if present and val else ""

            row = [
                timestamp,
                x, y, z,
                dx, dy, dz,
                roll, pitch, yaw,
                droll, dpitch, dyaw,
                vbat,
                motor_pass, battery_pass,
                int(get_val("motor_failed")),
                get_trigger("trigger_0"),
                int(get_val("battery_failed")),
                get_trigger("trigger_1"),
                int(get_val("x_drift_exceeded")),
                get_trigger("trigger_2"),
                int(get_val("y_drift_exceeded")),
                get_trigger("trigger_3"),
                int(get_val("z_drift_exceeded")),
                get_trigger("trigger_4"),
                int(get_val("pitch_exceeded")),
                get_trigger("trigger_5"),
                int(get_val("roll_exceeded")),
                get_trigger("trigger_6"),
                int(get_val("yaw_exceeded")),
                get_trigger("trigger_7"),
                int(get_val("battery_low")),
                get_trigger("trigger_8")
            ]

            logging_rows.append(row)

            print("\n" + "=" * 80)

            print(f"Timestamp: {timestamp}")

            print("-" * 80)

            print("Telemetry")
            print(f"  Position       → x: {x:.2f}, y: {y:.2f}, z: {z:.2f}")
            print(f"  Drift          → dx: {dx:.2f}, dy: {dy:.2f}, dz: {dz:.2f}")
            print(f"  Orientation    → roll: {roll:.2f}, pitch: {pitch:.2f}, yaw: {yaw:.2f}")
            print(f"  Drift (orient) → d_roll: {droll:.2f}, d_pitch: {dpitch:.2f}, d_yaw: {dyaw:.2f}")
            print(f"  Battery        → vbat: {vbat:.2f} V")
            print(f"  Health Flags   → motorPass: {motor_pass}, batteryPass: {battery_pass}")

            print("\n RTLola Verdict Flags")
            print(f"  motor_failed     : {int(get_val('motor_failed'))}")
            print(f"  battery_failed   : {int(get_val('battery_failed'))}")
            print(f"  x_drift_exceeded : {int(get_val('x_drift_exceeded'))}")
            print(f"  y_drift_exceeded : {int(get_val('y_drift_exceeded'))}")
            print(f"  z_drift_exceeded : {int(get_val('z_drift_exceeded'))}")
            print(f"  pitch_exceeded   : {int(get_val('pitch_exceeded'))}")
            print(f"  roll_exceeded    : {int(get_val('roll_exceeded'))}")
            print(f"  yaw_exceeded     : {int(get_val('yaw_exceeded'))}")
            print(f"  battery_low      : {int(get_val('battery_low'))}")

            print("\n Triggers Fired")
            display_verdict_triggers(verdict)

            print("=" * 80 + "\n")

    #Write to CSV
    write_csv_log(full_csv_path_log, logging_rows)

    #Plot data
    graph_data(project_directory_plot, logging_rows)

if __name__ == '__main__':
    cflib.crtp.init_drivers()

    start_time = time.time()
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
        log_drone = LogConfig(name='Crazyflie_Variables', period_in_ms=100)
        log_drone.add_variable('health.motorPass', 'float')
        log_drone.add_variable('health.batteryPass', 'float')

        #Battery Life
        log_drone.add_variable('pm.vbat', 'float')

        #Position
        log_drone.add_variable('stateEstimate.x', 'float')
        log_drone.add_variable('stateEstimate.y', 'float')
        log_drone.add_variable('stateEstimate.z', 'float')

        #Rotation
        log_drone.add_variable('stateEstimate.roll', 'float')
        log_drone.add_variable('stateEstimate.pitch', 'float')
        log_drone.add_variable('stateEstimate.yaw', 'float')

        t1 = threading.Thread(target=waypoint_flight, args=(scf, log_drone, True))
        t2 = threading.Thread(target=drone_logging, args=(scf, log_drone))

        t1.start()
        t2.start()

        t1.join()
        t2.join()

        print("Logging & flight completed.")