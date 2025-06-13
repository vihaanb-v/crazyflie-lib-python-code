#include <stdbool.h>
#include <stdint.h>
char* STR_CONSTANT_0 = "warning:Slowly return to flight area.";
char* STR_CONSTANT_1 = "violation:Return to flight area immediately.";
typedef struct {
double _0;
double _1;
} TUPLE2_DOUBLE__DOUBLE;
typedef struct {
double _0;
TUPLE2_DOUBLE__DOUBLE _1;
} TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_lat;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_lon;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_heading;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_pitch;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_bank;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_velocity;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_airspeed;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_vertical_speed;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_acceleration_x;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_acceleration_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_acceleration_z;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_altitude;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_reset_waypoints;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_skip_waypoint;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_velocity_kmh;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_current_lat;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_current_long;
typedef struct {
TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_current_waypoint;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_lat;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_long;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_reached;
typedef struct {
uint64_t values[2];
bool valid[2];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_index;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_finished;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_current_heading;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_current_lat_rad;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_current_long_rad;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_lat_rad;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_long_rad;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_mean_latitude;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_distance_to_waypoint_lat;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_distance_to_waypoint_long;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_distance_to_waypoint;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_r;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_phi;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_direction;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_geofence;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_0;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_hard_geofence;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_1;
typedef struct {
double values[2];
bool valid[2];
int current;
bool is_fresh;
} BoundedBuffer_x;
typedef struct {
double values[2];
bool valid[2];
int current;
bool is_fresh;
} BoundedBuffer_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_delta_x;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_delta_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_gradient;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_y_intercept;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_left_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_right_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_bottom_x;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_top_x;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_top_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_top_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_top_distance;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_bottom_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_bottom_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_bottom_distance;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_left_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_left_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_left_distance;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_right_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_right_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_right_distance;
typedef struct {
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_left_right_violation;
typedef struct {
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_top_bottom_violation;
typedef struct {
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_nearest_geofence_violation;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_distance_to_geofence;
typedef struct {
TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_geofence_violation_point;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_estimated_time_until_geofence;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_left_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_right_y;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_bottom_x;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_top_x;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_top_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_top_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_top_distance;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_bottom_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_bottom_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_bottom_distance;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_left_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_left_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_left_distance;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_right_dx;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_right_dy;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_right_distance;
typedef struct {
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_left_right_violation;
typedef struct {
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_top_bottom_violation;
typedef struct {
TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_geofence_violation_point_swapped;
typedef struct {
TUPLE2_DOUBLE__DOUBLE values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_soft_geofence_violation_point;
typedef struct {
BoundedBuffer_lat boundedbuffer_lat;
BoundedBuffer_lon boundedbuffer_lon;
BoundedBuffer_heading boundedbuffer_heading;
BoundedBuffer_pitch boundedbuffer_pitch;
BoundedBuffer_bank boundedbuffer_bank;
BoundedBuffer_velocity boundedbuffer_velocity;
BoundedBuffer_airspeed boundedbuffer_airspeed;
BoundedBuffer_vertical_speed boundedbuffer_vertical_speed;
BoundedBuffer_acceleration_x boundedbuffer_acceleration_x;
BoundedBuffer_acceleration_y boundedbuffer_acceleration_y;
BoundedBuffer_acceleration_z boundedbuffer_acceleration_z;
BoundedBuffer_altitude boundedbuffer_altitude;
BoundedBuffer_reset_waypoints boundedbuffer_reset_waypoints;
BoundedBuffer_skip_waypoint boundedbuffer_skip_waypoint;
BoundedBuffer_velocity_kmh boundedbuffer_velocity_kmh;
BoundedBuffer_current_lat boundedbuffer_current_lat;
BoundedBuffer_current_long boundedbuffer_current_long;
BoundedBuffer_current_waypoint boundedbuffer_current_waypoint;
BoundedBuffer_waypoint_lat boundedbuffer_waypoint_lat;
BoundedBuffer_waypoint_long boundedbuffer_waypoint_long;
BoundedBuffer_waypoint_reached boundedbuffer_waypoint_reached;
BoundedBuffer_waypoint_index boundedbuffer_waypoint_index;
BoundedBuffer_finished boundedbuffer_finished;
BoundedBuffer_current_heading boundedbuffer_current_heading;
BoundedBuffer_current_lat_rad boundedbuffer_current_lat_rad;
BoundedBuffer_current_long_rad boundedbuffer_current_long_rad;
BoundedBuffer_waypoint_lat_rad boundedbuffer_waypoint_lat_rad;
BoundedBuffer_waypoint_long_rad boundedbuffer_waypoint_long_rad;
BoundedBuffer_mean_latitude boundedbuffer_mean_latitude;
BoundedBuffer_distance_to_waypoint_lat boundedbuffer_distance_to_waypoint_lat;
BoundedBuffer_distance_to_waypoint_long boundedbuffer_distance_to_waypoint_long;
BoundedBuffer_distance_to_waypoint boundedbuffer_distance_to_waypoint;
BoundedBuffer_waypoint_r boundedbuffer_waypoint_r;
BoundedBuffer_waypoint_phi boundedbuffer_waypoint_phi;
BoundedBuffer_waypoint_direction boundedbuffer_waypoint_direction;
BoundedBuffer_soft_geofence boundedbuffer_soft_geofence;
BoundedBuffer_trigger_0 boundedbuffer_trigger_0;
BoundedBuffer_hard_geofence boundedbuffer_hard_geofence;
BoundedBuffer_trigger_1 boundedbuffer_trigger_1;
BoundedBuffer_x boundedbuffer_x;
BoundedBuffer_y boundedbuffer_y;
BoundedBuffer_delta_x boundedbuffer_delta_x;
BoundedBuffer_delta_y boundedbuffer_delta_y;
BoundedBuffer_gradient boundedbuffer_gradient;
BoundedBuffer_y_intercept boundedbuffer_y_intercept;
BoundedBuffer_left_y boundedbuffer_left_y;
BoundedBuffer_right_y boundedbuffer_right_y;
BoundedBuffer_bottom_x boundedbuffer_bottom_x;
BoundedBuffer_top_x boundedbuffer_top_x;
BoundedBuffer_top_dx boundedbuffer_top_dx;
BoundedBuffer_top_dy boundedbuffer_top_dy;
BoundedBuffer_top_distance boundedbuffer_top_distance;
BoundedBuffer_bottom_dx boundedbuffer_bottom_dx;
BoundedBuffer_bottom_dy boundedbuffer_bottom_dy;
BoundedBuffer_bottom_distance boundedbuffer_bottom_distance;
BoundedBuffer_left_dx boundedbuffer_left_dx;
BoundedBuffer_left_dy boundedbuffer_left_dy;
BoundedBuffer_left_distance boundedbuffer_left_distance;
BoundedBuffer_right_dx boundedbuffer_right_dx;
BoundedBuffer_right_dy boundedbuffer_right_dy;
BoundedBuffer_right_distance boundedbuffer_right_distance;
BoundedBuffer_left_right_violation boundedbuffer_left_right_violation;
BoundedBuffer_top_bottom_violation boundedbuffer_top_bottom_violation;
BoundedBuffer_nearest_geofence_violation boundedbuffer_nearest_geofence_violation;
BoundedBuffer_distance_to_geofence boundedbuffer_distance_to_geofence;
BoundedBuffer_geofence_violation_point boundedbuffer_geofence_violation_point;
BoundedBuffer_estimated_time_until_geofence boundedbuffer_estimated_time_until_geofence;
BoundedBuffer_soft_left_y boundedbuffer_soft_left_y;
BoundedBuffer_soft_right_y boundedbuffer_soft_right_y;
BoundedBuffer_soft_bottom_x boundedbuffer_soft_bottom_x;
BoundedBuffer_soft_top_x boundedbuffer_soft_top_x;
BoundedBuffer_soft_top_dx boundedbuffer_soft_top_dx;
BoundedBuffer_soft_top_dy boundedbuffer_soft_top_dy;
BoundedBuffer_soft_top_distance boundedbuffer_soft_top_distance;
BoundedBuffer_soft_bottom_dx boundedbuffer_soft_bottom_dx;
BoundedBuffer_soft_bottom_dy boundedbuffer_soft_bottom_dy;
BoundedBuffer_soft_bottom_distance boundedbuffer_soft_bottom_distance;
BoundedBuffer_soft_left_dx boundedbuffer_soft_left_dx;
BoundedBuffer_soft_left_dy boundedbuffer_soft_left_dy;
BoundedBuffer_soft_left_distance boundedbuffer_soft_left_distance;
BoundedBuffer_soft_right_dx boundedbuffer_soft_right_dx;
BoundedBuffer_soft_right_dy boundedbuffer_soft_right_dy;
BoundedBuffer_soft_right_distance boundedbuffer_soft_right_distance;
BoundedBuffer_soft_left_right_violation boundedbuffer_soft_left_right_violation;
BoundedBuffer_soft_top_bottom_violation boundedbuffer_soft_top_bottom_violation;
BoundedBuffer_soft_geofence_violation_point_swapped boundedbuffer_soft_geofence_violation_point_swapped;
BoundedBuffer_soft_geofence_violation_point boundedbuffer_soft_geofence_violation_point;
double time;
} Memory;
typedef struct {
double lat;
bool lat_is_present;
double lon;
bool lon_is_present;
double heading;
bool heading_is_present;
double pitch;
bool pitch_is_present;
double bank;
bool bank_is_present;
double velocity;
bool velocity_is_present;
double airspeed;
bool airspeed_is_present;
double vertical_speed;
bool vertical_speed_is_present;
double acceleration_x;
bool acceleration_x_is_present;
double acceleration_y;
bool acceleration_y_is_present;
double acceleration_z;
bool acceleration_z_is_present;
double altitude;
bool altitude_is_present;
bool reset_waypoints;
bool reset_waypoints_is_present;
bool skip_waypoint;
bool skip_waypoint_is_present;
double time;
} InternalEvent;
typedef struct {
double velocity_kmh;
bool velocity_kmh_is_present;
double current_lat;
bool current_lat_is_present;
double current_long;
bool current_long_is_present;
TUPLE2_DOUBLE__DOUBLE current_waypoint;
bool current_waypoint_is_present;
double waypoint_lat;
bool waypoint_lat_is_present;
double waypoint_long;
bool waypoint_long_is_present;
bool waypoint_reached;
bool waypoint_reached_is_present;
uint64_t waypoint_index;
bool waypoint_index_is_present;
bool finished;
bool finished_is_present;
double current_heading;
bool current_heading_is_present;
double current_lat_rad;
bool current_lat_rad_is_present;
double current_long_rad;
bool current_long_rad_is_present;
double waypoint_lat_rad;
bool waypoint_lat_rad_is_present;
double waypoint_long_rad;
bool waypoint_long_rad_is_present;
double mean_latitude;
bool mean_latitude_is_present;
double distance_to_waypoint_lat;
bool distance_to_waypoint_lat_is_present;
double distance_to_waypoint_long;
bool distance_to_waypoint_long_is_present;
double distance_to_waypoint;
bool distance_to_waypoint_is_present;
double waypoint_r;
bool waypoint_r_is_present;
double waypoint_phi;
bool waypoint_phi_is_present;
double waypoint_direction;
bool waypoint_direction_is_present;
bool soft_geofence;
bool soft_geofence_is_present;
char* trigger_0;
bool trigger_0_is_present;
bool hard_geofence;
bool hard_geofence_is_present;
char* trigger_1;
bool trigger_1_is_present;
double x;
bool x_is_present;
double y;
bool y_is_present;
double delta_x;
bool delta_x_is_present;
double delta_y;
bool delta_y_is_present;
double gradient;
bool gradient_is_present;
double y_intercept;
bool y_intercept_is_present;
double left_y;
bool left_y_is_present;
double right_y;
bool right_y_is_present;
double bottom_x;
bool bottom_x_is_present;
double top_x;
bool top_x_is_present;
double top_dx;
bool top_dx_is_present;
double top_dy;
bool top_dy_is_present;
double top_distance;
bool top_distance_is_present;
double bottom_dx;
bool bottom_dx_is_present;
double bottom_dy;
bool bottom_dy_is_present;
double bottom_distance;
bool bottom_distance_is_present;
double left_dx;
bool left_dx_is_present;
double left_dy;
bool left_dy_is_present;
double left_distance;
bool left_distance_is_present;
double right_dx;
bool right_dx_is_present;
double right_dy;
bool right_dy_is_present;
double right_distance;
bool right_distance_is_present;
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE left_right_violation;
bool left_right_violation_is_present;
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE top_bottom_violation;
bool top_bottom_violation_is_present;
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE nearest_geofence_violation;
bool nearest_geofence_violation_is_present;
double distance_to_geofence;
bool distance_to_geofence_is_present;
TUPLE2_DOUBLE__DOUBLE geofence_violation_point;
bool geofence_violation_point_is_present;
double estimated_time_until_geofence;
bool estimated_time_until_geofence_is_present;
double soft_left_y;
bool soft_left_y_is_present;
double soft_right_y;
bool soft_right_y_is_present;
double soft_bottom_x;
bool soft_bottom_x_is_present;
double soft_top_x;
bool soft_top_x_is_present;
double soft_top_dx;
bool soft_top_dx_is_present;
double soft_top_dy;
bool soft_top_dy_is_present;
double soft_top_distance;
bool soft_top_distance_is_present;
double soft_bottom_dx;
bool soft_bottom_dx_is_present;
double soft_bottom_dy;
bool soft_bottom_dy_is_present;
double soft_bottom_distance;
bool soft_bottom_distance_is_present;
double soft_left_dx;
bool soft_left_dx_is_present;
double soft_left_dy;
bool soft_left_dy_is_present;
double soft_left_distance;
bool soft_left_distance_is_present;
double soft_right_dx;
bool soft_right_dx_is_present;
double soft_right_dy;
bool soft_right_dy_is_present;
double soft_right_distance;
bool soft_right_distance_is_present;
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE soft_left_right_violation;
bool soft_left_right_violation_is_present;
TUPLE2_DOUBLE__TUPLE2_DOUBLE__DOUBLE soft_top_bottom_violation;
bool soft_top_bottom_violation_is_present;
TUPLE2_DOUBLE__DOUBLE soft_geofence_violation_point_swapped;
bool soft_geofence_violation_point_swapped_is_present;
TUPLE2_DOUBLE__DOUBLE soft_geofence_violation_point;
bool soft_geofence_violation_point_is_present;
double time;
} Verdict;
Verdict cycle(Memory* memory, InternalEvent internalevent);