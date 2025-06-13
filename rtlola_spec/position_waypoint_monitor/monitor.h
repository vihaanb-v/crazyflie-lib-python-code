#include <stdbool.h>
#include <stdint.h>
typedef struct {
double _0;
double _1;
} TUPLE2_DOUBLE__DOUBLE;
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
uint64_t values[2];
bool valid[2];
int current;
bool is_fresh;
} BoundedBuffer_waypoint_index;
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
} BoundedBuffer_waypoint_reached;
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
} BoundedBuffer_estimated_time_until_waypoint;
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
BoundedBuffer_waypoint_index boundedbuffer_waypoint_index;
BoundedBuffer_current_waypoint boundedbuffer_current_waypoint;
BoundedBuffer_waypoint_lat boundedbuffer_waypoint_lat;
BoundedBuffer_waypoint_long boundedbuffer_waypoint_long;
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
BoundedBuffer_waypoint_reached boundedbuffer_waypoint_reached;
BoundedBuffer_finished boundedbuffer_finished;
BoundedBuffer_estimated_time_until_waypoint boundedbuffer_estimated_time_until_waypoint;
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
uint64_t waypoint_index;
bool waypoint_index_is_present;
TUPLE2_DOUBLE__DOUBLE current_waypoint;
bool current_waypoint_is_present;
double waypoint_lat;
bool waypoint_lat_is_present;
double waypoint_long;
bool waypoint_long_is_present;
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
bool waypoint_reached;
bool waypoint_reached_is_present;
bool finished;
bool finished_is_present;
double estimated_time_until_waypoint;
bool estimated_time_until_waypoint_is_present;
double time;
} Verdict;
Verdict cycle(Memory* memory, InternalEvent internalevent);