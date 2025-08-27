import time
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie
from cflib.crazyflie.syncLogger import SyncLogger
from cflib.positioning.motion_commander import MotionCommander
import cflib.crtp

# Initialize drivers
cflib.crtp.init_drivers(enable_debug_driver=False)

URI = 'radio://0/80/2M/E7E7E7E7E7'

def test_linear_motion(scf):
    with MotionCommander(scf, default_height=0.5) as mc:
        time.sleep(2)  # hover stabilize

        print("Forward (+X)")
        mc.start_linear_motion(0.2, 0.0, 0.0)
        time.sleep(2)
        mc.stop()
        time.sleep(1)

        print("Backward (-X)")
        mc.start_linear_motion(-0.2, 0.0, 0.0)
        time.sleep(2)
        mc.stop()
        time.sleep(1)

        print("Left (+Y)")
        mc.start_linear_motion(0.0, 0.2, 0.0)
        time.sleep(2)
        mc.stop()
        time.sleep(1)

        print("Right (-Y)")
        mc.start_linear_motion(0.0, -0.2, 0.0)
        time.sleep(2)
        mc.stop()
        time.sleep(1)

        print("Done, landing...")
        time.sleep(2)

if __name__ == '__main__':
    with SyncCrazyflie(URI, cf=None) as scf:
        test_linear_motion(scf)
