import time
import threading
import logging
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.syncLogger import SyncLogger
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie
from cflib.crazyflie.log import LogConfig
from cflib.positioning.motion_commander import MotionCommander
from cflib.utils import uri_helper

URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')
logging.basicConfig(level=logging.ERROR)

log_config = LogConfig(name='Position', period_in_ms=100)
log_config.add_variable('stateEstimate.x', 'float')
log_config.add_variable('stateEstimate.y', 'float')
log_config.add_variable('stateEstimate.z', 'float')

def log_position(scf, duration=4.0):
    with SyncLogger(scf, log_config) as logger:
        start_time = time.time()
        for log_entry in logger:
            data = log_entry[1]
            x = data['stateEstimate.x']
            y = data['stateEstimate.y']
            z = data['stateEstimate.z']
            print(f"Time: {log_entry[0]} | Position (x={x:.2f}, y={y:.2f}, z={z:.2f})")
            if time.time() - start_time > duration:
                break

def fly_while_logging():
    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache='./cache')) as scf:
        # Reset Kalman filter
        scf.cf.param.set_value('kalman.resetEstimation', '1')
        time.sleep(0.1)
        scf.cf.param.set_value('kalman.resetEstimation', '0')
        time.sleep(1.0)

        with MotionCommander(scf, default_height=1.5) as mc:
            print("[TAKEOFF] Hovering at 1.5m...")
            time.sleep(1.5)

            # Start logging thread
            log_thread = threading.Thread(target=log_position, args=(scf, 4.0))
            log_thread.start()

            #print("[FLY] Moving forward 1 meter at 0.5 m/s")
            #mc.forward(1.0, velocity=0.5)

            #print("[FLY] Moving rightward 1 meter at 0.5 m/s")
            #mc.right(1.0, velocity=0.5)

            print("[FLY] Moving 'forward' 1 meter using mc.move_distance() at 0.5 m/s")
            mc.move_distance(0.0, 1.0, 0.0, velocity=0.5)

            #print("[FLY] Moving 'rightward' 1 meter using mc.move_distance() at 0.5 m/s")
            #mc.move_distance(1.0, 0.0, 0.0, velocity=0.5)

            log_thread.join()
            print("[LAND] Landing now...")
            mc.stop()

if __name__ == '__main__':
    import cflib.crtp
    cflib.crtp.init_drivers()
    fly_while_logging()
