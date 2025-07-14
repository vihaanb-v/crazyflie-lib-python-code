import time
from cflib.crazyflie import Crazyflie
from cflib.crazyflie.log import LogConfig
from cflib.utils import uri_helper

uri = uri_helper.uri_from_env(default='radio://0/80/2M/E7E7E7E7E7')
cf = Crazyflie()

def connected_callback(link_uri):
    print(f"Connected to {link_uri}")

    log_config = LogConfig(name='HealthLog', period_in_ms=100)
    log_config.add_variable('pm.vbat', 'float')
    log_config.add_variable('health.motorPass', 'uint8_t')
    log_config.add_variable('health.batteryPass', 'uint8_t')

    def log_callback(timestamp, data, logconf):
        vbat = data['pm.vbat']
        motor_pass = data['health.motorPass']
        battery_pass = data['health.batteryPass']
        print(f"[{timestamp}] VBAT: {vbat:.2f}V | motorPass: {motor_pass} | batteryPass: {battery_pass}")

    def stop_logging(_):
        print("Finished logging. Disconnecting...")
        cf.close_link()

    log_config.data_received_cb.add_callback(log_callback)
    cf.log.add_config(log_config)
    log_config.start()

    # Stop after 10 seconds
    time.sleep(10)
    log_config.stop()
    cf.close_link()

cf.connected.add_callback(connected_callback)
cf.open_link(uri)

print("Connecting to drone...")
while cf.is_connected():
    time.sleep(1)
