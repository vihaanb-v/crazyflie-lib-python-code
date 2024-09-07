import logging
import sys
import time
from threading import Event

import cflib.crtp
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie
from cflib.positioning.motion_commander import MotionCommander
from cflib.utils import uri_helper

URI = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')

DEFAULT_HEIGHT = 1

deck_attached_event = Event()

logging.basicConfig(level=logging.ERROR)

def take_off_simple(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        time.sleep(3)
        mc.stop()

def traverse_square(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code using turns from bottom right corner of square
        for i in range(1):
            time.sleep(3)
            mc.move_distance(2, 2, 0.0, velocity=1.5)
            time.sleep(3)
            mc.turn_right(90)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.turn_right(90)
            mc.move_distance(2, -2, 0.0, velocity=1.5)
            time.sleep(3)
            mc.turn_left(90)
            mc.forward(2, velocity=1.5)
            mc.turn_left(90)
            time.sleep(3)
            mc.stop()

def param_deck_flow(name, value_str):
    value = int(value_str)
    print(value)
    if value:
        deck_attached_event.set()
        print('Deck is attached!')
    else:
        print('Deck is NOT attached!')

if __name__ == '__main__':
    cflib.crtp.init_drivers()

    
    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache='./cache')) as scf:

        scf.cf.param.add_update_callback(group='deck', name='bcFlow2',
                                         cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=5):
            print('No flow deck detected!')
            sys.exit(1)

        take_off_simple(scf)
        #traverse_square(scf)
    
    '''
    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache='./cache')) as scf:

        scf.cf.param.add_update_callback(group='deck', name='ai_deck',
                                         cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=5):
            print('No ai deck detected!')
            sys.exit(1)

        #take_off_simple(scf)
        #traverse_square(scf)
    '''