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
double heading_sync(Memory* memory){
return memory->boundedbuffer_heading.values[memory->boundedbuffer_heading.current];
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
TUPLE2_DOUBLE__DOUBLE current_waypoint_sync(Memory* memory){
return memory->boundedbuffer_current_waypoint.values[memory->boundedbuffer_current_waypoint.current];
}
double waypoint_lat_sync(Memory* memory){
return memory->boundedbuffer_waypoint_lat.values[memory->boundedbuffer_waypoint_lat.current];
}
double waypoint_long_sync(Memory* memory){
return memory->boundedbuffer_waypoint_long.values[memory->boundedbuffer_waypoint_long.current];
}
bool waypoint_reached_sync(Memory* memory){
return memory->boundedbuffer_waypoint_reached.values[memory->boundedbuffer_waypoint_reached.current];
}
uint64_t waypoint_index_sync(Memory* memory){
return memory->boundedbuffer_waypoint_index.values[memory->boundedbuffer_waypoint_index.current];
}
bool finished_sync(Memory* memory){
return memory->boundedbuffer_finished.values[memory->boundedbuffer_finished.current];
}
double current_heading_sync(Memory* memory){
return memory->boundedbuffer_current_heading.values[memory->boundedbuffer_current_heading.current];
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
bool soft_geofence_sync(Memory* memory){
return memory->boundedbuffer_soft_geofence.values[memory->boundedbuffer_soft_geofence.current];
}
char* trigger_0_sync(Memory* memory){
return memory->boundedbuffer_trigger_0.values[memory->boundedbuffer_trigger_0.current];
}
bool hard_geofence_sync(Memory* memory){
return memory->boundedbuffer_hard_geofence.values[memory->boundedbuffer_hard_geofence.current];
}
char* trigger_1_sync(Memory* memory){
return memory->boundedbuffer_trigger_1.values[memory->boundedbuffer_trigger_1.current];
}
double x_sync(Memory* memory){
return memory->boundedbuffer_x.values[memory->boundedbuffer_x.current];
}
double y_sync(Memory* memory){
return memory->boundedbuffer_y.values[memory->boundedbuffer_y.current];
}
double delta_x_sync(Memory* memory){
return memory->boundedbuffer_delta_x.values[memory->boundedbuffer_delta_x.current];
}
double delta_y_sync(Memory* memory){
return memory->boundedbuffer_delta_y.values[memory->boundedbuffer_delta_y.current];
}
double gradient_sync(Memory* memory){
return memory->boundedbuffer_gradient.values[memory->boundedbuffer_gradient.current];
}
double y_intercept_sync(Memory* memory){
return memory->boundedbuffer_y_intercept.values[memory->boundedbuffer_y_intercept.current];
}
double left_y_sync(Memory* memory){
return memory->boundedbuffer_left_y.values[memory->boundedbuffer_left_y.current];
}
double right_y_sync(Memory* memory){
return memory->boundedbuffer_right_y.values[memory->boundedbuffer_right_y.current];
}
double bottom_x_sync(Memory* memory){
return memory->boundedbuffer_bottom_x.values[memory->boundedbuffer_bottom_x.current];
}
double top_x_sync(Memory* memory){
return memory->boundedbuffer_top_x.values[memory->boundedbuffer_top_x.current];
}
double top_dx_sync(Memory* memory){
return memory->boundedbuffer_top_dx.values[memory->boundedbuffer_top_dx.current];
}
double top_dy_sync(Memory* memory){
return memory->boundedbuffer_top_dy.values[memory->boundedbuffer_top_dy.current];
}
double top_distance_sync(Memory* memory){
return memory->boundedbuffer_top_distance.values[memory->boundedbuffer_top_distance.current];
}
double bottom_dx_sync(Memory* memory){
return memory->boundedbuffer_bottom_dx.values[memory->boundedbuffer_bottom_dx.current];
}
double bottom_dy_sync(Memory* memory){
return memory->boundedbuffer_bottom_dy.values[memory->boundedbuffer_bottom_dy.current];
}
double bottom_distance_sync(Memory* memory){
return memory->boundedbuffer_bottom_distance.values[memory->boundedbuffer_bottom_distance.current];
}
double left_dx_sync(Memory* memory){
return memory->boundedbuffer_left_dx.values[memory->boundedbuffer_left_dx.current];
}
double left_dy_sync(Memory* memory){
return memory->boundedbuffer_left_dy.values[memory->boundedbuffer_left_dy.current];
}
double left_distance_sync(Memory* memory){
return memory->boundedbuffer_left_distance.values[memory->boundedbuffer_left_distance.current];
}
double right_dx_sync(Memory* memory){
return memory->boundedbuffer_right_dx.values[memory->boundedbuffer_right_dx.current];
}
double right_dy_sync(Memory* memory){
return memory->boundedbuffer_right_dy.values[memory->boundedbuffer_right_dy.current];
}
double right_distance_sync(Memory* memory){
return memory->boundedbuffer_right_distance.values[memory->boundedbuffer_right_distance.current];
}
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE left_right_violation_sync(Memory* memory){
return memory->boundedbuffer_left_right_violation.values[memory->boundedbuffer_left_right_violation.current];
}
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE top_bottom_violation_sync(Memory* memory){
return memory->boundedbuffer_top_bottom_violation.values[memory->boundedbuffer_top_bottom_violation.current];
}
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE nearest_geofence_violation_sync(Memory* memory){
return memory->boundedbuffer_nearest_geofence_violation.values[memory->boundedbuffer_nearest_geofence_violation.current];
}
double distance_to_geofence_sync(Memory* memory){
return memory->boundedbuffer_distance_to_geofence.values[memory->boundedbuffer_distance_to_geofence.current];
}
TUPLE2_DOUBLE__DOUBLE geofence_violation_point_sync(Memory* memory){
return memory->boundedbuffer_geofence_violation_point.values[memory->boundedbuffer_geofence_violation_point.current];
}
double estimated_time_until_geofence_sync(Memory* memory){
return memory->boundedbuffer_estimated_time_until_geofence.values[memory->boundedbuffer_estimated_time_until_geofence.current];
}
double soft_left_y_sync(Memory* memory){
return memory->boundedbuffer_soft_left_y.values[memory->boundedbuffer_soft_left_y.current];
}
double soft_right_y_sync(Memory* memory){
return memory->boundedbuffer_soft_right_y.values[memory->boundedbuffer_soft_right_y.current];
}
double soft_bottom_x_sync(Memory* memory){
return memory->boundedbuffer_soft_bottom_x.values[memory->boundedbuffer_soft_bottom_x.current];
}
double soft_top_x_sync(Memory* memory){
return memory->boundedbuffer_soft_top_x.values[memory->boundedbuffer_soft_top_x.current];
}
double soft_top_dx_sync(Memory* memory){
return memory->boundedbuffer_soft_top_dx.values[memory->boundedbuffer_soft_top_dx.current];
}
double soft_top_dy_sync(Memory* memory){
return memory->boundedbuffer_soft_top_dy.values[memory->boundedbuffer_soft_top_dy.current];
}
double soft_top_distance_sync(Memory* memory){
return memory->boundedbuffer_soft_top_distance.values[memory->boundedbuffer_soft_top_distance.current];
}
double soft_bottom_dx_sync(Memory* memory){
return memory->boundedbuffer_soft_bottom_dx.values[memory->boundedbuffer_soft_bottom_dx.current];
}
double soft_bottom_dy_sync(Memory* memory){
return memory->boundedbuffer_soft_bottom_dy.values[memory->boundedbuffer_soft_bottom_dy.current];
}
double soft_bottom_distance_sync(Memory* memory){
return memory->boundedbuffer_soft_bottom_distance.values[memory->boundedbuffer_soft_bottom_distance.current];
}
double soft_left_dx_sync(Memory* memory){
return memory->boundedbuffer_soft_left_dx.values[memory->boundedbuffer_soft_left_dx.current];
}
double soft_left_dy_sync(Memory* memory){
return memory->boundedbuffer_soft_left_dy.values[memory->boundedbuffer_soft_left_dy.current];
}
double soft_left_distance_sync(Memory* memory){
return memory->boundedbuffer_soft_left_distance.values[memory->boundedbuffer_soft_left_distance.current];
}
double soft_right_dx_sync(Memory* memory){
return memory->boundedbuffer_soft_right_dx.values[memory->boundedbuffer_soft_right_dx.current];
}
double soft_right_dy_sync(Memory* memory){
return memory->boundedbuffer_soft_right_dy.values[memory->boundedbuffer_soft_right_dy.current];
}
double soft_right_distance_sync(Memory* memory){
return memory->boundedbuffer_soft_right_distance.values[memory->boundedbuffer_soft_right_distance.current];
}
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE soft_left_right_violation_sync(Memory* memory){
return memory->boundedbuffer_soft_left_right_violation.values[memory->boundedbuffer_soft_left_right_violation.current];
}
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE soft_top_bottom_violation_sync(Memory* memory){
return memory->boundedbuffer_soft_top_bottom_violation.values[memory->boundedbuffer_soft_top_bottom_violation.current];
}
TUPLE2_DOUBLE__DOUBLE soft_geofence_violation_point_swapped_sync(Memory* memory){
return memory->boundedbuffer_soft_geofence_violation_point_swapped.values[memory->boundedbuffer_soft_geofence_violation_point_swapped.current];
}
TUPLE2_DOUBLE__DOUBLE soft_geofence_violation_point_sync(Memory* memory){
return memory->boundedbuffer_soft_geofence_violation_point.values[memory->boundedbuffer_soft_geofence_violation_point.current];
}
uint64_t waypoint_index_offset(Memory* memory, int offset, uint64_t def){
int i = (memory->boundedbuffer_waypoint_index.current - offset + 2) % 2;
if (memory->boundedbuffer_waypoint_index.valid[i])
return memory->boundedbuffer_waypoint_index.values[i];
else
return def;
}
double x_offset(Memory* memory, int offset, double def){
int i = (memory->boundedbuffer_x.current - offset + 2) % 2;
if (memory->boundedbuffer_x.valid[i])
return memory->boundedbuffer_x.values[i];
else
return def;
}
double y_offset(Memory* memory, int offset, double def){
int i = (memory->boundedbuffer_y.current - offset + 2) % 2;
if (memory->boundedbuffer_y.valid[i])
return memory->boundedbuffer_y.values[i];
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
bool current_waypoint_is_fresh(Memory* memory){
return memory->boundedbuffer_current_waypoint.is_fresh;
}
bool waypoint_lat_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_lat.is_fresh;
}
bool waypoint_long_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_long.is_fresh;
}
bool waypoint_reached_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_reached.is_fresh;
}
bool waypoint_index_is_fresh(Memory* memory){
return memory->boundedbuffer_waypoint_index.is_fresh;
}
bool finished_is_fresh(Memory* memory){
return memory->boundedbuffer_finished.is_fresh;
}
bool current_heading_is_fresh(Memory* memory){
return memory->boundedbuffer_current_heading.is_fresh;
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
bool soft_geofence_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_geofence.is_fresh;
}
bool trigger_0_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_0.is_fresh;
}
bool hard_geofence_is_fresh(Memory* memory){
return memory->boundedbuffer_hard_geofence.is_fresh;
}
bool trigger_1_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_1.is_fresh;
}
bool x_is_fresh(Memory* memory){
return memory->boundedbuffer_x.is_fresh;
}
bool y_is_fresh(Memory* memory){
return memory->boundedbuffer_y.is_fresh;
}
bool delta_x_is_fresh(Memory* memory){
return memory->boundedbuffer_delta_x.is_fresh;
}
bool delta_y_is_fresh(Memory* memory){
return memory->boundedbuffer_delta_y.is_fresh;
}
bool gradient_is_fresh(Memory* memory){
return memory->boundedbuffer_gradient.is_fresh;
}
bool y_intercept_is_fresh(Memory* memory){
return memory->boundedbuffer_y_intercept.is_fresh;
}
bool left_y_is_fresh(Memory* memory){
return memory->boundedbuffer_left_y.is_fresh;
}
bool right_y_is_fresh(Memory* memory){
return memory->boundedbuffer_right_y.is_fresh;
}
bool bottom_x_is_fresh(Memory* memory){
return memory->boundedbuffer_bottom_x.is_fresh;
}
bool top_x_is_fresh(Memory* memory){
return memory->boundedbuffer_top_x.is_fresh;
}
bool top_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_top_dx.is_fresh;
}
bool top_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_top_dy.is_fresh;
}
bool top_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_top_distance.is_fresh;
}
bool bottom_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_bottom_dx.is_fresh;
}
bool bottom_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_bottom_dy.is_fresh;
}
bool bottom_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_bottom_distance.is_fresh;
}
bool left_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_left_dx.is_fresh;
}
bool left_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_left_dy.is_fresh;
}
bool left_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_left_distance.is_fresh;
}
bool right_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_right_dx.is_fresh;
}
bool right_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_right_dy.is_fresh;
}
bool right_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_right_distance.is_fresh;
}
bool left_right_violation_is_fresh(Memory* memory){
return memory->boundedbuffer_left_right_violation.is_fresh;
}
bool top_bottom_violation_is_fresh(Memory* memory){
return memory->boundedbuffer_top_bottom_violation.is_fresh;
}
bool nearest_geofence_violation_is_fresh(Memory* memory){
return memory->boundedbuffer_nearest_geofence_violation.is_fresh;
}
bool distance_to_geofence_is_fresh(Memory* memory){
return memory->boundedbuffer_distance_to_geofence.is_fresh;
}
bool geofence_violation_point_is_fresh(Memory* memory){
return memory->boundedbuffer_geofence_violation_point.is_fresh;
}
bool estimated_time_until_geofence_is_fresh(Memory* memory){
return memory->boundedbuffer_estimated_time_until_geofence.is_fresh;
}
bool soft_left_y_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_left_y.is_fresh;
}
bool soft_right_y_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_right_y.is_fresh;
}
bool soft_bottom_x_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_bottom_x.is_fresh;
}
bool soft_top_x_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_top_x.is_fresh;
}
bool soft_top_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_top_dx.is_fresh;
}
bool soft_top_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_top_dy.is_fresh;
}
bool soft_top_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_top_distance.is_fresh;
}
bool soft_bottom_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_bottom_dx.is_fresh;
}
bool soft_bottom_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_bottom_dy.is_fresh;
}
bool soft_bottom_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_bottom_distance.is_fresh;
}
bool soft_left_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_left_dx.is_fresh;
}
bool soft_left_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_left_dy.is_fresh;
}
bool soft_left_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_left_distance.is_fresh;
}
bool soft_right_dx_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_right_dx.is_fresh;
}
bool soft_right_dy_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_right_dy.is_fresh;
}
bool soft_right_distance_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_right_distance.is_fresh;
}
bool soft_left_right_violation_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_left_right_violation.is_fresh;
}
bool soft_top_bottom_violation_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_top_bottom_violation.is_fresh;
}
bool soft_geofence_violation_point_swapped_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_geofence_violation_point_swapped.is_fresh;
}
bool soft_geofence_violation_point_is_fresh(Memory* memory){
return memory->boundedbuffer_soft_geofence_violation_point.is_fresh;
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
void eval_waypoint_reached_0(Memory* memory){
bool new_value = (distance_to_waypoint_sync(memory) < 100);
memory->boundedbuffer_waypoint_reached.values[memory->boundedbuffer_waypoint_reached.current] = new_value;
memory->boundedbuffer_waypoint_reached.valid[memory->boundedbuffer_waypoint_reached.current] = 1;
memory->boundedbuffer_waypoint_reached.is_fresh = 1;
}
void eval_waypoint_index_0(Memory* memory){
uint64_t new_value = (reset_waypoints_get(memory, false)?0:((skip_waypoint_get(memory, false) || waypoint_reached_get(memory, false))?((waypoint_index_offset(memory, 1, 0) + 1) % 10):waypoint_index_offset(memory, 1, 0)));
memory->boundedbuffer_waypoint_index.values[memory->boundedbuffer_waypoint_index.current] = new_value;
memory->boundedbuffer_waypoint_index.valid[memory->boundedbuffer_waypoint_index.current] = 1;
memory->boundedbuffer_waypoint_index.is_fresh = 1;
}
void eval_finished_0(Memory* memory){
bool new_value = (waypoint_reached_sync(memory) && (waypoint_index_sync(memory) == 0));
memory->boundedbuffer_finished.values[memory->boundedbuffer_finished.current] = new_value;
memory->boundedbuffer_finished.valid[memory->boundedbuffer_finished.current] = 1;
memory->boundedbuffer_finished.is_fresh = 1;
}
void eval_current_heading_0(Memory* memory){
double new_value = heading_sync(memory);
memory->boundedbuffer_current_heading.values[memory->boundedbuffer_current_heading.current] = new_value;
memory->boundedbuffer_current_heading.valid[memory->boundedbuffer_current_heading.current] = 1;
memory->boundedbuffer_current_heading.is_fresh = 1;
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
void eval_soft_geofence_0(Memory* memory){
bool new_value = ((((current_lat_sync(memory) < 49.19409) || (current_lat_sync(memory) > 49.27683)) || (current_long_sync(memory) < 7.05102)) || (current_long_sync(memory) > 7.194109999999999));
memory->boundedbuffer_soft_geofence.values[memory->boundedbuffer_soft_geofence.current] = new_value;
memory->boundedbuffer_soft_geofence.valid[memory->boundedbuffer_soft_geofence.current] = 1;
memory->boundedbuffer_soft_geofence.is_fresh = 1;
}
void eval_trigger_0_0(Memory* memory){
char* new_value = STR_CONSTANT_0;
memory->boundedbuffer_trigger_0.values[memory->boundedbuffer_trigger_0.current] = new_value;
memory->boundedbuffer_trigger_0.valid[memory->boundedbuffer_trigger_0.current] = 1;
memory->boundedbuffer_trigger_0.is_fresh = 1;
}
void eval_hard_geofence_0(Memory* memory){
bool new_value = ((((current_lat_sync(memory) < 49.18409) || (current_lat_sync(memory) > 49.28683)) || (current_long_sync(memory) < 7.03602)) || (current_long_sync(memory) > 7.20911));
memory->boundedbuffer_hard_geofence.values[memory->boundedbuffer_hard_geofence.current] = new_value;
memory->boundedbuffer_hard_geofence.valid[memory->boundedbuffer_hard_geofence.current] = 1;
memory->boundedbuffer_hard_geofence.is_fresh = 1;
}
void eval_trigger_1_0(Memory* memory){
char* new_value = STR_CONSTANT_1;
memory->boundedbuffer_trigger_1.values[memory->boundedbuffer_trigger_1.current] = new_value;
memory->boundedbuffer_trigger_1.valid[memory->boundedbuffer_trigger_1.current] = 1;
memory->boundedbuffer_trigger_1.is_fresh = 1;
}
void eval_x_0(Memory* memory){
double new_value = lat_sync(memory);
memory->boundedbuffer_x.values[memory->boundedbuffer_x.current] = new_value;
memory->boundedbuffer_x.valid[memory->boundedbuffer_x.current] = 1;
memory->boundedbuffer_x.is_fresh = 1;
}
void eval_y_0(Memory* memory){
double new_value = lon_sync(memory);
memory->boundedbuffer_y.values[memory->boundedbuffer_y.current] = new_value;
memory->boundedbuffer_y.valid[memory->boundedbuffer_y.current] = 1;
memory->boundedbuffer_y.is_fresh = 1;
}
void eval_delta_x_0(Memory* memory){
double new_value = (x_sync(memory) - x_offset(memory, 1, x_sync(memory)));
memory->boundedbuffer_delta_x.values[memory->boundedbuffer_delta_x.current] = new_value;
memory->boundedbuffer_delta_x.valid[memory->boundedbuffer_delta_x.current] = 1;
memory->boundedbuffer_delta_x.is_fresh = 1;
}
void eval_delta_y_0(Memory* memory){
double new_value = (y_sync(memory) - y_offset(memory, 1, y_sync(memory)));
memory->boundedbuffer_delta_y.values[memory->boundedbuffer_delta_y.current] = new_value;
memory->boundedbuffer_delta_y.valid[memory->boundedbuffer_delta_y.current] = 1;
memory->boundedbuffer_delta_y.is_fresh = 1;
}
void eval_gradient_0(Memory* memory){
double new_value = ((delta_x_sync(memory) == 0)?1000:(delta_y_sync(memory) / delta_x_sync(memory)));
memory->boundedbuffer_gradient.values[memory->boundedbuffer_gradient.current] = new_value;
memory->boundedbuffer_gradient.valid[memory->boundedbuffer_gradient.current] = 1;
memory->boundedbuffer_gradient.is_fresh = 1;
}
void eval_y_intercept_0(Memory* memory){
double new_value = (y_sync(memory) - (gradient_sync(memory) * x_sync(memory)));
memory->boundedbuffer_y_intercept.values[memory->boundedbuffer_y_intercept.current] = new_value;
memory->boundedbuffer_y_intercept.valid[memory->boundedbuffer_y_intercept.current] = 1;
memory->boundedbuffer_y_intercept.is_fresh = 1;
}
void eval_left_y_0(Memory* memory){
double new_value = (y_intercept_sync(memory) + (49.28683 * gradient_sync(memory)));
memory->boundedbuffer_left_y.values[memory->boundedbuffer_left_y.current] = new_value;
memory->boundedbuffer_left_y.valid[memory->boundedbuffer_left_y.current] = 1;
memory->boundedbuffer_left_y.is_fresh = 1;
}
void eval_right_y_0(Memory* memory){
double new_value = (y_intercept_sync(memory) + (49.18409 * gradient_sync(memory)));
memory->boundedbuffer_right_y.values[memory->boundedbuffer_right_y.current] = new_value;
memory->boundedbuffer_right_y.valid[memory->boundedbuffer_right_y.current] = 1;
memory->boundedbuffer_right_y.is_fresh = 1;
}
void eval_bottom_x_0(Memory* memory){
double new_value = ((7.03602 - y_intercept_sync(memory)) / gradient_sync(memory));
memory->boundedbuffer_bottom_x.values[memory->boundedbuffer_bottom_x.current] = new_value;
memory->boundedbuffer_bottom_x.valid[memory->boundedbuffer_bottom_x.current] = 1;
memory->boundedbuffer_bottom_x.is_fresh = 1;
}
void eval_top_x_0(Memory* memory){
double new_value = ((7.20911 - y_intercept_sync(memory)) / gradient_sync(memory));
memory->boundedbuffer_top_x.values[memory->boundedbuffer_top_x.current] = new_value;
memory->boundedbuffer_top_x.valid[memory->boundedbuffer_top_x.current] = 1;
memory->boundedbuffer_top_x.is_fresh = 1;
}
void eval_top_dx_0(Memory* memory){
double new_value = (((top_x_sync(memory) - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_top_dx.values[memory->boundedbuffer_top_dx.current] = new_value;
memory->boundedbuffer_top_dx.valid[memory->boundedbuffer_top_dx.current] = 1;
memory->boundedbuffer_top_dx.is_fresh = 1;
}
void eval_top_dy_0(Memory* memory){
double new_value = (((7.20911 - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_top_dy.values[memory->boundedbuffer_top_dy.current] = new_value;
memory->boundedbuffer_top_dy.valid[memory->boundedbuffer_top_dy.current] = 1;
memory->boundedbuffer_top_dy.is_fresh = 1;
}
void eval_top_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(top_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * top_dy_sync(memory)), 2))));
memory->boundedbuffer_top_distance.values[memory->boundedbuffer_top_distance.current] = new_value;
memory->boundedbuffer_top_distance.valid[memory->boundedbuffer_top_distance.current] = 1;
memory->boundedbuffer_top_distance.is_fresh = 1;
}
void eval_bottom_dx_0(Memory* memory){
double new_value = (((bottom_x_sync(memory) - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_bottom_dx.values[memory->boundedbuffer_bottom_dx.current] = new_value;
memory->boundedbuffer_bottom_dx.valid[memory->boundedbuffer_bottom_dx.current] = 1;
memory->boundedbuffer_bottom_dx.is_fresh = 1;
}
void eval_bottom_dy_0(Memory* memory){
double new_value = (((7.03602 - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_bottom_dy.values[memory->boundedbuffer_bottom_dy.current] = new_value;
memory->boundedbuffer_bottom_dy.valid[memory->boundedbuffer_bottom_dy.current] = 1;
memory->boundedbuffer_bottom_dy.is_fresh = 1;
}
void eval_bottom_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(bottom_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * bottom_dy_sync(memory)), 2))));
memory->boundedbuffer_bottom_distance.values[memory->boundedbuffer_bottom_distance.current] = new_value;
memory->boundedbuffer_bottom_distance.valid[memory->boundedbuffer_bottom_distance.current] = 1;
memory->boundedbuffer_bottom_distance.is_fresh = 1;
}
void eval_left_dx_0(Memory* memory){
double new_value = (((49.28683 - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_left_dx.values[memory->boundedbuffer_left_dx.current] = new_value;
memory->boundedbuffer_left_dx.valid[memory->boundedbuffer_left_dx.current] = 1;
memory->boundedbuffer_left_dx.is_fresh = 1;
}
void eval_left_dy_0(Memory* memory){
double new_value = (((left_y_sync(memory) - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_left_dy.values[memory->boundedbuffer_left_dy.current] = new_value;
memory->boundedbuffer_left_dy.valid[memory->boundedbuffer_left_dy.current] = 1;
memory->boundedbuffer_left_dy.is_fresh = 1;
}
void eval_left_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(left_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * left_dy_sync(memory)), 2))));
memory->boundedbuffer_left_distance.values[memory->boundedbuffer_left_distance.current] = new_value;
memory->boundedbuffer_left_distance.valid[memory->boundedbuffer_left_distance.current] = 1;
memory->boundedbuffer_left_distance.is_fresh = 1;
}
void eval_right_dx_0(Memory* memory){
double new_value = (((49.18409 - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_right_dx.values[memory->boundedbuffer_right_dx.current] = new_value;
memory->boundedbuffer_right_dx.valid[memory->boundedbuffer_right_dx.current] = 1;
memory->boundedbuffer_right_dx.is_fresh = 1;
}
void eval_right_dy_0(Memory* memory){
double new_value = (((right_y_sync(memory) - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_right_dy.values[memory->boundedbuffer_right_dy.current] = new_value;
memory->boundedbuffer_right_dy.valid[memory->boundedbuffer_right_dy.current] = 1;
memory->boundedbuffer_right_dy.is_fresh = 1;
}
void eval_right_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(right_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * right_dy_sync(memory)), 2))));
memory->boundedbuffer_right_distance.values[memory->boundedbuffer_right_distance.current] = new_value;
memory->boundedbuffer_right_distance.valid[memory->boundedbuffer_right_distance.current] = 1;
memory->boundedbuffer_right_distance.is_fresh = 1;
}
void eval_left_right_violation_0(Memory* memory){
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE new_value = (((delta_y_sync(memory) * left_dy_sync(memory)) < 0)?(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=right_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=right_y_sync(memory), ._1=49.18409}}:(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=left_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=left_y_sync(memory), ._1=49.28683}});
memory->boundedbuffer_left_right_violation.values[memory->boundedbuffer_left_right_violation.current] = new_value;
memory->boundedbuffer_left_right_violation.valid[memory->boundedbuffer_left_right_violation.current] = 1;
memory->boundedbuffer_left_right_violation.is_fresh = 1;
}
void eval_top_bottom_violation_0(Memory* memory){
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE new_value = (((delta_x_sync(memory) * top_dx_sync(memory)) >= 0)?(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=top_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=7.20911, ._1=top_x_sync(memory)}}:(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=bottom_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=7.03602, ._1=bottom_x_sync(memory)}});
memory->boundedbuffer_top_bottom_violation.values[memory->boundedbuffer_top_bottom_violation.current] = new_value;
memory->boundedbuffer_top_bottom_violation.valid[memory->boundedbuffer_top_bottom_violation.current] = 1;
memory->boundedbuffer_top_bottom_violation.is_fresh = 1;
}
void eval_nearest_geofence_violation_0(Memory* memory){
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE new_value = (((left_right_violation_sync(memory))._0 < (top_bottom_violation_sync(memory))._0)?left_right_violation_sync(memory):top_bottom_violation_sync(memory));
memory->boundedbuffer_nearest_geofence_violation.values[memory->boundedbuffer_nearest_geofence_violation.current] = new_value;
memory->boundedbuffer_nearest_geofence_violation.valid[memory->boundedbuffer_nearest_geofence_violation.current] = 1;
memory->boundedbuffer_nearest_geofence_violation.is_fresh = 1;
}
void eval_distance_to_geofence_0(Memory* memory){
double new_value = (nearest_geofence_violation_sync(memory))._0;
memory->boundedbuffer_distance_to_geofence.values[memory->boundedbuffer_distance_to_geofence.current] = new_value;
memory->boundedbuffer_distance_to_geofence.valid[memory->boundedbuffer_distance_to_geofence.current] = 1;
memory->boundedbuffer_distance_to_geofence.is_fresh = 1;
}
void eval_geofence_violation_point_0(Memory* memory){
TUPLE2_DOUBLE__DOUBLE new_value = (TUPLE2_DOUBLE__DOUBLE){._0=((nearest_geofence_violation_sync(memory))._1)._1, ._1=((nearest_geofence_violation_sync(memory))._1)._0};
memory->boundedbuffer_geofence_violation_point.values[memory->boundedbuffer_geofence_violation_point.current] = new_value;
memory->boundedbuffer_geofence_violation_point.valid[memory->boundedbuffer_geofence_violation_point.current] = 1;
memory->boundedbuffer_geofence_violation_point.is_fresh = 1;
}
void eval_estimated_time_until_geofence_0(Memory* memory){
double new_value = ((velocity_kmh_sync(memory) == 0)?5000:(distance_to_geofence_sync(memory) / (velocity_kmh_sync(memory) / 3.6)));
memory->boundedbuffer_estimated_time_until_geofence.values[memory->boundedbuffer_estimated_time_until_geofence.current] = new_value;
memory->boundedbuffer_estimated_time_until_geofence.valid[memory->boundedbuffer_estimated_time_until_geofence.current] = 1;
memory->boundedbuffer_estimated_time_until_geofence.is_fresh = 1;
}
void eval_soft_left_y_0(Memory* memory){
double new_value = (y_intercept_sync(memory) + (49.27683 * gradient_sync(memory)));
memory->boundedbuffer_soft_left_y.values[memory->boundedbuffer_soft_left_y.current] = new_value;
memory->boundedbuffer_soft_left_y.valid[memory->boundedbuffer_soft_left_y.current] = 1;
memory->boundedbuffer_soft_left_y.is_fresh = 1;
}
void eval_soft_right_y_0(Memory* memory){
double new_value = (y_intercept_sync(memory) + (49.19409 * gradient_sync(memory)));
memory->boundedbuffer_soft_right_y.values[memory->boundedbuffer_soft_right_y.current] = new_value;
memory->boundedbuffer_soft_right_y.valid[memory->boundedbuffer_soft_right_y.current] = 1;
memory->boundedbuffer_soft_right_y.is_fresh = 1;
}
void eval_soft_bottom_x_0(Memory* memory){
double new_value = ((7.05102 - y_intercept_sync(memory)) / gradient_sync(memory));
memory->boundedbuffer_soft_bottom_x.values[memory->boundedbuffer_soft_bottom_x.current] = new_value;
memory->boundedbuffer_soft_bottom_x.valid[memory->boundedbuffer_soft_bottom_x.current] = 1;
memory->boundedbuffer_soft_bottom_x.is_fresh = 1;
}
void eval_soft_top_x_0(Memory* memory){
double new_value = ((7.194109999999999 - y_intercept_sync(memory)) / gradient_sync(memory));
memory->boundedbuffer_soft_top_x.values[memory->boundedbuffer_soft_top_x.current] = new_value;
memory->boundedbuffer_soft_top_x.valid[memory->boundedbuffer_soft_top_x.current] = 1;
memory->boundedbuffer_soft_top_x.is_fresh = 1;
}
void eval_soft_top_dx_0(Memory* memory){
double new_value = (((soft_top_x_sync(memory) - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_top_dx.values[memory->boundedbuffer_soft_top_dx.current] = new_value;
memory->boundedbuffer_soft_top_dx.valid[memory->boundedbuffer_soft_top_dx.current] = 1;
memory->boundedbuffer_soft_top_dx.is_fresh = 1;
}
void eval_soft_top_dy_0(Memory* memory){
double new_value = (((7.194109999999999 - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_top_dy.values[memory->boundedbuffer_soft_top_dy.current] = new_value;
memory->boundedbuffer_soft_top_dy.valid[memory->boundedbuffer_soft_top_dy.current] = 1;
memory->boundedbuffer_soft_top_dy.is_fresh = 1;
}
void eval_soft_top_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(top_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * soft_top_dy_sync(memory)), 2))));
memory->boundedbuffer_soft_top_distance.values[memory->boundedbuffer_soft_top_distance.current] = new_value;
memory->boundedbuffer_soft_top_distance.valid[memory->boundedbuffer_soft_top_distance.current] = 1;
memory->boundedbuffer_soft_top_distance.is_fresh = 1;
}
void eval_soft_bottom_dx_0(Memory* memory){
double new_value = (((soft_bottom_x_sync(memory) - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_bottom_dx.values[memory->boundedbuffer_soft_bottom_dx.current] = new_value;
memory->boundedbuffer_soft_bottom_dx.valid[memory->boundedbuffer_soft_bottom_dx.current] = 1;
memory->boundedbuffer_soft_bottom_dx.is_fresh = 1;
}
void eval_soft_bottom_dy_0(Memory* memory){
double new_value = (((7.05102 - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_bottom_dy.values[memory->boundedbuffer_soft_bottom_dy.current] = new_value;
memory->boundedbuffer_soft_bottom_dy.valid[memory->boundedbuffer_soft_bottom_dy.current] = 1;
memory->boundedbuffer_soft_bottom_dy.is_fresh = 1;
}
void eval_soft_bottom_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(bottom_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * soft_bottom_dy_sync(memory)), 2))));
memory->boundedbuffer_soft_bottom_distance.values[memory->boundedbuffer_soft_bottom_distance.current] = new_value;
memory->boundedbuffer_soft_bottom_distance.valid[memory->boundedbuffer_soft_bottom_distance.current] = 1;
memory->boundedbuffer_soft_bottom_distance.is_fresh = 1;
}
void eval_soft_left_dx_0(Memory* memory){
double new_value = (((49.27683 - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_left_dx.values[memory->boundedbuffer_soft_left_dx.current] = new_value;
memory->boundedbuffer_soft_left_dx.valid[memory->boundedbuffer_soft_left_dx.current] = 1;
memory->boundedbuffer_soft_left_dx.is_fresh = 1;
}
void eval_soft_left_dy_0(Memory* memory){
double new_value = (((soft_left_y_sync(memory) - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_left_dy.values[memory->boundedbuffer_soft_left_dy.current] = new_value;
memory->boundedbuffer_soft_left_dy.valid[memory->boundedbuffer_soft_left_dy.current] = 1;
memory->boundedbuffer_soft_left_dy.is_fresh = 1;
}
void eval_soft_left_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(left_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * soft_left_dy_sync(memory)), 2))));
memory->boundedbuffer_soft_left_distance.values[memory->boundedbuffer_soft_left_distance.current] = new_value;
memory->boundedbuffer_soft_left_distance.valid[memory->boundedbuffer_soft_left_distance.current] = 1;
memory->boundedbuffer_soft_left_distance.is_fresh = 1;
}
void eval_soft_right_dx_0(Memory* memory){
double new_value = (((49.19409 - x_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_right_dx.values[memory->boundedbuffer_soft_right_dx.current] = new_value;
memory->boundedbuffer_soft_right_dx.valid[memory->boundedbuffer_soft_right_dx.current] = 1;
memory->boundedbuffer_soft_right_dx.is_fresh = 1;
}
void eval_soft_right_dy_0(Memory* memory){
double new_value = (((soft_right_y_sync(memory) - y_sync(memory)) * 3.14519) / 180);
memory->boundedbuffer_soft_right_dy.values[memory->boundedbuffer_soft_right_dy.current] = new_value;
memory->boundedbuffer_soft_right_dy.valid[memory->boundedbuffer_soft_right_dy.current] = 1;
memory->boundedbuffer_soft_right_dy.is_fresh = 1;
}
void eval_soft_right_distance_0(Memory* memory){
double new_value = (6371009 * sqrt((pow(right_dx_sync(memory), 2) + pow((cos(mean_latitude_sync(memory)) * soft_right_dy_sync(memory)), 2))));
memory->boundedbuffer_soft_right_distance.values[memory->boundedbuffer_soft_right_distance.current] = new_value;
memory->boundedbuffer_soft_right_distance.valid[memory->boundedbuffer_soft_right_distance.current] = 1;
memory->boundedbuffer_soft_right_distance.is_fresh = 1;
}
void eval_soft_left_right_violation_0(Memory* memory){
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE new_value = (((delta_y_sync(memory) * soft_left_dy_sync(memory)) < 0)?(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=soft_right_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=soft_right_y_sync(memory), ._1=49.19409}}:(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=soft_left_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=soft_left_y_sync(memory), ._1=49.27683}});
memory->boundedbuffer_soft_left_right_violation.values[memory->boundedbuffer_soft_left_right_violation.current] = new_value;
memory->boundedbuffer_soft_left_right_violation.valid[memory->boundedbuffer_soft_left_right_violation.current] = 1;
memory->boundedbuffer_soft_left_right_violation.is_fresh = 1;
}
void eval_soft_top_bottom_violation_0(Memory* memory){
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE new_value = (((delta_x_sync(memory) * soft_top_dx_sync(memory)) >= 0)?(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=soft_top_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=7.194109999999999, ._1=soft_top_x_sync(memory)}}:(TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE){._0=soft_bottom_distance_sync(memory), ._1=(TUPLE2_DOUBLE__DOUBLE){._0=7.05102, ._1=soft_bottom_x_sync(memory)}});
memory->boundedbuffer_soft_top_bottom_violation.values[memory->boundedbuffer_soft_top_bottom_violation.current] = new_value;
memory->boundedbuffer_soft_top_bottom_violation.valid[memory->boundedbuffer_soft_top_bottom_violation.current] = 1;
memory->boundedbuffer_soft_top_bottom_violation.is_fresh = 1;
}
void eval_soft_geofence_violation_point_swapped_0(Memory* memory){
TUPLE2_DOUBLE__DOUBLE new_value = (((soft_left_right_violation_sync(memory))._0 < (soft_top_bottom_violation_sync(memory))._0)?(soft_left_right_violation_sync(memory))._1:(soft_top_bottom_violation_sync(memory))._1);
memory->boundedbuffer_soft_geofence_violation_point_swapped.values[memory->boundedbuffer_soft_geofence_violation_point_swapped.current] = new_value;
memory->boundedbuffer_soft_geofence_violation_point_swapped.valid[memory->boundedbuffer_soft_geofence_violation_point_swapped.current] = 1;
memory->boundedbuffer_soft_geofence_violation_point_swapped.is_fresh = 1;
}
void eval_soft_geofence_violation_point_0(Memory* memory){
TUPLE2_DOUBLE__DOUBLE new_value = (TUPLE2_DOUBLE__DOUBLE){._0=(soft_geofence_violation_point_swapped_sync(memory))._1, ._1=(soft_geofence_violation_point_swapped_sync(memory))._0};
memory->boundedbuffer_soft_geofence_violation_point.values[memory->boundedbuffer_soft_geofence_violation_point.current] = new_value;
memory->boundedbuffer_soft_geofence_violation_point.valid[memory->boundedbuffer_soft_geofence_violation_point.current] = 1;
memory->boundedbuffer_soft_geofence_violation_point.is_fresh = 1;
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
void shift_current_waypoint(Memory* memory){
memory->boundedbuffer_current_waypoint.current = (memory->boundedbuffer_current_waypoint.current + 1) % 1;
}
void shift_waypoint_lat(Memory* memory){
memory->boundedbuffer_waypoint_lat.current = (memory->boundedbuffer_waypoint_lat.current + 1) % 1;
}
void shift_waypoint_long(Memory* memory){
memory->boundedbuffer_waypoint_long.current = (memory->boundedbuffer_waypoint_long.current + 1) % 1;
}
void shift_waypoint_reached(Memory* memory){
memory->boundedbuffer_waypoint_reached.current = (memory->boundedbuffer_waypoint_reached.current + 1) % 1;
}
void shift_waypoint_index(Memory* memory){
memory->boundedbuffer_waypoint_index.current = (memory->boundedbuffer_waypoint_index.current + 1) % 2;
}
void shift_finished(Memory* memory){
memory->boundedbuffer_finished.current = (memory->boundedbuffer_finished.current + 1) % 1;
}
void shift_current_heading(Memory* memory){
memory->boundedbuffer_current_heading.current = (memory->boundedbuffer_current_heading.current + 1) % 1;
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
void shift_soft_geofence(Memory* memory){
memory->boundedbuffer_soft_geofence.current = (memory->boundedbuffer_soft_geofence.current + 1) % 1;
}
void shift_trigger_0(Memory* memory){
memory->boundedbuffer_trigger_0.current = (memory->boundedbuffer_trigger_0.current + 1) % 1;
}
void shift_hard_geofence(Memory* memory){
memory->boundedbuffer_hard_geofence.current = (memory->boundedbuffer_hard_geofence.current + 1) % 1;
}
void shift_trigger_1(Memory* memory){
memory->boundedbuffer_trigger_1.current = (memory->boundedbuffer_trigger_1.current + 1) % 1;
}
void shift_x(Memory* memory){
memory->boundedbuffer_x.current = (memory->boundedbuffer_x.current + 1) % 2;
}
void shift_y(Memory* memory){
memory->boundedbuffer_y.current = (memory->boundedbuffer_y.current + 1) % 2;
}
void shift_delta_x(Memory* memory){
memory->boundedbuffer_delta_x.current = (memory->boundedbuffer_delta_x.current + 1) % 1;
}
void shift_delta_y(Memory* memory){
memory->boundedbuffer_delta_y.current = (memory->boundedbuffer_delta_y.current + 1) % 1;
}
void shift_gradient(Memory* memory){
memory->boundedbuffer_gradient.current = (memory->boundedbuffer_gradient.current + 1) % 1;
}
void shift_y_intercept(Memory* memory){
memory->boundedbuffer_y_intercept.current = (memory->boundedbuffer_y_intercept.current + 1) % 1;
}
void shift_left_y(Memory* memory){
memory->boundedbuffer_left_y.current = (memory->boundedbuffer_left_y.current + 1) % 1;
}
void shift_right_y(Memory* memory){
memory->boundedbuffer_right_y.current = (memory->boundedbuffer_right_y.current + 1) % 1;
}
void shift_bottom_x(Memory* memory){
memory->boundedbuffer_bottom_x.current = (memory->boundedbuffer_bottom_x.current + 1) % 1;
}
void shift_top_x(Memory* memory){
memory->boundedbuffer_top_x.current = (memory->boundedbuffer_top_x.current + 1) % 1;
}
void shift_top_dx(Memory* memory){
memory->boundedbuffer_top_dx.current = (memory->boundedbuffer_top_dx.current + 1) % 1;
}
void shift_top_dy(Memory* memory){
memory->boundedbuffer_top_dy.current = (memory->boundedbuffer_top_dy.current + 1) % 1;
}
void shift_top_distance(Memory* memory){
memory->boundedbuffer_top_distance.current = (memory->boundedbuffer_top_distance.current + 1) % 1;
}
void shift_bottom_dx(Memory* memory){
memory->boundedbuffer_bottom_dx.current = (memory->boundedbuffer_bottom_dx.current + 1) % 1;
}
void shift_bottom_dy(Memory* memory){
memory->boundedbuffer_bottom_dy.current = (memory->boundedbuffer_bottom_dy.current + 1) % 1;
}
void shift_bottom_distance(Memory* memory){
memory->boundedbuffer_bottom_distance.current = (memory->boundedbuffer_bottom_distance.current + 1) % 1;
}
void shift_left_dx(Memory* memory){
memory->boundedbuffer_left_dx.current = (memory->boundedbuffer_left_dx.current + 1) % 1;
}
void shift_left_dy(Memory* memory){
memory->boundedbuffer_left_dy.current = (memory->boundedbuffer_left_dy.current + 1) % 1;
}
void shift_left_distance(Memory* memory){
memory->boundedbuffer_left_distance.current = (memory->boundedbuffer_left_distance.current + 1) % 1;
}
void shift_right_dx(Memory* memory){
memory->boundedbuffer_right_dx.current = (memory->boundedbuffer_right_dx.current + 1) % 1;
}
void shift_right_dy(Memory* memory){
memory->boundedbuffer_right_dy.current = (memory->boundedbuffer_right_dy.current + 1) % 1;
}
void shift_right_distance(Memory* memory){
memory->boundedbuffer_right_distance.current = (memory->boundedbuffer_right_distance.current + 1) % 1;
}
void shift_left_right_violation(Memory* memory){
memory->boundedbuffer_left_right_violation.current = (memory->boundedbuffer_left_right_violation.current + 1) % 1;
}
void shift_top_bottom_violation(Memory* memory){
memory->boundedbuffer_top_bottom_violation.current = (memory->boundedbuffer_top_bottom_violation.current + 1) % 1;
}
void shift_nearest_geofence_violation(Memory* memory){
memory->boundedbuffer_nearest_geofence_violation.current = (memory->boundedbuffer_nearest_geofence_violation.current + 1) % 1;
}
void shift_distance_to_geofence(Memory* memory){
memory->boundedbuffer_distance_to_geofence.current = (memory->boundedbuffer_distance_to_geofence.current + 1) % 1;
}
void shift_geofence_violation_point(Memory* memory){
memory->boundedbuffer_geofence_violation_point.current = (memory->boundedbuffer_geofence_violation_point.current + 1) % 1;
}
void shift_estimated_time_until_geofence(Memory* memory){
memory->boundedbuffer_estimated_time_until_geofence.current = (memory->boundedbuffer_estimated_time_until_geofence.current + 1) % 1;
}
void shift_soft_left_y(Memory* memory){
memory->boundedbuffer_soft_left_y.current = (memory->boundedbuffer_soft_left_y.current + 1) % 1;
}
void shift_soft_right_y(Memory* memory){
memory->boundedbuffer_soft_right_y.current = (memory->boundedbuffer_soft_right_y.current + 1) % 1;
}
void shift_soft_bottom_x(Memory* memory){
memory->boundedbuffer_soft_bottom_x.current = (memory->boundedbuffer_soft_bottom_x.current + 1) % 1;
}
void shift_soft_top_x(Memory* memory){
memory->boundedbuffer_soft_top_x.current = (memory->boundedbuffer_soft_top_x.current + 1) % 1;
}
void shift_soft_top_dx(Memory* memory){
memory->boundedbuffer_soft_top_dx.current = (memory->boundedbuffer_soft_top_dx.current + 1) % 1;
}
void shift_soft_top_dy(Memory* memory){
memory->boundedbuffer_soft_top_dy.current = (memory->boundedbuffer_soft_top_dy.current + 1) % 1;
}
void shift_soft_top_distance(Memory* memory){
memory->boundedbuffer_soft_top_distance.current = (memory->boundedbuffer_soft_top_distance.current + 1) % 1;
}
void shift_soft_bottom_dx(Memory* memory){
memory->boundedbuffer_soft_bottom_dx.current = (memory->boundedbuffer_soft_bottom_dx.current + 1) % 1;
}
void shift_soft_bottom_dy(Memory* memory){
memory->boundedbuffer_soft_bottom_dy.current = (memory->boundedbuffer_soft_bottom_dy.current + 1) % 1;
}
void shift_soft_bottom_distance(Memory* memory){
memory->boundedbuffer_soft_bottom_distance.current = (memory->boundedbuffer_soft_bottom_distance.current + 1) % 1;
}
void shift_soft_left_dx(Memory* memory){
memory->boundedbuffer_soft_left_dx.current = (memory->boundedbuffer_soft_left_dx.current + 1) % 1;
}
void shift_soft_left_dy(Memory* memory){
memory->boundedbuffer_soft_left_dy.current = (memory->boundedbuffer_soft_left_dy.current + 1) % 1;
}
void shift_soft_left_distance(Memory* memory){
memory->boundedbuffer_soft_left_distance.current = (memory->boundedbuffer_soft_left_distance.current + 1) % 1;
}
void shift_soft_right_dx(Memory* memory){
memory->boundedbuffer_soft_right_dx.current = (memory->boundedbuffer_soft_right_dx.current + 1) % 1;
}
void shift_soft_right_dy(Memory* memory){
memory->boundedbuffer_soft_right_dy.current = (memory->boundedbuffer_soft_right_dy.current + 1) % 1;
}
void shift_soft_right_distance(Memory* memory){
memory->boundedbuffer_soft_right_distance.current = (memory->boundedbuffer_soft_right_distance.current + 1) % 1;
}
void shift_soft_left_right_violation(Memory* memory){
memory->boundedbuffer_soft_left_right_violation.current = (memory->boundedbuffer_soft_left_right_violation.current + 1) % 1;
}
void shift_soft_top_bottom_violation(Memory* memory){
memory->boundedbuffer_soft_top_bottom_violation.current = (memory->boundedbuffer_soft_top_bottom_violation.current + 1) % 1;
}
void shift_soft_geofence_violation_point_swapped(Memory* memory){
memory->boundedbuffer_soft_geofence_violation_point_swapped.current = (memory->boundedbuffer_soft_geofence_violation_point_swapped.current + 1) % 1;
}
void shift_soft_geofence_violation_point(Memory* memory){
memory->boundedbuffer_soft_geofence_violation_point.current = (memory->boundedbuffer_soft_geofence_violation_point.current + 1) % 1;
}
bool expr_0(Memory* memory){
return (soft_geofence_sync(memory) && (!hard_geofence_sync(memory)));
}
bool expr_1(Memory* memory){
return hard_geofence_sync(memory);
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
if (current_waypoint_is_fresh(memory)) {
verdict.current_waypoint = current_waypoint_sync(memory);verdict.current_waypoint_is_present = 1;
}
if (waypoint_lat_is_fresh(memory)) {
verdict.waypoint_lat = waypoint_lat_sync(memory);verdict.waypoint_lat_is_present = 1;
}
if (waypoint_long_is_fresh(memory)) {
verdict.waypoint_long = waypoint_long_sync(memory);verdict.waypoint_long_is_present = 1;
}
if (waypoint_reached_is_fresh(memory)) {
verdict.waypoint_reached = waypoint_reached_sync(memory);verdict.waypoint_reached_is_present = 1;
}
if (waypoint_index_is_fresh(memory)) {
verdict.waypoint_index = waypoint_index_sync(memory);verdict.waypoint_index_is_present = 1;
}
if (finished_is_fresh(memory)) {
verdict.finished = finished_sync(memory);verdict.finished_is_present = 1;
}
if (current_heading_is_fresh(memory)) {
verdict.current_heading = current_heading_sync(memory);verdict.current_heading_is_present = 1;
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
if (soft_geofence_is_fresh(memory)) {
verdict.soft_geofence = soft_geofence_sync(memory);verdict.soft_geofence_is_present = 1;
}
if (trigger_0_is_fresh(memory)) {
verdict.trigger_0 = trigger_0_sync(memory);verdict.trigger_0_is_present = 1;
}
if (hard_geofence_is_fresh(memory)) {
verdict.hard_geofence = hard_geofence_sync(memory);verdict.hard_geofence_is_present = 1;
}
if (trigger_1_is_fresh(memory)) {
verdict.trigger_1 = trigger_1_sync(memory);verdict.trigger_1_is_present = 1;
}
if (x_is_fresh(memory)) {
verdict.x = x_sync(memory);verdict.x_is_present = 1;
}
if (y_is_fresh(memory)) {
verdict.y = y_sync(memory);verdict.y_is_present = 1;
}
if (delta_x_is_fresh(memory)) {
verdict.delta_x = delta_x_sync(memory);verdict.delta_x_is_present = 1;
}
if (delta_y_is_fresh(memory)) {
verdict.delta_y = delta_y_sync(memory);verdict.delta_y_is_present = 1;
}
if (gradient_is_fresh(memory)) {
verdict.gradient = gradient_sync(memory);verdict.gradient_is_present = 1;
}
if (y_intercept_is_fresh(memory)) {
verdict.y_intercept = y_intercept_sync(memory);verdict.y_intercept_is_present = 1;
}
if (left_y_is_fresh(memory)) {
verdict.left_y = left_y_sync(memory);verdict.left_y_is_present = 1;
}
if (right_y_is_fresh(memory)) {
verdict.right_y = right_y_sync(memory);verdict.right_y_is_present = 1;
}
if (bottom_x_is_fresh(memory)) {
verdict.bottom_x = bottom_x_sync(memory);verdict.bottom_x_is_present = 1;
}
if (top_x_is_fresh(memory)) {
verdict.top_x = top_x_sync(memory);verdict.top_x_is_present = 1;
}
if (top_dx_is_fresh(memory)) {
verdict.top_dx = top_dx_sync(memory);verdict.top_dx_is_present = 1;
}
if (top_dy_is_fresh(memory)) {
verdict.top_dy = top_dy_sync(memory);verdict.top_dy_is_present = 1;
}
if (top_distance_is_fresh(memory)) {
verdict.top_distance = top_distance_sync(memory);verdict.top_distance_is_present = 1;
}
if (bottom_dx_is_fresh(memory)) {
verdict.bottom_dx = bottom_dx_sync(memory);verdict.bottom_dx_is_present = 1;
}
if (bottom_dy_is_fresh(memory)) {
verdict.bottom_dy = bottom_dy_sync(memory);verdict.bottom_dy_is_present = 1;
}
if (bottom_distance_is_fresh(memory)) {
verdict.bottom_distance = bottom_distance_sync(memory);verdict.bottom_distance_is_present = 1;
}
if (left_dx_is_fresh(memory)) {
verdict.left_dx = left_dx_sync(memory);verdict.left_dx_is_present = 1;
}
if (left_dy_is_fresh(memory)) {
verdict.left_dy = left_dy_sync(memory);verdict.left_dy_is_present = 1;
}
if (left_distance_is_fresh(memory)) {
verdict.left_distance = left_distance_sync(memory);verdict.left_distance_is_present = 1;
}
if (right_dx_is_fresh(memory)) {
verdict.right_dx = right_dx_sync(memory);verdict.right_dx_is_present = 1;
}
if (right_dy_is_fresh(memory)) {
verdict.right_dy = right_dy_sync(memory);verdict.right_dy_is_present = 1;
}
if (right_distance_is_fresh(memory)) {
verdict.right_distance = right_distance_sync(memory);verdict.right_distance_is_present = 1;
}
if (left_right_violation_is_fresh(memory)) {
verdict.left_right_violation = left_right_violation_sync(memory);verdict.left_right_violation_is_present = 1;
}
if (top_bottom_violation_is_fresh(memory)) {
verdict.top_bottom_violation = top_bottom_violation_sync(memory);verdict.top_bottom_violation_is_present = 1;
}
if (nearest_geofence_violation_is_fresh(memory)) {
verdict.nearest_geofence_violation = nearest_geofence_violation_sync(memory);verdict.nearest_geofence_violation_is_present = 1;
}
if (distance_to_geofence_is_fresh(memory)) {
verdict.distance_to_geofence = distance_to_geofence_sync(memory);verdict.distance_to_geofence_is_present = 1;
}
if (geofence_violation_point_is_fresh(memory)) {
verdict.geofence_violation_point = geofence_violation_point_sync(memory);verdict.geofence_violation_point_is_present = 1;
}
if (estimated_time_until_geofence_is_fresh(memory)) {
verdict.estimated_time_until_geofence = estimated_time_until_geofence_sync(memory);verdict.estimated_time_until_geofence_is_present = 1;
}
if (soft_left_y_is_fresh(memory)) {
verdict.soft_left_y = soft_left_y_sync(memory);verdict.soft_left_y_is_present = 1;
}
if (soft_right_y_is_fresh(memory)) {
verdict.soft_right_y = soft_right_y_sync(memory);verdict.soft_right_y_is_present = 1;
}
if (soft_bottom_x_is_fresh(memory)) {
verdict.soft_bottom_x = soft_bottom_x_sync(memory);verdict.soft_bottom_x_is_present = 1;
}
if (soft_top_x_is_fresh(memory)) {
verdict.soft_top_x = soft_top_x_sync(memory);verdict.soft_top_x_is_present = 1;
}
if (soft_top_dx_is_fresh(memory)) {
verdict.soft_top_dx = soft_top_dx_sync(memory);verdict.soft_top_dx_is_present = 1;
}
if (soft_top_dy_is_fresh(memory)) {
verdict.soft_top_dy = soft_top_dy_sync(memory);verdict.soft_top_dy_is_present = 1;
}
if (soft_top_distance_is_fresh(memory)) {
verdict.soft_top_distance = soft_top_distance_sync(memory);verdict.soft_top_distance_is_present = 1;
}
if (soft_bottom_dx_is_fresh(memory)) {
verdict.soft_bottom_dx = soft_bottom_dx_sync(memory);verdict.soft_bottom_dx_is_present = 1;
}
if (soft_bottom_dy_is_fresh(memory)) {
verdict.soft_bottom_dy = soft_bottom_dy_sync(memory);verdict.soft_bottom_dy_is_present = 1;
}
if (soft_bottom_distance_is_fresh(memory)) {
verdict.soft_bottom_distance = soft_bottom_distance_sync(memory);verdict.soft_bottom_distance_is_present = 1;
}
if (soft_left_dx_is_fresh(memory)) {
verdict.soft_left_dx = soft_left_dx_sync(memory);verdict.soft_left_dx_is_present = 1;
}
if (soft_left_dy_is_fresh(memory)) {
verdict.soft_left_dy = soft_left_dy_sync(memory);verdict.soft_left_dy_is_present = 1;
}
if (soft_left_distance_is_fresh(memory)) {
verdict.soft_left_distance = soft_left_distance_sync(memory);verdict.soft_left_distance_is_present = 1;
}
if (soft_right_dx_is_fresh(memory)) {
verdict.soft_right_dx = soft_right_dx_sync(memory);verdict.soft_right_dx_is_present = 1;
}
if (soft_right_dy_is_fresh(memory)) {
verdict.soft_right_dy = soft_right_dy_sync(memory);verdict.soft_right_dy_is_present = 1;
}
if (soft_right_distance_is_fresh(memory)) {
verdict.soft_right_distance = soft_right_distance_sync(memory);verdict.soft_right_distance_is_present = 1;
}
if (soft_left_right_violation_is_fresh(memory)) {
verdict.soft_left_right_violation = soft_left_right_violation_sync(memory);verdict.soft_left_right_violation_is_present = 1;
}
if (soft_top_bottom_violation_is_fresh(memory)) {
verdict.soft_top_bottom_violation = soft_top_bottom_violation_sync(memory);verdict.soft_top_bottom_violation_is_present = 1;
}
if (soft_geofence_violation_point_swapped_is_fresh(memory)) {
verdict.soft_geofence_violation_point_swapped = soft_geofence_violation_point_swapped_sync(memory);verdict.soft_geofence_violation_point_swapped_is_present = 1;
}
if (soft_geofence_violation_point_is_fresh(memory)) {
verdict.soft_geofence_violation_point = soft_geofence_violation_point_sync(memory);verdict.soft_geofence_violation_point_is_present = 1;
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
memory->boundedbuffer_current_waypoint.is_fresh = 0;
memory->boundedbuffer_waypoint_lat.is_fresh = 0;
memory->boundedbuffer_waypoint_long.is_fresh = 0;
memory->boundedbuffer_waypoint_reached.is_fresh = 0;
memory->boundedbuffer_waypoint_index.is_fresh = 0;
memory->boundedbuffer_finished.is_fresh = 0;
memory->boundedbuffer_current_heading.is_fresh = 0;
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
memory->boundedbuffer_soft_geofence.is_fresh = 0;
memory->boundedbuffer_trigger_0.is_fresh = 0;
memory->boundedbuffer_hard_geofence.is_fresh = 0;
memory->boundedbuffer_trigger_1.is_fresh = 0;
memory->boundedbuffer_x.is_fresh = 0;
memory->boundedbuffer_y.is_fresh = 0;
memory->boundedbuffer_delta_x.is_fresh = 0;
memory->boundedbuffer_delta_y.is_fresh = 0;
memory->boundedbuffer_gradient.is_fresh = 0;
memory->boundedbuffer_y_intercept.is_fresh = 0;
memory->boundedbuffer_left_y.is_fresh = 0;
memory->boundedbuffer_right_y.is_fresh = 0;
memory->boundedbuffer_bottom_x.is_fresh = 0;
memory->boundedbuffer_top_x.is_fresh = 0;
memory->boundedbuffer_top_dx.is_fresh = 0;
memory->boundedbuffer_top_dy.is_fresh = 0;
memory->boundedbuffer_top_distance.is_fresh = 0;
memory->boundedbuffer_bottom_dx.is_fresh = 0;
memory->boundedbuffer_bottom_dy.is_fresh = 0;
memory->boundedbuffer_bottom_distance.is_fresh = 0;
memory->boundedbuffer_left_dx.is_fresh = 0;
memory->boundedbuffer_left_dy.is_fresh = 0;
memory->boundedbuffer_left_distance.is_fresh = 0;
memory->boundedbuffer_right_dx.is_fresh = 0;
memory->boundedbuffer_right_dy.is_fresh = 0;
memory->boundedbuffer_right_distance.is_fresh = 0;
memory->boundedbuffer_left_right_violation.is_fresh = 0;
memory->boundedbuffer_top_bottom_violation.is_fresh = 0;
memory->boundedbuffer_nearest_geofence_violation.is_fresh = 0;
memory->boundedbuffer_distance_to_geofence.is_fresh = 0;
memory->boundedbuffer_geofence_violation_point.is_fresh = 0;
memory->boundedbuffer_estimated_time_until_geofence.is_fresh = 0;
memory->boundedbuffer_soft_left_y.is_fresh = 0;
memory->boundedbuffer_soft_right_y.is_fresh = 0;
memory->boundedbuffer_soft_bottom_x.is_fresh = 0;
memory->boundedbuffer_soft_top_x.is_fresh = 0;
memory->boundedbuffer_soft_top_dx.is_fresh = 0;
memory->boundedbuffer_soft_top_dy.is_fresh = 0;
memory->boundedbuffer_soft_top_distance.is_fresh = 0;
memory->boundedbuffer_soft_bottom_dx.is_fresh = 0;
memory->boundedbuffer_soft_bottom_dy.is_fresh = 0;
memory->boundedbuffer_soft_bottom_distance.is_fresh = 0;
memory->boundedbuffer_soft_left_dx.is_fresh = 0;
memory->boundedbuffer_soft_left_dy.is_fresh = 0;
memory->boundedbuffer_soft_left_distance.is_fresh = 0;
memory->boundedbuffer_soft_right_dx.is_fresh = 0;
memory->boundedbuffer_soft_right_dy.is_fresh = 0;
memory->boundedbuffer_soft_right_distance.is_fresh = 0;
memory->boundedbuffer_soft_left_right_violation.is_fresh = 0;
memory->boundedbuffer_soft_top_bottom_violation.is_fresh = 0;
memory->boundedbuffer_soft_geofence_violation_point_swapped.is_fresh = 0;
memory->boundedbuffer_soft_geofence_violation_point.is_fresh = 0;
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
shift_current_waypoint(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_lat(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_long(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_waypoint_reached(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
shift_waypoint_index(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_finished(memory);
}
if (internalevent.heading_is_present) {
shift_current_heading(memory);
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
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_geofence(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_hard_geofence(memory);
}
if (internalevent.lat_is_present) {
shift_x(memory);
}
if (internalevent.lon_is_present) {
shift_y(memory);
}
if (internalevent.lat_is_present) {
shift_delta_x(memory);
}
if (internalevent.lon_is_present) {
shift_delta_y(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_gradient(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_y_intercept(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_left_y(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_right_y(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_bottom_x(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_top_x(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_top_dx(memory);
}
if (internalevent.lon_is_present) {
shift_top_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_top_distance(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_bottom_dx(memory);
}
if (internalevent.lon_is_present) {
shift_bottom_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_bottom_distance(memory);
}
if (internalevent.lat_is_present) {
shift_left_dx(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_left_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_left_distance(memory);
}
if (internalevent.lat_is_present) {
shift_right_dx(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_right_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_right_distance(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_left_right_violation(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_top_bottom_violation(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_nearest_geofence_violation(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_distance_to_geofence(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_geofence_violation_point(memory);
}
if ((((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present)) || ((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.skip_waypoint_is_present))) {
shift_estimated_time_until_geofence(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_left_y(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_right_y(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_bottom_x(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_top_x(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_top_dx(memory);
}
if (internalevent.lon_is_present) {
shift_soft_top_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_top_distance(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_bottom_dx(memory);
}
if (internalevent.lon_is_present) {
shift_soft_bottom_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_bottom_distance(memory);
}
if (internalevent.lat_is_present) {
shift_soft_left_dx(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_left_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_left_distance(memory);
}
if (internalevent.lat_is_present) {
shift_soft_right_dx(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
shift_soft_right_dy(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_right_distance(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_left_right_violation(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_top_bottom_violation(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_geofence_violation_point_swapped(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
shift_soft_geofence_violation_point(memory);
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
if (internalevent.heading_is_present) {
eval_current_heading_0(memory);
}
if (internalevent.lat_is_present) {
eval_x_0(memory);
}
if (internalevent.lon_is_present) {
eval_y_0(memory);
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
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_geofence_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_hard_geofence_0(memory);
}
if (internalevent.lat_is_present) {
eval_delta_x_0(memory);
}
if (internalevent.lon_is_present) {
eval_delta_y_0(memory);
}
if (internalevent.lon_is_present) {
eval_top_dy_0(memory);
}
if (internalevent.lon_is_present) {
eval_bottom_dy_0(memory);
}
if (internalevent.lat_is_present) {
eval_left_dx_0(memory);
}
if (internalevent.lat_is_present) {
eval_right_dx_0(memory);
}
if (internalevent.lon_is_present) {
eval_soft_top_dy_0(memory);
}
if (internalevent.lon_is_present) {
eval_soft_bottom_dy_0(memory);
}
if (internalevent.lat_is_present) {
eval_soft_left_dx_0(memory);
}
if (internalevent.lat_is_present) {
eval_soft_right_dx_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_lat_rad_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_long_rad_0(memory);
}
if (((internalevent.lat_is_present && internalevent.lon_is_present) && expr_0(memory))) {
shift_trigger_0(memory);
}
if (((internalevent.lat_is_present && internalevent.lon_is_present) && expr_1(memory))) {
shift_trigger_1(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_gradient_0(memory);
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
if (((internalevent.lat_is_present && internalevent.lon_is_present) && expr_0(memory))) {
eval_trigger_0_0(memory);
}
if (((internalevent.lat_is_present && internalevent.lon_is_present) && expr_1(memory))) {
eval_trigger_1_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_y_intercept_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_distance_to_waypoint_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_r_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_left_y_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_right_y_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_bottom_x_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_top_x_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_left_y_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_right_y_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_bottom_x_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_top_x_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_reached_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_phi_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_top_dx_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_bottom_dx_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_left_dy_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_right_dy_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_top_dx_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_bottom_dx_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_left_dy_0(memory);
}
if ((internalevent.lat_is_present && internalevent.lon_is_present)) {
eval_soft_right_dy_0(memory);
}
if ((((((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present)) || internalevent.reset_waypoints_is_present) || (internalevent.reset_waypoints_is_present && internalevent.skip_waypoint_is_present)) || internalevent.skip_waypoint_is_present)) {
eval_waypoint_index_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_waypoint_direction_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_top_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_bottom_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_left_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_right_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_top_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_bottom_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_left_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_right_distance_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_finished_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_left_right_violation_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_top_bottom_violation_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_left_right_violation_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_top_bottom_violation_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_nearest_geofence_violation_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_geofence_violation_point_swapped_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_distance_to_geofence_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_geofence_violation_point_0(memory);
}
if (((((internalevent.lat_is_present && internalevent.lon_is_present) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || ((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.skip_waypoint_is_present))) {
eval_soft_geofence_violation_point_0(memory);
}
if ((((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present)) || ((((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.reset_waypoints_is_present) && internalevent.skip_waypoint_is_present)) || (((internalevent.lat_is_present && internalevent.lon_is_present) && internalevent.velocity_is_present) && internalevent.skip_waypoint_is_present))) {
eval_estimated_time_until_geofence_0(memory);
}
Verdict verdict = build_verdict(memory);
clear_activation(memory);
return verdict;
}