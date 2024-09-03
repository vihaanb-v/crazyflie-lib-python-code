import logging
import time

import cflib.crtp
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.syncCrazyflie import SyncCrazyflie

from cflib.crazyflie.log import LogConfig
from cflib.crazyflie.log import LogVariable
from cflib.crazyflie.syncLogger import SyncLogger


# URI to the Crazyflie to connect to
uri = 'radio://0/80/2M/E7E7E7E7E7'

# Only output errors from the logging framework
logging.basicConfig(level=logging.ERROR)

def param_stab_est_callback(name, value):
    print('The crazyflie has parameter ' + name + ' set at number: ' + value)

def simple_param_async(scf, groupstr, namestr):
    cf = scf.cf
    full_name = groupstr + '.' + namestr

    cf.param.add_update_callback(group=groupstr, name=namestr,
                                 cb=param_stab_est_callback)
    time.sleep(1)
    cf.param.set_value(full_name, 2)
    time.sleep(1)
    cf.param.set_value(full_name, 1)
    time.sleep(1)

def log_stab_callback(timestamp, data, logconf):
    print('Timestamp: {}, Log Config Group Name: {}, Position: ({}, {}, {})'.format(timestamp, logconf.name, data['stateEstimate.x'], data['stateEstimate.y'], data['stateEstimate.z']))

def simple_log_async(scf, logconf):
    cf = scf.cf
    cf.log.add_config(logconf)
    logconf.data_received_cb.add_callback(log_stab_callback)
    logconf.start()
    time.sleep(5)

def simple_log(scf, logconf):
    with SyncLogger(scf, logconf) as logger:
        for log_entry in logger:
            timestamp = log_entry[0]
            data = log_entry[1]
            logconf_name = log_entry[2]
            print('[%d][%s]: %s' % (timestamp, logconf_name, data))
            break

# Function to set EKF parameters (if needed)
def set_ekf_parameters(cf):
    # Set EKF parameters, for example, setting the fusion mode
    cf.param.set_value('stabilizer.estimator', '2')  # '2' typically represents EKF in some firmware versions

if __name__ == '__main__':
    # Initialize the low-level drivers
    cflib.crtp.init_drivers()
    

    lg_stab = LogConfig(name='EKF', period_in_ms=10)
    lg_stab.add_variable('stateEstimate.x', 'float')
    lg_stab.add_variable('stateEstimate.y', 'float')
    lg_stab.add_variable('stateEstimate.z', 'float')

    #group = 'stabilizer'
    #name = 'estimator'

    with SyncCrazyflie(uri, cf=Crazyflie(rw_cache='./cache')) as scf:
        cf = scf.cf

        set_ekf_parameters(cf)

        simple_log_async(scf, lg_stab)

        #simple_param_async(scf, group, name)