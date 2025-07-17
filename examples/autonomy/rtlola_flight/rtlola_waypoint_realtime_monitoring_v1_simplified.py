import ctypes
from ctypes import CDLL, POINTER, Structure, c_double, c_int64, c_bool, c_char_p

import time

drift_lib = ctypes.CDLL("/home/bitcraze/projects/crazyflie-lib-python-code/rtlola/waypoint_monitor_simplified/libmonitor.so")

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
    
class Memory(ctypes.Structure):
    _fields_ = [
        ("boundedbuffer_x_drift", BoundedBuffer_double),
        ("boundedbuffer_y_drift", BoundedBuffer_double),
        ("boundedbuffer_z_drift", BoundedBuffer_double),
        ("boundedbuffer_pitch", BoundedBuffer_double),
        ("boundedbuffer_roll", BoundedBuffer_double),
        ("boundedbuffer_yaw", BoundedBuffer_double),
        ("boundedbuffer_abs_x_drift", BoundedBuffer_double),
        ("boundedbuffer_abs_y_drift", BoundedBuffer_double),
        ("boundedbuffer_abs_z_drift", BoundedBuffer_double),
        ("boundedbuffer_abs_pitch", BoundedBuffer_double),
        ("boundedbuffer_abs_roll", BoundedBuffer_double),
        ("boundedbuffer_abs_yaw", BoundedBuffer_double),
        ("boundedbuffer_x_drift_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_0", BoundedBuffer_str),
        ("boundedbuffer_y_drift_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_1", BoundedBuffer_str),
        ("boundedbuffer_z_drift_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_2", BoundedBuffer_str),
        ("boundedbuffer_pitch_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_3", BoundedBuffer_str),
        ("boundedbuffer_roll_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_4", BoundedBuffer_str),
        ("boundedbuffer_yaw_exceeded", BoundedBuffer_bool),
        ("boundedbuffer_trigger_5", BoundedBuffer_str),
        ("time", ctypes.c_double)
    ]

class InternalEvent(Structure):
    _fields_ = [
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
        ("x_drift_exceeded", c_bool),
        ("x_drift_exceeded_is_present", c_bool),
        ("trigger_0", c_char_p),
        ("trigger_0_is_present", c_bool),
        ("y_drift_exceeded", c_bool),
        ("y_drift_exceeded_is_present", c_bool),
        ("trigger_1", c_char_p),
        ("trigger_1_is_present", c_bool),
        ("z_drift_exceeded", c_bool),
        ("z_drift_exceeded_is_present", c_bool),
        ("trigger_2", c_char_p),
        ("trigger_2_is_present", c_bool),
        ("pitch_exceeded", c_bool),
        ("pitch_exceeded_is_present", c_bool),
        ("trigger_3", c_char_p),
        ("trigger_3_is_present", c_bool),
        ("roll_exceeded", c_bool),
        ("roll_exceeded_is_present", c_bool),
        ("trigger_4", c_char_p),
        ("trigger_4_is_present", c_bool),
        ("yaw_exceeded", c_bool),
        ("yaw_exceeded_is_present", c_bool),
        ("trigger_5", c_char_p),
        ("trigger_5_is_present", c_bool),
        ("time", c_double),
    ]

drift_lib.cycle.restype = Verdict
drift_lib.cycle.argtypes = [POINTER(Memory), InternalEvent]

def create_event(x_drift=None, y_drift=None, z_drift=None,
                 pitch=None, roll=None, yaw=None, time_val=0.0):
    e = InternalEvent()

    def set_field(value, attr, present_attr):
        if value is not None:
            setattr(e, attr, value)
            setattr(e, present_attr, True)
        else:
            setattr(e, present_attr, False)

    set_field(x_drift, 'x_drift', 'x_drift_is_present')
    set_field(y_drift, 'y_drift', 'y_drift_is_present')
    set_field(z_drift, 'z_drift', 'z_drift_is_present')
    set_field(pitch, 'pitch', 'pitch_is_present')
    set_field(roll, 'roll', 'roll_is_present')
    set_field(yaw, 'yaw', 'yaw_is_present')
    e.time = time_val
    return e

def display_verdict_triggers(verdict):
    for i in range(6):
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
import matplotlib.pyplot as plt

URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')

DEFAULT_HEIGHT = 1
ERROR_PARAMETER = 0.2

deck_attached_event = Event()

logging.basicConfig(level=logging.ERROR)

memory_instance = Memory()

