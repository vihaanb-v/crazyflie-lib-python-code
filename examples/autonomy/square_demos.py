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
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        '''
        with SyncLogger(scf, lg_stab) as logger:
            # Iterate the logger to get the values
            count = 0
            for log_entry in logger:
                print(log_entry)
                # Do useful stuff
                count += 1
                if (count > 15):
                    # The logging will continue until you exit the loop
                    break
        
        print('hi')
        '''

        time.sleep(15)
        mc.stop()

def straight_line(scf):
    with MotionCommander(scf, default_height=DEFAULT_HEIGHT) as mc:
        #Code to fly 2 meters forward
        for i in range(1):
            time.sleep(3)
            mc.forward(2, velocity=1.5)
            time.sleep(3)
            mc.stop()

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

def drone_logging(scf, lg_stab):
    with SyncLogger(scf, lg_stab) as logger:
            # Iterate the logger to get the values
            count = 0
            for log_entry in logger:
                print(log_entry)
                # Do useful stuff
                count += 1
                if (count > 15):
                    # The logging will continue until you exit the loop
                    break

class Logging:
    """
    Simple logging example class that logs the Stabilizer from a supplied
    link uri and disconnects after 5s.
    """

    def __init__(self, link_uri):
        """ Initialize and run the example with the specified link_uri """

        self._cf = Crazyflie(rw_cache='./cache')

        # Connect some callbacks from the Crazyflie API
        self._cf.connected.add_callback(self._connected)
        self._cf.disconnected.add_callback(self._disconnected)
        self._cf.connection_failed.add_callback(self._connection_failed)
        self._cf.connection_lost.add_callback(self._connection_lost)

        print('Connecting to %s' % link_uri)

        # Try to connect to the Crazyflie
        self._cf.open_link(link_uri)

        # Variable used to keep main loop occupied until disconnect
        self.is_connected = True

    def _connected(self, link_uri):
        """ This callback is called form the Crazyflie API when a Crazyflie
        has been connected and the TOCs have been downloaded."""
        print('Connected to %s' % link_uri)

        # The definition of the logconfig can be made before connecting
        self._lg_stab = LogConfig(name='Stabilizer', period_in_ms=100)
        self._lg_stab.add_variable('stateEstimate.x', 'float')
        self._lg_stab.add_variable('stateEstimate.y', 'float')
        self._lg_stab.add_variable('stateEstimate.z', 'float')
        #self._lg_stab.add_variable('stabilizer.roll', 'float')
        #self._lg_stab.add_variable('stabilizer.pitch', 'float')
        #self._lg_stab.add_variable('stabilizer.yaw', 'float')
        # The fetch-as argument can be set to FP16 to save space in the log packet
        #self._lg_stab.add_variable('pm.vbat', 'FP16')

        # Adding the configuration cannot be done until a Crazyflie is
        # connected, since we need to check that the variables we
        # would like to log are in the TOC.
        try:
            self._cf.log.add_config(self._lg_stab)
            # This callback will receive the data
            self._lg_stab.data_received_cb.add_callback(self._stab_log_data)
            # This callback will be called on errors
            self._lg_stab.error_cb.add_callback(self._stab_log_error)
            # Start the logging
            self._lg_stab.start()
        except KeyError as e:
            print('Could not start log configuration,'
                  '{} not found in TOC'.format(str(e)))
        except AttributeError:
            print('Could not add Stabilizer log config, bad configuration.')

        # Start a timer to disconnect in 10s
        t = Timer(2.5, self._cf.close_link)
        t.start()

    def _stab_log_error(self, logconf, msg):
        """Callback from the log API when an error occurs"""
        print('Error when logging %s: %s' % (logconf.name, msg))

    def _stab_log_data(self, timestamp, data, logconf):
        """Callback from a the log API when data arrives"""
        print(f'[{timestamp}][{logconf.name}]: ', end='')
        for name, value in data.items():
            print(f'{name}: {value:3.3f} ', end='')
        print()

    def _connection_failed(self, link_uri, msg):
        """Callback when connection initial connection fails (i.e no Crazyflie
        at the specified address)"""
        print('Connection to %s failed: %s' % (link_uri, msg))
        self.is_connected = False

    def _connection_lost(self, link_uri, msg):
        """Callback when disconnected after a connection has been made (i.e
        Crazyflie moves out of range)"""
        print('Connection to %s lost: %s' % (link_uri, msg))

    def _disconnected(self, link_uri):
        """Callback when the Crazyflie is disconnected (called in all cases)"""
        print('Disconnected from %s' % link_uri)
        self.is_connected = False

if __name__ == '__main__':
    cflib.crtp.init_drivers()

    #le = Logging(URI)

    #while le.is_connected:
    #   time.sleep(1)

    with SyncCrazyflie(URI, cf=Crazyflie(rw_cache= './cache')) as scf:

        scf.cf.param.add_update_callback(group='deck', name='bcFlow2',
                                         cb=param_deck_flow)
        time.sleep(1)

        if not deck_attached_event.wait(timeout=1):
            print('No flow deck detected!')
            sys.exit(1)

        lg_stab = LogConfig(name='Stabilizer', period_in_ms=1000)
        lg_stab.add_variable('stateEstimate.x', 'float')
        lg_stab.add_variable('stateEstimate.y', 'float')
        lg_stab.add_variable('stateEstimate.z', 'float')

        t1 = threading.Thread(target=take_off_simple, args=(scf, lg_stab))
        t2 = threading.Thread(target=drone_logging, args=(scf, lg_stab))

        t1.start()
        t2.start()

        t1.join()
        t2.join()

        print("Logging & Flight done!")

        #take_off_simple(scf, lg_stab)
        #straight_line(scf)
        #square_turns(scf)
        #square_no_turns(scf)
        #square_turns_no_stops(scf)
        #square_no_turns_no_stops(scf)

        #le = Logging(URI)

        #while le.is_connected:
        #    time.sleep(1)