import time
import logging
from threading import Event

import cflib.crtp
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie
from cflib.crazyflie.syncLogger import SyncLogger
from cflib.positioning.motion_commander import MotionCommander
from cflib.crazyflie.log import LogConfig
from cflib.utils import uri_helper

# Set your URI
URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')

# Initialize drivers
cflib.crtp.init_drivers()
logging.basicConfig(level=logging.ERROR)

deck_attached_event = Event()

def param_deck(name, value_str):
    value = int(value_str)
    if value:
        deck_attached_event.set()
        print("[✔] MultiRanger deck attached.")
    else:
        print("[✘] MultiRanger deck NOT detected!")

def fmt_mm_to_m(val_mm):
    return f"{val_mm / 1000.0:.3f} m" if 10 < val_mm < 32000 else "No reading"

def multi_ranger_hover_log():
    log_config = LogConfig(name='Multiranger', period_in_ms=100)
    log_config.add_variable('range.front', 'float')
    log_config.add_variable('range.back', 'float')
    log_config.add_variable('range.left', 'float')
    log_config.add_variable('range.right', 'float')
    log_config.add_variable('range.up', 'float')

    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache='./cache')) as scf:
        # Attach deck check callback
        scf.cf.param.add_update_callback(group='deck', name='bcMultiranger', cb=param_deck)

        if not deck_attached_event.wait(timeout=2):
            print("[EXIT] MultiRanger deck not detected. Aborting.")
            return

        print("[✔] MultiRanger deck detected. Proceeding with hover and logging...")

        with MotionCommander(scf, default_height=1.25) as mc:
            print("[Hovering for 10 seconds and logging MultiRanger values...]")
            time.sleep(1.5)  # Allow time for sensors to stabilize

            with SyncLogger(scf, log_config) as logger:
                start_time = time.time()
                for log_entry in logger:
                    timestamp = log_entry[0]
                    data = log_entry[1]

                    # Exit after 10 seconds
                    if time.time() - start_time > 10:
                        break

                    print(" | ".join([
                        f"Front: {fmt_mm_to_m(data['range.front'])}",
                        f"Back: {fmt_mm_to_m(data['range.back'])}",
                        f"Left: {fmt_mm_to_m(data['range.left'])}",
                        f"Right: {fmt_mm_to_m(data['range.right'])}",
                        f"Up: {fmt_mm_to_m(data['range.up'])}",
                    ]))

            mc.stop()
            print("Hover complete.")

if __name__ == '__main__':
    multi_ranger_hover_log()
