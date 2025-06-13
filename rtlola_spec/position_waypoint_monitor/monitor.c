#include <math.h>
#include "monitor.h"
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
double lat_sync(Memory* memory){
return memory->boundedbuffer_lat.values[memory->boundedbuffer_lat.current];
}
double lon_sync(Memory* memory){
return memory->boundedbuffer_lon.values[memory->boundedbuffer_lon.current];
}
double velocity_sync(Memory* memory){
return memory->boundedbuffer_velocity.values[memory->boundedbuffer_velocity.current];
}
double velocity_kmh_sync(Memory* memory){
return memory->boundedbuffer_velocity_kmh.values[memory->boundedbuffer_velocity_kmh.current];
}
double current_lat_sync(Memory* memory){
return memory->boundedbuffer_current_lat.values[memory->boundedbuffer_current_lat.current];
}
double current_long_sync(Memory* memory){
return memory->boundedbuffer_current_long.values[memory->boundedbuffer_current_long.current];
}
uint64_t waypoint_index_sync(Memory* memory){
return memory->boundedbuffer_waypoint_index.values[memory->boundedbuffer_waypoint_index.current];
}
TUPLE2_DOUBLE__DOUBLE current_waypoint_sync(Memory* memory){
return memory->boundedbuffer_current_waypoint.values[memory->boundedbuffer_current_waypoint.current];
}
double waypoint_lat_sync(Memory* memory){
return memory->boundedbuffer_waypoint_lat.values[memory->boundedbuffer_waypoint_lat.current];
}
double waypoint_long_sync(Memory* memory){
return memory->boundedbuffer_waypoint_long.values[memory->boundedbuffer_waypoint_long.current];
}
double current_lat_rad_sync(Memory* memory){
return memory->boundedbuffer_current_lat_rad.values[memory->boundedbuffer_current_lat_rad.current];
}
double current_long_rad_sync(Memory* memory){
return memory->boundedbuffer_current_long_rad.values[memory->boundedbuffer_current_long_rad.current];
}
double waypoint_lat_rad_sync(Memory* memory){
return memory->boundedbuffer_waypoint_lat_rad.values[memory->boundedbuffer_waypoint_lat_rad.current];
}
double waypoint_long_rad_sync(Memory* memory){
return memory->boundedbuffer_waypoint_long_rad.values[memory->boundedbuffer_waypoint_long_rad.current];
}
double mean_latitude_sync(Memory* memory){
return memory->boundedbuffer_mean_latitude.values[memory->boundedbuffer_mean_latitude.current];
}
double distance_to_waypoint_lat_sync(Memory* memory){
return memory->boundedbuffer_distance_to_waypoint_lat.values[memory->boundedbuffer_distance_to_waypoint_lat.current];
}
double distance_to_waypoint_long_sync(Memory* memory){
return memory->boundedbuffer_distance_to_waypoint_long.values[memory->boundedbuffer_distance_to_waypoint_long.current];
}
double distance_to_waypoint_sync(Memory* memory){
return memory->boundedbuffer_distance_to_waypoint.values[memory->boundedbuffer_distance_to_waypoint.current];
}
double waypoint_r_sync(Memory* memory){
return memory->boundedbuffer_waypoint_r.values[memory->boundedbuffer_waypoint_r.current];
}
double waypoint_phi_sync(Memory* memory){
return memory->boundedbuffer_waypoint_phi.values[memory->boundedbuffer_waypoint_phi.current];
}
double waypoint_direction_sync(Memory* memory){
return memory->boundedbuffer_waypoint_direction.values[memory->boundedbuffer_waypoint_direction.current];
}
bool waypoint_reached_sync(Memory* memory){
return memory->boundedbuffer_waypoint_reached.values[memory->boundedbuffer_waypoint_reached.current];
}
bool finished_sync(Memory* memory){
return memory->boundedbuffer_finished.values[memory->boundedbuffer_finished.current];
}
double estimated_time_until_waypoint_sync(Memory* memory){
return memory->boundedbuffer_estimated_time_until_waypoint.values[memory->boundedbuffer_estimated_time_until_waypoint.current];
}
uint64_t waypoint_index_offset(Memory* memory, int offset, uint64_t def){
int i = (memory->boundedbuffer_waypoint_index.current - offset + 2) % 2;
if (memory->boundedbuffer_waypoint_index.valid[i])
return memory->boundedbuffer_waypoint_index.values[i];
else
return def;
}
bool reset_waypoints_get(Memory* memory, bool def){
if (memory->boundedbuffer_reset_waypoints.is_fresh) return memory->boundedbuffer_reset_waypoints.values[memory->boundedbuffer_reset_waypoints.current]; else return def;
}
bool skip_waypoint_get(Memory* memory, bool def){
if (memory->boundedbuffer_skip_waypoint.is_fresh) return memory->boundedbuffer_skip_waypoint.values[memory->boundedbuffer_skip_waypoint.current]; else return def;
}
bool waypoint_reached_get(Memory* memory, bool def){
if (memory->boundedbuffer_waypoint_reached.is_fresh) return memory->boundedbuffer_waypoint_reached.values[memory->boundedbuffer_waypoint_reached.current]; else return def;
}
bool velocity_kmh_is_fresh(Memory* memory){
return memory->boundedbuffer_velocity_kmh.is_fresh;
}
bool current_lat_is_fresh(Memory* memory){
return memory->boundedbuffer_current_lat.is_fresh;
}
bool current_long_is_fresh(Memory* memory){
return memory->boundedbuffer_current_long.is_fresh;
}
bool waypoint_index_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_index.is_fresh;
}
bool current_waypoint_is_fresh(Memory* memory){
return memory->boundedbuffer_current_waypoint.is_fresh;
}
bool waypoint_lat_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_lat.is_fresh;
}
bool waypoint_long_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_long.is_fresh;
}
bool current_lat_rad_is_fresh(Memory* memory){
return memory->boundedbuffer_current_lat_rad.is_fresh;
}
bool current_long_rad_is_fresh(Memory* memory){
return memory->boundedbuffer_current_long_rad.is_fresh;
}
bool waypoint_lat_rad_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_lat_rad.is_fresh;
}
bool waypoint_long_rad_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_long_rad.is_fresh;
}
bool mean_latitude_is_fresh(Memory* memory){
return memory->boundedbuffer_mean_latitude.is_fresh;
}
bool distance_to_waypoint_lat_is_fresh(Memory* memory){
return memory->boundedbuffer_distance_to_waypoint_lat.is_fresh;
}
bool distance_to_waypoint_long_is_fresh(Memory* memory){
return memory->boundedbuffer_distance_to_waypoint_long.is_fresh;
}
bool distance_to_waypoint_is_fresh(Memory* memory){
return memory->boundedbuffer_distance_to_waypoint.is_fresh;
}
bool waypoint_r_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_r.is_fresh;
}
bool waypoint_phi_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_phi.is_fresh;
}
bool waypoint_direction_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_direction.is_fresh;
}
bool waypoint_reached_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_reached.is_fresh;
}
bool finished_is_fresh(Memory* memory){
return memory->boundedbuffer_finished.is_fresh;
}
bool estimated_time_until_waypoint_is_fresh(Memory* memory){
return memory->boundedbuffer_estimated_time_until_waypoint.is_fresh;
}
void input_lat(Memory* memory, double new_value){
memory->boundedbuffer_lat.values[memory->boundedbuffer_lat.current] = new_value;
memory->boundedbuffer_lat.valid[memory->boundedbuffer_lat.current] = 1;
memory->boundedbuffer_lat.is_fresh = 1;
}
void input_lon(Memory* memory, double new_value){
memory->boundedbuffer_lon.values[memory->boundedbuffer_lon.current] = new_value;
memory->boundedbuffer_lon.valid[memory->boundedbuffer_lon.current] = 1;
memory->boundedbuffer_lon.is_fresh = 1;
}
void input_heading(Memory* memory, double new_value){
memory->boundedbuffer_heading.values[memory->boundedbuffer_heading.current] = new_value;
memory->boundedbuffer_heading.valid[memory->boundedbuffer_heading.current] = 1;
memory->boundedbuffer_heading.is_fresh = 1;
}
void input_pitch(Memory* memory, double new_value){
memory->boundedbuffer_pitch.values[memory->boundedbuffer_pitch.current] = new_value;
memory->boundedbuffer_pitch.valid[memory->boundedbuffer_pitch.current] = 1;
memory->boundedbuffer_pitch.is_fresh = 1;
}
void input_bank(Memory* memory, double new_value){
memory->boundedbuffer_bank.values[memory->boundedbuffer_bank.current] = new_value;
memory->boundedbuffer_bank.valid[memory->boundedbuffer_bank.current] = 1;
memory->boundedbuffer_bank.is_fresh = 1;
}
void input_velocity(Memory* memory, double new_value){
memory->boundedbuffer_velocity.values[memory->boundedbuffer_velocity.current] = new_value;
memory->boundedbuffer_velocity.valid[memory->boundedbuffer_velocity.current] = 1;
memory->boundedbuffer_velocity.is_fresh = 1;
}
void input_airspeed(Memory* memory, double new_value){
memory->boundedbuffer_airspeed.values[memory->boundedbuffer_airspeed.current] = new_value;
memory->boundedbuffer_airspeed.valid[memory->boundedbuffer_airspeed.current] = 1;
memory->boundedbuffer_airspeed.is_fresh = 1;
}
void input_vertical_speed(Memory* memory, double new_value){
memory->boundedbuffer_vertical_speed.values[memory->boundedbuffer_vertical_speed.current] = new_value;
memory->boundedbuffer_vertical_speed.valid[memory->boundedbuffer_vertical_speed.current] = 1;
memory->boundedbuffer_vertical_speed.is_fresh = 1;
}
void input_acceleration_x(Memory* memory, double new_value){
memory->boundedbuffer_acceleration_x.values[memory->boundedbuffer_acceleration_x.current] = new_value;
memory->boundedbuffer_acceleration_x.valid[memory->boundedbuffer_acceleration_x.current] = 1;
memory->boundedbuffer_acceleration_x.is_fresh = 1;
}
void input_acceleration_y(Memory* memory, double new_value){
memory->boundedbuffer_acceleration_y.values[memory->boundedbuffer_acceleration_y.current] = new_value;
memory->boundedbuffer_acceleration_y.valid[memory->boundedbuffer_acceleration_y.current] = 1;
memory->boundedbuffer_acceleration_y.is_fresh = 1;
}
void input_acceleration_z(Memory* memory, double new_value){
memory->boundedbuffer_acceleration_z.values[memory->boundedbuffer_acceleration_z.current] = new_value;
memory->boundedbuffer_acceleration_z.valid[memory->boundedbuffer_acceleration_z.current] = 1;
memory->boundedbuffer_acceleration_z.is_fresh = 1;
}
void input_altitude(Memory* memory, double new_value){
memory->boundedbuffer_altitude.values[memory->boundedbuffer_altitude.current] = new_value;
memory->boundedbuffer_altitude.valid[memory->boundedbuffer_altitude.current] = 1;
memory->boundedbuffer_altitude.is_fresh = 1;
}
void input_reset_waypoints(Memory* memory, bool new_value){
memory->boundedbuffer_reset_waypoints.values[memory->boundedbuffer_reset_waypoints.current] = new_value;
memory->boundedbuffer_reset_waypoints.valid[memory->boundedbuffer_reset_waypoints.current] = 1;
memory->boundedbuffer_reset_waypoints.is_fresh = 1;
}
void input_skip_waypoint(Memory* memory, bool new_value){
memory->boundedbuffer_skip_waypoint.values[memory->boundedbuffer_skip_waypoint.current] = new_value;
memory->boundedbuffer_skip_waypoint.valid[memory->boundedbuffer_skip_waypoint.current] = 1;
memory->boundedbuffer_skip_waypoint.is_fresh = 1;
}
void eval_velocity_kmh_0(Memory* memory){
double new_value = (velocity_sync(memory) * 1.853);
memory->boundedbuffer_velocity_kmh.values[memory->boundedbuffer_velocity_kmh.current] = new_value;
memory->boundedbuffer_velocity_kmh.valid[memory->boundedbuffer_velocity_kmh.current] = 1;
memory->boundedbuffer_velocity_kmh.is_fresh = 1;
}
void eval_current_lat_0(Memory* memory){
double new_value = lat_sync(memory);
memory->boundedbuffer_current_lat.values[memory->boundedbuffer_current_lat.current] = new_value;
memory->boundedbuffer_current_lat.valid[memory->boundedbuffer_current_lat.current] = 1;
memory->boundedbuffer_current_lat.is_fresh = 1;
}
void eval_current_long_0(Memory* memory){
double new_value = lon_sync(memory);
memory->boundedbuffer_current_long.values[memory->boundedbuffer_current_long.current] = new_value;
memory->boundedbuffer_current_long.valid[memory->boundedbuffer_current_long.current] = 1;
memory->boundedbuffer_current_long.is_fresh = 1;
}
void eval_waypoint_index_0(Memory* memory){
uint64_t new_value = (reset_waypoints_get(memory, false)?0:((skip_waypoint_get(memory, false) || waypoint_reached_get(memory, false))?((waypoint_index_offset(memory, 1, 0) + 1) % 10):waypoint_index_offset(memory, 1, 0)));
memory->boundedbuffer_waypoint_index.values[memory->boundedbuffer_waypoint_index.current] = new_value;
memory->boundedbuffer_waypoint_index.valid[memory->boundedbuffer_waypoint_index.current] = 1;
memory->boundedbuffer_waypoint_index.is_fresh = 1;
}
void eval_current_waypoint_0(Memory* memory){
TUPLE2_DOUBLE__DOUBLE new_value = ((waypoint_index_offset(memory, 1, 0) == 0)?(TUPLE2_DOUBLE__DOUBLE){._0=49.21444, ._1=7.11135}:((waypoint_index_offset(memory, 1, 0) == 1)?(TUPLE2_DOUBLE__DOUBLE){._0=49.21802, ._1=7.08268}:((waypoint_index_offset(memory, 1, 0) == 2)?(TUPLE2_DOUBLE__DOUBLE){._0=49.23394, ._1=7.06602}:((waypoint_index_offset(memory, 1, 0) == 3)?(TUPLE2_DOUBLE__DOUBLE){._0=49.25944, ._1=7.0699}:((waypoint_index_offset(memory, 1, 0) == 4)?(TUPLE2_DOUBLE__DOUBLE){._0=49.26683, ._1=7.10939}:((waypoint_index_offset(memory, 1, 0) == 5)?(TUPLE2_DOUBLE__DOUBLE){._0=49.26683, ._1=7.15095}:((waypoint_index_offset(memory, 1, 0) == 6)?(TUPLE2_DOUBLE__DOUBLE){._0=49.25631, ._1=7.1767}:((waypoint_index_offset(memory, 1, 0) == 7)?(TUPLE2_DOUBLE__DOUBLE){._0=49.23749, ._1=7.17911}:((waypoint_index_offset(memory, 1, 0) == 8)?(TUPLE2_DOUBLE__DOUBLE){._0=49.21418, ._1=7.16709}:((waypoint_index_offset(memory, 1, 0) == 9)?(TUPLE2_DOUBLE__DOUBLE){._0=49.20409, ._1=7.12897}:(TUPLE2_DOUBLE__DOUBLE){._0=0, ._1=0}))))))))));
memory->boundedbuffer_current_waypoint.values[memory->boundedbuffer_current_waypoint.current] = new_value;
memory->boundedbuffer_current_waypoint.valid[memory->boundedbuffer_current_waypoint.current] = 1;
memory->boundedbuffer_current_waypoint.is_fresh = 1;
}
void eval_waypoint_lat_0(Memory* memory){
double new_value = (current_waypoint_sync(memory))._0;
memory->boundedbuffer_waypoint_lat.values[memory->boundedbuffer_waypoint_lat.current] = new_value;
memory->boundedbuffer_waypoint_lat.valid[memory->boundedbuffer_waypoint_lat.current] = 1;
memory->boundedbuffer_waypoint_lat.is_fresh = 1;
}
void eval_waypoint_long_0(Memory* memory){
double new_value = (current_waypoint_sync(memory))._1;
memory->boundedbuffer_waypoint_long.values[memory->boundedbuffer_waypoint_long.current] = new_value;
memory->boundedbuffer_waypoint_long.valid[memory->boundedbuffer_waypoint_long.current] = 1;
memory->boundedbuffer_waypoint_long.is_fresh = 1;
}
void eval_current_lat_rad_0(Memory* memory){
double new_value = ((current_lat_sync(memory) * 3.14519) / 180);
memory->boundedbuffer_current_lat_rad.values[memory->boundedbuffer_current_lat_rad.current] = new_value;
memory->boundedbuffer_current_lat_rad.valid[memory->boundedbuffer_current_lat_rad.current] = 1;
memory->boundedbuffer_current_lat_rad.is_fresh = 1;
}
void eval_current_long_rad_0(Memory* memory){
double new_value = ((current_long_sync(memory) * 3.14519) / 180);
memory->boundedbuffer_current_long_rad.values[memory->boundedbuffer_current_long_rad.current] = new_value;
memory->boundedbuffer_current_long_rad.valid[memory->boundedbuffer_current_long_rad.current] = 1;
memory->boundedbuffer_current_long_rad.is_fresh = 1;
}
void eval_waypoint_lat_rad_0(Memory* memory){
double new_value = ((waypoint_lat_sync(memory) * 3.14519) / 180);
memory->boundedbuffer_waypoint_lat_rad.values[memory->boundedbuffer_waypoint_lat_rad.current] = new_value;
memory->boundedbuffer_waypoint_lat_rad.valid[memory->boundedbuffer_waypoint_lat_rad.current] = 1;
memory->boundedbuffer_waypoint_lat_rad.is_fresh = 1;
}
void eval_waypoint_long_rad_0(Memory* memory){
double new_value = ((waypoint_long_sync(memory) * 3.14519) / 180);
memory->boundedbuffer_waypoint_long_rad.values[memory->boundedbuffer_waypoint_long_rad.current] = new_value;
memory->boundedbuffer_waypoint_long_rad.valid[memory->boundedbuffer_waypoint_long_rad.current] = 1;
memory->boundedbuffer_waypoint_long_rad.is_fresh = 1;
}
void eval_mean_latitude_0(Memory* memory){
double new_value = ((waypoint_lat_rad_sync(memory) + current_lat_rad_sync(memory)) / 2);
memory->boundedbuffer_mean_latitude.values[memory->boundedbuffer_mean_latitude.current] = new_value;
memory->boundedbuffer_mean_latitude.valid[memory->boundedbuffer_mean_latitude.current] = 1;
memory->boundedbuffer_mean_latitude.is_fresh = 1;
}
void eval_distance_to_waypoint_lat_0(Memory* memory){
double new_value = (waypoint_lat_rad_sync(memory) - current_lat_rad_sync(memory));
memory->boundedbuffer_distance_to_waypoint_lat.values[memory->boundedbuffer_distance_to_waypoint_lat.current] = new_value;
memory->boundedbuffer_distance_to_waypoint_lat.valid[memory->boundedbuffer_distance_to_waypoint_lat.current] = 1;
memory->boundedbuffer_distance_to_waypoint_lat.is_fresh = 1;
}
void eval_distance_to_waypoint_long_0(Memory* memory){
double new_value = (waypoint_long_rad_sync(memory) - current_long_rad_sync(memory));
memory->boundedbuffer_distance_to_waypoint_long.values[memory->boundedbuffer_distance_to_waypoint_long.current] = new_value;
memory->boundedbuffer_distance_to_waypoint_long.valid[memory->boundedbuffer_distance_to_waypoint_long.current] = 1;
memory->boundedbuffer_distance_to_waypoint_long.is_fresh = 1;
}
void eval_distance_to_waypoint_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(distance_to_waypoint_long_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * distance_to_waypoint_lat_sync(memory)), 2))));
memory->boundedbuffer_distance_to_waypoint.values[memory->boundedbuffer_distance_to_waypoint.current] = new_value;
memory->boundedbuffer_distance_to_waypoint.valid[memory->boundedbuffer_distance_to_waypoint.current] = 1;
memory->boundedbuffer_distance_to_waypoint.is_fresh = 1;
}
void eval_waypoint_r_0(Memory* memory){
double new_value = sqrt((pow(distance_to_waypoint_long_sync(memory), 2) + pow(distance_to_waypoint_lat_sync(memory), 2)));
memory->boundedbuffer_waypoint_r.values[memory->boundedbuffer_waypoint_r.current] = new_value;
memory->boundedbuffer_waypoint_r.valid[memory->boundedbuffer_waypoint_r.current] = 1;
memory->boundedbuffer_waypoint_r.is_fresh = 1;
}
void eval_waypoint_phi_0(Memory* memory){
double new_value = ((waypoint_r_sync(memory) == 0)?0:((distance_to_waypoint_long_sync(memory) >= 0)?acos((distance_to_waypoint_lat_sync(memory) / waypoint_r_sync(memory))):(-acos((distance_to_waypoint_lat_sync(memory) / waypoint_r_sync(memory))))));
memory->boundedbuffer_waypoint_phi.values[memory->boundedbuffer_waypoint_phi.current] = new_value;
memory->boundedbuffer_waypoint_phi.valid[memory->boundedbuffer_waypoint_phi.current] = 1;
memory->boundedbuffer_waypoint_phi.is_fresh = 1;
}
void eval_waypoint_direction_0(Memory* memory){
double new_value = ((waypoint_phi_sync(memory) / (2 * 3.14519)) * 360);
memory->boundedbuffer_waypoint_direction.values[memory->boundedbuffer_waypoint_direction.current] = new_value;
memory->boundedbuffer_waypoint_direction.valid[memory->boundedbuffer_waypoint_direction.current] = 1;
memory->boundedbuffer_waypoint_direction.is_fresh = 1;
}
void eval_waypoint_reached_0(Memory* memory){
bool new_value = (distance_to_waypoint_sync(memory) < 100);
memory->boundedbuffer_waypoint_reached.values[memory->boundedbuffer_waypoint_reached.current] = new_value;
memory->boundedbuffer_waypoint_reached.valid[memory->boundedbuffer_waypoint_reached.current] = 1;
memory->boundedbuffer_waypoint_reached.is_fresh = 1;
}
void eval_finished_0(Memory* memory){
bool new_value = (waypoint_reached_sync(memory) && (waypoint_index_sync(memory) == 0));
memory->boundedbuffer_finished.values[memory->boundedbuffer_finished.current] = new_value;
memory->boundedbuffer_finished.valid[memory->boundedbuffer_finished.current] = 1;
memory->boundedbuffer_finished.is_fresh = 1;
}
void eval_estimated_time_until_waypoint_0(Memory* memory){
double new_value = ((velocity_kmh_sync(memory) == 0)?5000:(distance_to_waypoint_sync(memory) / (velocity_kmh_sync(memory) / 3.6)));
memory->boundedbuffer_estimated_time_until_waypoint.values[memory->boundedbuffer_estimated_time_until_waypoint.current] = new_value;
memory->boundedbuffer_estimated_time_until_waypoint.valid[memory->boundedbuffer_estimated_time_until_waypoint.current] = 1;
memory->boundedbuffer_estimated_time_until_waypoint.is_fresh = 1;
}
void shift_lat(Memory* memory){
memory->boundedbuffer_lat.current = (memory->boundedbuffer_lat.current + 1) % 1;
}
void shift_lon(Memory* memory){
memory->boundedbuffer_lon.current = (memory->boundedbuffer_lon.current + 1) % 1;
}
void shift_heading(Memory* memory){
memory->boundedbuffer_heading.current = (memory->boundedbuffer_heading.current + 1) % 1;
}
void shift_pitch(Memory* memory){
memory->boundedbuffer_pitch.current = (memory->boundedbuffer_pitch.current + 1) % 1;
}
void shift_bank(Memory* memory){
memory->boundedbuffer_bank.current = (memory->boundedbuffer_bank.current + 1) % 1;
}
void shift_velocity(Memory* memory){
memory->boundedbuffer_velocity.current = (memory->boundedbuffer_velocity.current + 1) % 1;
}
void shift_airspeed(Memory* memory){
memory->boundedbuffer_airspeed.current = (memory->boundedbuffer_airspeed.current + 1) % 1;
}
void shift_vertical_speed(Memory* memory){
memory->boundedbuffer_vertical_speed.current = (memory->boundedbuffer_vertical_speed.current + 1) % 1;
}
void shift_acceleration_x(Memory* memory){
memory->boundedbuffer_acceleration_x.current = (memory->boundedbuffer_acceleration_x.current + 1) % 1;
}
void shift_acceleration_y(Memory* memory){
memory->boundedbuffer_acceleration_y.current = (memory->boundedbuffer_acceleration_y.current + 1) % 1;
}
void shift_acceleration_z(Memory* memory){
memory->boundedbuffer_acceleration_z.current = (memory->boundedbuffer_acceleration_z.current + 1) % 1;
}
void shift_altitude(Memory* memory){
memory->boundedbuffer_altitude.current = (memory->boundedbuffer_altitude.current + 1) % 1;
}
void shift_reset_waypoints(Memory* memory){
memory->boundedbuffer_reset_waypoints.current = (memory->boundedbuffer_reset_waypoints.current + 1) % 1;
}
void shift_skip_waypoint(Memory* memory){
memory->boundedbuffer_skip_waypoint.current = (memory->boundedbuffer_skip_waypoint.current + 1) % 1;
}
void shift_velocity_kmh(Memory* memory){
memory->boundedbuffer_velocity_kmh.current = (memory->boundedbuffer_velocity_kmh.current + 1) % 1;
}
void shift_current_lat(Memory* memory){
memory->boundedbuffer_current_lat.current = (memory->boundedbuffer_current_lat.current + 1) % 1;
}
void shift_current_long(Memory* memory){
memory->boundedbuffer_current_long.current = (memory->boundedbuffer_current_long.current + 1) % 1;
}
void shift_waypoint_index(Memory* memory){
memory->boundedbuffer_waypoint_index.current = (memory->boundedbuffer_waypoint_index.current + 1) % 2;
}
void shift_current_waypoint(Memory* memory){
memory->boundedbuffer_current_waypoint.current = (memory->boundedbuffer_current_waypoint.current + 1) % 1;
}
void shift_waypoint_lat(Memory* memory){
memory->boundedbuffer_waypoint_lat.current = (memory->boundedbuffer_waypoint_lat.current + 1) % 1;
}
void shift_waypoint_long(Memory* memory){
memory->boundedbuffer_waypoint_long.current = (memory->boundedbuffer_waypoint_long.current + 1) % 1;
}
void shift_current_lat_rad(Memory* memory){
memory->boundedbuffer_current_lat_rad.current = (memory->boundedbuffer_current_lat_rad.current + 1) % 1;
}
void shift_current_long_rad(Memory* memory){
memory->boundedbuffer_current_long_rad.current = (memory->boundedbuffer_current_long_rad.current + 1) % 1;
}
void shift_waypoint_lat_rad(Memory* memory){
memory->boundedbuffer_waypoint_lat_rad.current = (memory->boundedbuffer_waypoint_lat_rad.current + 1) % 1;
}
void shift_waypoint_long_rad(Memory* memory){
memory->boundedbuffer_waypoint_long_rad.current = (memory->boundedbuffer_waypoint_long_rad.current + 1) % 1;
}
void shift_mean_latitude(Memory* memory){
memory->boundedbuffer_mean_latitude.current = (memory->boundedbuffer_mean_latitude.current + 1) % 1;
}
void shift_distance_to_waypoint_lat(Memory* memory){
memory->boundedbuffer_distance_to_waypoint_lat.current = (memory->boundedbuffer_distance_to_waypoint_lat.current + 1) % 1;
}
void shift_distance_to_waypoint_long(Memory* memory){
memory->boundedbuffer_distance_to_waypoint_long.current = (memory->boundedbuffer_distance_to_waypoint_long.current + 1) % 1;
}
void shift_distance_to_waypoint(Memory* memory){
memory->boundedbuffer_distance_to_waypoint.current = (memory->boundedbuffer_distance_to_waypoint.current + 1) % 1;
}
void shift_waypoint_r(Memory* memory){
memory->boundedbuffer_waypoint_r.current = (memory->boundedbuffer_waypoint_r.current + 1) % 1;
}
void shift_waypoint_phi(Memory* memory){
memory->boundedbuffer_waypoint_phi.current = (memory->boundedbuffer_waypoint_phi.current + 1) % 1;
}
void shift_waypoint_direction(Memory* memory){
memory->boundedbuffer_waypoint_direction.current = (memory->boundedbuffer_waypoint_direction.current + 1) % 1;
}
void shift_waypoint_reached(Memory* memory){
memory->boundedbuffer_waypoint_reached.current = (memory->boundedbuffer_waypoint_reached.current + 1) % 1;
}
void shift_finished(Memory* memory){
memory->boundedbuffer_finished.current = (memory->boundedbuffer_finished.current + 1) % 1;
}
void shift_estimated_time_until_waypoint(Memory* memory){
memory->boundedbuffer_estimated_time_until_waypoint.current = (memory->boundedbuffer_estimated_time_until_waypoint.current + 1) % 1;
}
Verdict build_verdict(Memory* memory){
Verdict verdict;
memset(&verdict, 0, sizeof(verdict));
if (velocity_kmh_is_fresh(memory)) {
verdict.velocity_kmh = velocity_kmh_sync(memory);verdict.velocity_kmh_is_present = 1;
}
if (current_lat_is_fresh(memory)) {
verdict.current_lat = current_lat_sync(memory);verdict.current_lat_is_present = 1;
}
if (current_long_is_fresh(memory)) {
verdict.current_long = current_long_sync(memory);verdict.current_long_is_present = 1;
}
if (waypoint_index_is_fresh(memory)) {
verdict.waypoint_index = waypoint_index_sync(memory);verdict.waypoint_index_is_present = 1;
}
if (current_waypoint_is_fresh(memory)) {
verdict.current_waypoint = current_waypoint_sync(memory);verdict.current_waypoint_is_present = 1;
}
if (waypoint_lat_is_fresh(memory)) {
verdict.waypoint_lat = waypoint_lat_sync(memory);verdict.waypoint_lat_is_present = 1;
}
if (waypoint_long_is_fresh(memory)) {
verdict.waypoint_long = waypoint_long_sync(memory);verdict.waypoint_long_is_present = 1;
}
if (current_lat_rad_is_fresh(memory)) {
verdict.current_lat_rad = current_lat_rad_sync(memory);verdict.current_lat_rad_is_present = 1;
}
if (current_long_rad_is_fresh(memory)) {
verdict.current_long_rad = current_long_rad_sync(memory);verdict.current_long_rad_is_present = 1;
}
if (waypoint_lat_rad_is_fresh(memory)) {
verdict.waypoint_lat_rad = waypoint_lat_rad_sync(memory);verdict.waypoint_lat_rad_is_present = 1;
}
if (waypoint_long_rad_is_fresh(memory)) {
verdict.waypoint_long_rad = waypoint_long_rad_sync(memory);verdict.waypoint_long_rad_is_present = 1;
}
if (mean_latitude_is_fresh(memory)) {
verdict.mean_latitude = mean_latitude_sync(memory);verdict.mean_latitude_is_present = 1;
}
if (distance_to_waypoint_lat_is_fresh(memory)) {
verdict.distance_to_waypoint_lat = distance_to_waypoint_lat_sync(memory);verdict.distance_to_waypoint_lat_is_present = 1;
}
if (distance_to_waypoint_long_is_fresh(memory)) {
verdict.distance_to_waypoint_long = distance_to_waypoint_long_sync(memory);verdict.distance_to_waypoint_long_is_present = 1;
}
if (distance_to_waypoint_is_fresh(memory)) {
verdict.distance_to_waypoint = distance_to_waypoint_sync(memory);verdict.distance_to_waypoint_is_present = 1;
}
if (waypoint_r_is_fresh(memory)) {
verdict.waypoint_r = waypoint_r_sync(memory);verdict.waypoint_r_is_present = 1;
}
if (waypoint_phi_is_fresh(memory)) {
verdict.waypoint_phi = waypoint_phi_sync(memory);verdict.waypoint_phi_is_present = 1;
}
if (waypoint_direction_is_fresh(memory)) {
verdict.waypoint_direction = waypoint_direction_sync(memory);verdict.waypoint_direction_is_present = 1;
}
if (waypoint_reached_is_fresh(memory)) {
verdict.waypoint_reached = waypoint_reached_sync(memory);verdict.waypoint_reached_is_present = 1;
}
if (finished_is_fresh(memory)) {
verdict.finished = finished_sync(memory);verdict.finished_is_present = 1;
}
if (estimated_time_until_waypoint_is_fresh(memory)) {
verdict.estimated_time_until_waypoint = estimated_time_until_waypoint_sync(memory);verdict.estimated_time_until_waypoint_is_present = 1;
}
verdict.time = memory->time;
return verdict;
}
void clear_activation(Memory* memory){
memory->boundedbuffer_lat.is_fresh = 0;
memory->boundedbuffer_lon.is_fresh = 0;
memory->boundedbuffer_heading.is_fresh = 0;
memory->boundedbuffer_pitch.is_fresh = 0;
memory->boundedbuffer_bank.is_fresh = 0;
memory->boundedbuffer_velocity.is_fresh = 0;
memory->boundedbuffer_airspeed.is_fresh = 0;
memory->boundedbuffer_vertical_speed.is_fresh = 0;
memory->boundedbuffer_acceleration_x.is_fresh = 0;
memory->boundedbuffer_acceleration_y.is_fresh = 0;
memory->boundedbuffer_acceleration_z.is_fresh = 0;
memory->boundedbuffer_altitude.is_fresh = 0;
memory->boundedbuffer_reset_waypoints.is_fresh = 0;
memory->boundedbuffer_skip_waypoint.is_fresh = 0;
memory->boundedbuffer_velocity_kmh.is_fresh = 0;
memory->boundedbuffer_current_lat.is_fresh = 0;
memory->boundedbuffer_current_long.is_fresh = 0;
memory->boundedbuffer_waypoint_index.is_fresh = 0;
memory->boundedbuffer_current_waypoint.is_fresh = 0;
memory->boundedbuffer_waypoint_lat.is_fresh = 0;
memory->boundedbuffer_waypoint_long.is_fresh = 0;
memory->boundedbuffer_current_lat_rad.is_fresh = 0;
memory->boundedbuffer_current_long_rad.is_fresh = 0;
memory->boundedbuffer_waypoint_lat_rad.is_fresh = 0;
memory->boundedbuffer_waypoint_long_rad.is_fresh = 0;
memory->boundedbuffer_mean_latitude.is_fresh = 0;
memory->boundedbuffer_distance_to_waypoint_lat.is_fresh = 0;
memory->boundedbuffer_distance_to_waypoint_long.is_fresh = 0;
memory->boundedbuffer_distance_to_waypoint.is_fresh = 0;
memory->boundedbuffer_waypoint_r.is_fresh = 0;
memory->boundedbuffer_waypoint_phi.is_fresh = 0;
memory->boundedbuffer_waypoint_direction.is_fresh = 0;
memory->boundedbuffer_waypoint_reached.is_fresh = 0;
memory->boundedbuffer_finished.is_fresh = 0;
memory->boundedbuffer_estimated_time_until_waypoint.is_fresh = 0;
}
Verdict cycle(Memory* memory, InternalEvent internalevent){
memory->time = internalevent.time;
if (internalevent.lat_is_present) {
shift_lat(memory);
input_lat(memory, internalevent.lat);
}
if (internalevent.lon_is_present) {
shift_lon(memory);
input_lon(memory, internalevent.lon);
}
if (internalevent.heading_is_present) {
shift_heading(memory);
input_heading(memory, internalevent.heading);
}
if (internalevent.pitch_is_present) {
shift_pitch(memory);
input_pitch(memory, internalevent.pitch);
}
if (internalevent.bank_is_present) {
shift_bank(memory);
input_bank(memory, internalevent.bank);
}
if (internalevent.velocity_is_present) {
shift_velocity(memory);
input_velocity(memory, internalevent.velocity);
}
if (internalevent.airspeed_is_present) {
shift_airspeed(memory);
input_airspeed(memory, internalevent.airspeed);
}
if (internalevent.vertical_speed_is_present) {
shift_vertical_speed(memory);
input_vertical_speed(memory, internalevent.vertical_speed);
}
if (internalevent.acceleration_x_is_present) {
shift_acceleration_x(memory);
input_acceleration_x(memory, internalevent.acceleration_x);
}
if (internalevent.acceleration_y_is_present) {
shift_acceleration_y(memory);
input_acceleration_y(memory, internalevent.acceleration_y);
}
if (internalevent.acceleration_z_is_present) {
shift_acceleration_z(memory);
input_acceleration_z(memory, internalevent.acceleration_z);
}
if (internalevent.altitude_is_present) {
shift_altitude(memory);
input_altitude(memory, internalevent.altitude);
}
if (internalevent.reset_waypoints_is_present) {
shift_reset_waypoints(memory);
input_reset_waypoints(memory, internalevent.reset_waypoints);
}
if (internalevent.skip_waypoint_is_present) {
shift_skip_waypoint(memory);
input_skip_waypoint(memory, internalevent.skip_waypoint);
}
if (internalevent.velocity_is_present) {
shift_velocity_kmh(memory);
}
if (internalevent.lat_is_present) {
shift_current_lat(memory);
}
if (internalevent.lon_is_present) {
shift_current_long(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_index(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_current_waypoint(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_lat(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_long(memory);
}
if (internalevent.lat_is_present) {
shift_current_lat_rad(memory);
}
if (internalevent.lon_is_present) {
shift_current_long_rad(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_lat_rad(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_long_rad(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.reset_waypoints_is_present)) || ((internalevent.lat_is_present && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.skip_waypoint_is_present))) {
shift_mean_latitude(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.reset_waypoints_is_present)) || ((internalevent.lat_is_present && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.skip_waypoint_is_present))) {
shift_distance_to_waypoint_lat(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lon_is_present && internalevent.reset_waypoints_is_present)) || ((internalevent.lon_is_present && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lon_is_present && internalevent.skip_waypoint_is_present))) {
shift_distance_to_waypoint_long(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_distance_to_waypoint(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_waypoint_r(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_waypoint_phi(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_waypoint_direction(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_waypoint_reached(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_finished(memory);
}
if ((((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present)) || ((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.skip_waypoint_is_present))) {
shift_estimated_time_until_waypoint(memory);
}
if (internalevent.velocity_is_present) {
eval_velocity_kmh_0(memory);
}
if (internalevent.lat_is_present) {
eval_current_lat_0(memory);
}
if (internalevent.lon_is_present) {
eval_current_long_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_current_waypoint_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_lat_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_long_0(memory);
}
if (internalevent.lat_is_present) {
eval_current_lat_rad_0(memory);
}
if (internalevent.lon_is_present) {
eval_current_long_rad_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_lat_rad_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_long_rad_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.reset_waypoints_is_present)) || ((internalevent.lat_is_present && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.skip_waypoint_is_present))) {
eval_mean_latitude_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.reset_waypoints_is_present)) || ((internalevent.lat_is_present && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lat_is_present && internalevent.skip_waypoint_is_present))) {
eval_distance_to_waypoint_lat_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lon_is_present && internalevent.reset_waypoints_is_present)) || ((internalevent.lon_is_present && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (internalevent.lon_is_present && internalevent.skip_waypoint_is_present))) {
eval_distance_to_waypoint_long_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_distance_to_waypoint_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_r_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_phi_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_reached_0(memory);
}
if ((((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present)) || ((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.skip_waypoint_is_present))) {
eval_estimated_time_until_waypoint_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_index_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_direction_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_finished_0(memory);
}
Verdict verdict = build_verdict(memory);
clear_activation(memory);
return verdict;
}