takeoff_started = threading.Event()

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
        print("Coordinate 2 (Hover): (0, 0, 1)")
        print("Coordinate 3: (-0.6, 1, 1.2)")
        print("Coordinate 4: (0.8, 1.6, 0.4)")
        print("Coordinate 5: (2, 0.4, 0.8)")
        print("Coordinate 6 (RTH): (0, 0, 1)")
        print("Coordinate 7 (Landing): (0, 0, 0)")

        print("Takeoff.")
        takeoff_started.set()
        with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
            time.sleep(5)
            mc.move_distance(-0.6, 1, 0.2, velocity=1.5)
            time.sleep(5)
            mc.move_distance(1.4, 0.6, -0.8, velocity=1.5)
            time.sleep(5)
            mc.move_distance(1.2, -1.2, 0.4, velocity=1.5)
            time.sleep(5)
            mc.move_distance(-2, -0.4, 0.2, velocity=1.5)
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

def write_csv_log(full_csv_path_log, log_dict):
    headers = [
        "timestamp", "x", "y", "z",
        "x_drift", "y_drift", "z_drift",
        "roll", "pitch", "yaw",
        "roll_drift", "pitch_drift", "yaw_drift",
        "x_drift_exceeded", "x_drift_trigger",
        "y_drift_exceeded", "y_drift_trigger",
        "z_drift_exceeded", "z_drift_trigger",
        "pitch_exceeded", "pitch_trigger",
        "roll_exceeded", "roll_trigger",
        "yaw_exceeded", "yaw_trigger"
    ]

    with open(full_csv_path_log, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(headers)
        for ts in sorted(log_dict.keys()):
            row = [log_dict[ts].get(col, "") for col in headers]
            writer.writerow(row)

    print(f"CSV saved to: {full_csv_path_log}")

def graph_data(project_directory_plot, logging_rows):
    # Extract from logging_rows
    timestamps = [row["timestamp"] for row in logging_rows]
    time_seconds = [t / 1000.0 for t in timestamps]

    x_drift = [row['x_drift'] for row in logging_rows]
    y_drift = [row['y_drift'] for row in logging_rows]
    z_drift = [row['z_drift'] for row in logging_rows]

    roll_drift = [row['roll_drift'] for row in logging_rows]
    pitch_drift = [row['pitch_drift'] for row in logging_rows]
    yaw_drift = [row['yaw_drift'] for row in logging_rows]

    # Plot position drift
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

    position_plot_path = os.path.join(project_directory_plot, "run11_position_drift.png")
    plt.savefig(position_plot_path, dpi=300)
    plt.close()

    # Plot orientation drift
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

    orientation_plot_path = os.path.join(project_directory_plot, "run11_orientation_drift.png")
    plt.savefig(orientation_plot_path, dpi=300)
    plt.close()

    print(f"Saved plots to:\n{position_plot_path}\n{orientation_plot_path}")

def graph_3d_trajectory(project_directory_plot, logging_rows, ideal_coords):
    import matplotlib.pyplot as plt
    from mpl_toolkits.mplot3d import Axes3D
    import os

    # Actual flight positions
    x_actual = [row["x"] for row in logging_rows]
    y_actual = [row["y"] for row in logging_rows]
    z_actual = [row["z"] for row in logging_rows]

    x_ideal, y_ideal, z_ideal = zip(*ideal_coords[:5])

    x_ideal_loop = list(x_ideal) + [x_ideal[1]]
    y_ideal_loop = list(y_ideal) + [y_ideal[1]]
    z_ideal_loop = list(z_ideal) + [z_ideal[1]]

    fig = plt.figure(figsize=(18, 14))
    ax = fig.add_subplot(111, projection='3d')

    # Plot actual flight path
    ax.plot(x_actual, y_actual, z_actual, label='Actual Flight Path', color='blue', linewidth=2)

    # Plot ideal path
    ax.plot(x_ideal_loop, y_ideal_loop, z_ideal_loop, label='Ideal Waypoints Path', color='green', linestyle='--', linewidth=2)
    ax.scatter(x_ideal, y_ideal, z_ideal, color='black', s=80, marker='x')

    waypoint_labels = [
        "Takeoff / Landing",
        "Hover / Return-to-Home",
        "Coordinate 1",
        "Coordinate 2",
        "Coordinate 3"
    ]
    for i, label in enumerate(waypoint_labels):
        ax.text(x_ideal[i], y_ideal[i], z_ideal[i] + 0.08, label, fontsize=14, color='black', weight='bold')

    ax.set_title("3D Flight Visualization: Actual vs. Ideal Waypoints", fontsize=18)
    ax.set_xlabel("X (m)", fontsize=14)
    ax.set_ylabel("Y (m)", fontsize=14)
    ax.set_zlabel("Z (m)", fontsize=14)

    ax.legend(fontsize=12)
    ax.grid(True)

    ax.view_init(elev=25, azim=135)

    plt.tight_layout()
    plot_path_base = os.path.join(project_directory_plot, "run11_3d_flight_path.svg")
    plt.savefig(plot_path_base, format='svg')
    plt.close()
    print(f"Saved 3D trajectory plot to: {plot_path_base}")

def closest_point_on_segment(p, a, b):
    """Find the closest point on segment ab to point p."""
    p = np.array(p)
    a = np.array(a)
    b = np.array(b)
    ab = b - a
    ap = p - a
    ab_len_sq = np.dot(ab, ab)

    if ab_len_sq == 0:
        return a  # segment is a point
    t = np.dot(ap, ab) / ab_len_sq
    t = max(0, min(1, t))  # Clamp to segment
    return a + t * ab

def compute_drift_from_path(pos, waypoints):
    """Return drift vector (dx, dy, dz) from pos to nearest path segment."""
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

def drone_logging_position(scf, log_position, log_dict, waypoints):
    takeoff_started.wait()

    first_time = True
    roll0 = pitch0 = yaw0 = None

    with SyncLogger(scf, log_position) as logger:
        end_time = time.time() + 60
        time.sleep(2)

        for log_entry in logger:
            if time.time() >= end_time:
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

            # === Use geometric drift ===
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

            row_data = {
                "timestamp": timestamp,
                "x": x, "y": y, "z": z,
                "x_drift": dx, "y_drift": dy, "z_drift": dz,
                "roll": roll, "pitch": pitch, "yaw": yaw,
                "roll_drift": droll, "pitch_drift": dpitch, "yaw_drift": dyaw,

                "x_drift_exceeded": int(get_val("x_drift_exceeded")),
                "x_drift_trigger": get_trigger("trigger_0"),

                "y_drift_exceeded": int(get_val("y_drift_exceeded")),
                "y_drift_trigger": get_trigger("trigger_1"),

                "z_drift_exceeded": int(get_val("z_drift_exceeded")),
                "z_drift_trigger": get_trigger("trigger_2"),

                "pitch_exceeded": int(get_val("pitch_exceeded")),
                "pitch_trigger": get_trigger("trigger_3"),

                "roll_exceeded": int(get_val("roll_exceeded")),
                "roll_trigger": get_trigger("trigger_4"),

                "yaw_exceeded": int(get_val("yaw_exceeded")),
                "yaw_trigger": get_trigger("trigger_5"),
            }

            log_dict[timestamp] = row_data

            print(f"[{timestamp}] Pos: ({x:.2f}, {y:.2f}, {z:.2f}) | Drift: (x = {dx:.2f}, y = {dy:.2f}, z ={ dz:.2f}) | "
                  f"Orientation: (roll = {roll:.2f}, pitch = {pitch:.2f}, yaw = {yaw:.2f}) | "
                  f"Orient Drift: (droll = {droll:.2f}, dpitch = {dpitch:.2f}, dyaw = {dyaw:.2f})")
            display_verdict_triggers(verdict)

if __name__ == '__main__':
    cflib.crtp.init_drivers()

    log_dict = {}
    ideal_coords_holder = {}

    start_time = time.time()    

    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache= './cache')) as scf:

        #Check if flow deck is attached
        scf.cf.param.add_update_callback(group='deck', name='bcFlow2', cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=1):
            print('No flow deck detected!')
            sys.exit(1)

        #Defining log variables
        #Position
        log_position = LogConfig(name='Position', period_in_ms=100)
        log_position.add_variable('stateEstimate.x', 'float')
        log_position.add_variable('stateEstimate.y', 'float')
        log_position.add_variable('stateEstimate.z', 'float')

        #Rotation
        log_position.add_variable('stateEstimate.roll', 'float')
        log_position.add_variable('stateEstimate.pitch', 'float')
        log_position.add_variable('stateEstimate.yaw', 'float')

        def flight_wrapper():
            ideal_coords_holder["coords"] = waypoint_flight(scf, False)

        flight_thread = threading.Thread(target=flight_wrapper)

        project_directory_data_log = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/rtlola/v1_logs"
        full_csv_path_data_log = os.path.join(project_directory_data_log, "run11.csv")

        project_directory_plot = "/home/bitcraze/projects/crazyflie-lib-python-code/examples/log_data/rtlola/v1_plots/run11"

        #Create directories
        os.makedirs(project_directory_data_log, exist_ok=True)
        os.makedirs(project_directory_plot, exist_ok=True)

        print(f"Logging at: {project_directory_data_log}")
        print(f"Plotting at: {project_directory_plot}")

        # Ideal coordinates for predefined flight
        ideal_coords = [
                (0, 0, 0),
                (0, 0, 1),
                (-0.6, 1, 1.2),
                (0.8, 1.6, 0.4),
                (2, 0.4, 0.8)
            ]

        position_thread = threading.Thread(target=drone_logging_position, args=(scf, log_position, log_dict, ideal_coords))

        flight_thread.start()
        position_thread.start()

        flight_thread.join()
        position_thread.join()
            
        write_csv_log(full_csv_path_data_log, log_dict)
        graph_data(project_directory_plot, list(log_dict.values()))
        graph_3d_trajectory(project_directory_plot, list(log_dict.values()), ideal_coords)

        print("Logging & flight completed.")