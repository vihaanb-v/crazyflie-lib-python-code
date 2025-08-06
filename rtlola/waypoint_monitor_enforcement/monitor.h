#include <stdbool.h>
char* STR_CONSTANT_0 = "X drift right (State Estimate) > 0.2m — correcting left!";
char* STR_CONSTANT_1 = "X drift left (State Estimate) < -0.2m — correcting right!";
char* STR_CONSTANT_2 = "Y drift forward (State Estimate) > 0.2m — correcting back!";
char* STR_CONSTANT_3 = "Y drift back (State Estimate) < -0.2m — correcting forward!";
char* STR_CONSTANT_4 = "Z drift up (State Estimate) > 0.2m — correcting down!";
char* STR_CONSTANT_5 = "Z drift down (State Estimate) < -0.2m — correcting up!";
char* STR_CONSTANT_6 = "X drift right (Multi-Ranger) > 0.2m — correcting left!";
char* STR_CONSTANT_7 = "X drift left (Multi-Ranger) < -0.2m — correcting right!";
char* STR_CONSTANT_8 = "Y drift forward (Multi-Ranger) > 0.2m — correcting back!";
char* STR_CONSTANT_9 = "Y drift back (Multi-Ranger) < -0.2m — correcting forward!";
char* STR_CONSTANT_10 = "Z drift up (Multi-Ranger) > 0.2m — correcting down!";
char* STR_CONSTANT_11 = "Z drift down (Multi-Ranger) < -0.2m — correcting up!";
char* STR_CONSTANT_12 = "Pitch exceeded 0.3 radians! Stabilizing!";
char* STR_CONSTANT_13 = "Roll exceeded 0.3 radians! Stabilizing!";
char* STR_CONSTANT_14 = "Yaw exceeded 0.3 radians! Stabilizing!";
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_x_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_y_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_z_drift;
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
} BoundedBuffer_roll;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_yaw;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_x_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_y_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_z_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_pitch;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_roll;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_yaw;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_x_drift_pos_exceeded;
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
} BoundedBuffer_x_drift_neg_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_1;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_y_drift_pos_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_2;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_y_drift_neg_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_3;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_z_drift_pos_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_4;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_z_drift_neg_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_5;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_x_drift_pos_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_6;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_x_drift_neg_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_7;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_y_drift_pos_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_8;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_y_drift_neg_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_9;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_z_drift_pos_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_10;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_multi_ranger_z_drift_neg_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_11;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_pitch_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_12;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_roll_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_13;
typedef struct {
bool values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_yaw_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_14;
typedef struct {
BoundedBuffer_x_drift boundedbuffer_x_drift;
BoundedBuffer_y_drift boundedbuffer_y_drift;
BoundedBuffer_z_drift boundedbuffer_z_drift;
BoundedBuffer_pitch boundedbuffer_pitch;
BoundedBuffer_roll boundedbuffer_roll;
BoundedBuffer_yaw boundedbuffer_yaw;
BoundedBuffer_multi_ranger_x_drift boundedbuffer_multi_ranger_x_drift;
BoundedBuffer_multi_ranger_y_drift boundedbuffer_multi_ranger_y_drift;
BoundedBuffer_multi_ranger_z_drift boundedbuffer_multi_ranger_z_drift;
BoundedBuffer_abs_pitch boundedbuffer_abs_pitch;
BoundedBuffer_abs_roll boundedbuffer_abs_roll;
BoundedBuffer_abs_yaw boundedbuffer_abs_yaw;
BoundedBuffer_x_drift_pos_exceeded boundedbuffer_x_drift_pos_exceeded;
BoundedBuffer_trigger_0 boundedbuffer_trigger_0;
BoundedBuffer_x_drift_neg_exceeded boundedbuffer_x_drift_neg_exceeded;
BoundedBuffer_trigger_1 boundedbuffer_trigger_1;
BoundedBuffer_y_drift_pos_exceeded boundedbuffer_y_drift_pos_exceeded;
BoundedBuffer_trigger_2 boundedbuffer_trigger_2;
BoundedBuffer_y_drift_neg_exceeded boundedbuffer_y_drift_neg_exceeded;
BoundedBuffer_trigger_3 boundedbuffer_trigger_3;
BoundedBuffer_z_drift_pos_exceeded boundedbuffer_z_drift_pos_exceeded;
BoundedBuffer_trigger_4 boundedbuffer_trigger_4;
BoundedBuffer_z_drift_neg_exceeded boundedbuffer_z_drift_neg_exceeded;
BoundedBuffer_trigger_5 boundedbuffer_trigger_5;
BoundedBuffer_multi_ranger_x_drift_pos_exceeded boundedbuffer_multi_ranger_x_drift_pos_exceeded;
BoundedBuffer_trigger_6 boundedbuffer_trigger_6;
BoundedBuffer_multi_ranger_x_drift_neg_exceeded boundedbuffer_multi_ranger_x_drift_neg_exceeded;
BoundedBuffer_trigger_7 boundedbuffer_trigger_7;
BoundedBuffer_multi_ranger_y_drift_pos_exceeded boundedbuffer_multi_ranger_y_drift_pos_exceeded;
BoundedBuffer_trigger_8 boundedbuffer_trigger_8;
BoundedBuffer_multi_ranger_y_drift_neg_exceeded boundedbuffer_multi_ranger_y_drift_neg_exceeded;
BoundedBuffer_trigger_9 boundedbuffer_trigger_9;
BoundedBuffer_multi_ranger_z_drift_pos_exceeded boundedbuffer_multi_ranger_z_drift_pos_exceeded;
BoundedBuffer_trigger_10 boundedbuffer_trigger_10;
BoundedBuffer_multi_ranger_z_drift_neg_exceeded boundedbuffer_multi_ranger_z_drift_neg_exceeded;
BoundedBuffer_trigger_11 boundedbuffer_trigger_11;
BoundedBuffer_pitch_exceeded boundedbuffer_pitch_exceeded;
BoundedBuffer_trigger_12 boundedbuffer_trigger_12;
BoundedBuffer_roll_exceeded boundedbuffer_roll_exceeded;
BoundedBuffer_trigger_13 boundedbuffer_trigger_13;
BoundedBuffer_yaw_exceeded boundedbuffer_yaw_exceeded;
BoundedBuffer_trigger_14 boundedbuffer_trigger_14;
double time;
} Memory;
typedef struct {
double x_drift;
bool x_drift_is_present;
double y_drift;
bool y_drift_is_present;
double z_drift;
bool z_drift_is_present;
double pitch;
bool pitch_is_present;
double roll;
bool roll_is_present;
double yaw;
bool yaw_is_present;
double multi_ranger_x_drift;
bool multi_ranger_x_drift_is_present;
double multi_ranger_y_drift;
bool multi_ranger_y_drift_is_present;
double multi_ranger_z_drift;
bool multi_ranger_z_drift_is_present;
double time;
} InternalEvent;
typedef struct {
double abs_pitch;
bool abs_pitch_is_present;
double abs_roll;
bool abs_roll_is_present;
double abs_yaw;
bool abs_yaw_is_present;
bool x_drift_pos_exceeded;
bool x_drift_pos_exceeded_is_present;
char* trigger_0;
bool trigger_0_is_present;
bool x_drift_neg_exceeded;
bool x_drift_neg_exceeded_is_present;
char* trigger_1;
bool trigger_1_is_present;
bool y_drift_pos_exceeded;
bool y_drift_pos_exceeded_is_present;
char* trigger_2;
bool trigger_2_is_present;
bool y_drift_neg_exceeded;
bool y_drift_neg_exceeded_is_present;
char* trigger_3;
bool trigger_3_is_present;
bool z_drift_pos_exceeded;
bool z_drift_pos_exceeded_is_present;
char* trigger_4;
bool trigger_4_is_present;
bool z_drift_neg_exceeded;
bool z_drift_neg_exceeded_is_present;
char* trigger_5;
bool trigger_5_is_present;
bool multi_ranger_x_drift_pos_exceeded;
bool multi_ranger_x_drift_pos_exceeded_is_present;
char* trigger_6;
bool trigger_6_is_present;
bool multi_ranger_x_drift_neg_exceeded;
bool multi_ranger_x_drift_neg_exceeded_is_present;
char* trigger_7;
bool trigger_7_is_present;
bool multi_ranger_y_drift_pos_exceeded;
bool multi_ranger_y_drift_pos_exceeded_is_present;
char* trigger_8;
bool trigger_8_is_present;
bool multi_ranger_y_drift_neg_exceeded;
bool multi_ranger_y_drift_neg_exceeded_is_present;
char* trigger_9;
bool trigger_9_is_present;
bool multi_ranger_z_drift_pos_exceeded;
bool multi_ranger_z_drift_pos_exceeded_is_present;
char* trigger_10;
bool trigger_10_is_present;
bool multi_ranger_z_drift_neg_exceeded;
bool multi_ranger_z_drift_neg_exceeded_is_present;
char* trigger_11;
bool trigger_11_is_present;
bool pitch_exceeded;
bool pitch_exceeded_is_present;
char* trigger_12;
bool trigger_12_is_present;
bool roll_exceeded;
bool roll_exceeded_is_present;
char* trigger_13;
bool trigger_13_is_present;
bool yaw_exceeded;
bool yaw_exceeded_is_present;
char* trigger_14;
bool trigger_14_is_present;
double time;
} Verdict;
Verdict cycle(Memory* memory, InternalEvent internalevent);