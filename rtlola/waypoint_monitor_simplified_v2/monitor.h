#include <stdbool.h>
char* STR_CONSTANT_0 = "X drift (State Estimate) exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_1 = "Y drift (State Estimate) exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_2 = "Z drift (State Estimate) exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_3 = "X drift (Multi-Ranger Estimate) exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_4 = "Y drift (Multi-Ranger Estimate) exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_5 = "Z drift (Multi-Ranger Estimate) exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_6 = "Pitch exceeded 0.3 radians! Stabilizing!";
char* STR_CONSTANT_7 = "Roll exceeded 0.3 radians! Stabilizing!";
char* STR_CONSTANT_8 = "Yaw exceeded 0.3 radians! Stabilizing!";
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
} BoundedBuffer_abs_x_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_y_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_z_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_multi_ranger_x_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_multi_ranger_y_drift;
typedef struct {
double values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_abs_multi_ranger_z_drift;
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
} BoundedBuffer_x_drift_exceeded;
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
} BoundedBuffer_y_drift_exceeded;
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
} BoundedBuffer_z_drift_exceeded;
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
} BoundedBuffer_multi_ranger_x_drift_exceeded;
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
} BoundedBuffer_multi_ranger_y_drift_exceeded;
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
} BoundedBuffer_multi_ranger_z_drift_exceeded;
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
} BoundedBuffer_pitch_exceeded;
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
} BoundedBuffer_roll_exceeded;
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
} BoundedBuffer_yaw_exceeded;
typedef struct {
char* values[1];
bool valid[1];
int current;
bool is_fresh;
} BoundedBuffer_trigger_8;
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
BoundedBuffer_abs_x_drift boundedbuffer_abs_x_drift;
BoundedBuffer_abs_y_drift boundedbuffer_abs_y_drift;
BoundedBuffer_abs_z_drift boundedbuffer_abs_z_drift;
BoundedBuffer_abs_multi_ranger_x_drift boundedbuffer_abs_multi_ranger_x_drift;
BoundedBuffer_abs_multi_ranger_y_drift boundedbuffer_abs_multi_ranger_y_drift;
BoundedBuffer_abs_multi_ranger_z_drift boundedbuffer_abs_multi_ranger_z_drift;
BoundedBuffer_abs_pitch boundedbuffer_abs_pitch;
BoundedBuffer_abs_roll boundedbuffer_abs_roll;
BoundedBuffer_abs_yaw boundedbuffer_abs_yaw;
BoundedBuffer_x_drift_exceeded boundedbuffer_x_drift_exceeded;
BoundedBuffer_trigger_0 boundedbuffer_trigger_0;
BoundedBuffer_y_drift_exceeded boundedbuffer_y_drift_exceeded;
BoundedBuffer_trigger_1 boundedbuffer_trigger_1;
BoundedBuffer_z_drift_exceeded boundedbuffer_z_drift_exceeded;
BoundedBuffer_trigger_2 boundedbuffer_trigger_2;
BoundedBuffer_multi_ranger_x_drift_exceeded boundedbuffer_multi_ranger_x_drift_exceeded;
BoundedBuffer_trigger_3 boundedbuffer_trigger_3;
BoundedBuffer_multi_ranger_y_drift_exceeded boundedbuffer_multi_ranger_y_drift_exceeded;
BoundedBuffer_trigger_4 boundedbuffer_trigger_4;
BoundedBuffer_multi_ranger_z_drift_exceeded boundedbuffer_multi_ranger_z_drift_exceeded;
BoundedBuffer_trigger_5 boundedbuffer_trigger_5;
BoundedBuffer_pitch_exceeded boundedbuffer_pitch_exceeded;
BoundedBuffer_trigger_6 boundedbuffer_trigger_6;
BoundedBuffer_roll_exceeded boundedbuffer_roll_exceeded;
BoundedBuffer_trigger_7 boundedbuffer_trigger_7;
BoundedBuffer_yaw_exceeded boundedbuffer_yaw_exceeded;
BoundedBuffer_trigger_8 boundedbuffer_trigger_8;
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
double abs_x_drift;
bool abs_x_drift_is_present;
double abs_y_drift;
bool abs_y_drift_is_present;
double abs_z_drift;
bool abs_z_drift_is_present;
double abs_multi_ranger_x_drift;
bool abs_multi_ranger_x_drift_is_present;
double abs_multi_ranger_y_drift;
bool abs_multi_ranger_y_drift_is_present;
double abs_multi_ranger_z_drift;
bool abs_multi_ranger_z_drift_is_present;
double abs_pitch;
bool abs_pitch_is_present;
double abs_roll;
bool abs_roll_is_present;
double abs_yaw;
bool abs_yaw_is_present;
bool x_drift_exceeded;
bool x_drift_exceeded_is_present;
char* trigger_0;
bool trigger_0_is_present;
bool y_drift_exceeded;
bool y_drift_exceeded_is_present;
char* trigger_1;
bool trigger_1_is_present;
bool z_drift_exceeded;
bool z_drift_exceeded_is_present;
char* trigger_2;
bool trigger_2_is_present;
bool multi_ranger_x_drift_exceeded;
bool multi_ranger_x_drift_exceeded_is_present;
char* trigger_3;
bool trigger_3_is_present;
bool multi_ranger_y_drift_exceeded;
bool multi_ranger_y_drift_exceeded_is_present;
char* trigger_4;
bool trigger_4_is_present;
bool multi_ranger_z_drift_exceeded;
bool multi_ranger_z_drift_exceeded_is_present;
char* trigger_5;
bool trigger_5_is_present;
bool pitch_exceeded;
bool pitch_exceeded_is_present;
char* trigger_6;
bool trigger_6_is_present;
bool roll_exceeded;
bool roll_exceeded_is_present;
char* trigger_7;
bool trigger_7_is_present;
bool yaw_exceeded;
bool yaw_exceeded_is_present;
char* trigger_8;
bool trigger_8_is_present;
double time;
} Verdict;
Verdict cycle(Memory* memory, InternalEvent internalevent);