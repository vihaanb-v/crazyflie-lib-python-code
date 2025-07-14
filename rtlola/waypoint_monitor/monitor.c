#include "monitor.h"
#include <stdbool.h>
#include <stdint.h>
#include <string.h>
char* STR_CONSTANT_0 = "Battery check failed! Flight aborted! Fix before flying again!";
char* STR_CONSTANT_1 = "Battery voltage dropped below 3.3V! Will either land or return to home!";
char* STR_CONSTANT_2 = "Motor check failed! Flight aborted! Fix before flying again!";
char* STR_CONSTANT_3 = "X drift exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_4 = "Y drift exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_5 = "Z drift exceeded 5 cm! Correcting position!";
char* STR_CONSTANT_6 = "Pitch exceeded 0.3 radians! Stabilizing!";
char* STR_CONSTANT_7 = "Roll exceeded 0.3 radians! Stabilizing!";
char* STR_CONSTANT_8 = "Yaw exceeded 0.3 radians! Stabilizing!";
int64_t motor_pass_sync(Memory* memory){
return memory->boundedbuffer_motor_pass.values[memory->boundedbuffer_motor_pass.current];
}
int64_t battery_pass_sync(Memory* memory){
return memory->boundedbuffer_battery_pass.values[memory->boundedbuffer_battery_pass.current];
}
double x_drift_sync(Memory* memory){
return memory->boundedbuffer_x_drift.values[memory->boundedbuffer_x_drift.current];
}
double y_drift_sync(Memory* memory){
return memory->boundedbuffer_y_drift.values[memory->boundedbuffer_y_drift.current];
}
double z_drift_sync(Memory* memory){
return memory->boundedbuffer_z_drift.values[memory->boundedbuffer_z_drift.current];
}
double pitch_sync(Memory* memory){
return memory->boundedbuffer_pitch.values[memory->boundedbuffer_pitch.current];
}
double roll_sync(Memory* memory){
return memory->boundedbuffer_roll.values[memory->boundedbuffer_roll.current];
}
double yaw_sync(Memory* memory){
return memory->boundedbuffer_yaw.values[memory->boundedbuffer_yaw.current];
}
double vbat_sync(Memory* memory){
return memory->boundedbuffer_vbat.values[memory->boundedbuffer_vbat.current];
}
double abs_x_drift_sync(Memory* memory){
return memory->boundedbuffer_abs_x_drift.values[memory->boundedbuffer_abs_x_drift.current];
}
double abs_y_drift_sync(Memory* memory){
return memory->boundedbuffer_abs_y_drift.values[memory->boundedbuffer_abs_y_drift.current];
}
double abs_z_drift_sync(Memory* memory){
return memory->boundedbuffer_abs_z_drift.values[memory->boundedbuffer_abs_z_drift.current];
}
double abs_pitch_sync(Memory* memory){
return memory->boundedbuffer_abs_pitch.values[memory->boundedbuffer_abs_pitch.current];
}
double abs_roll_sync(Memory* memory){
return memory->boundedbuffer_abs_roll.values[memory->boundedbuffer_abs_roll.current];
}
double abs_yaw_sync(Memory* memory){
return memory->boundedbuffer_abs_yaw.values[memory->boundedbuffer_abs_yaw.current];
}
int64_t motor_check_sync(Memory* memory){
return memory->boundedbuffer_motor_check.values[memory->boundedbuffer_motor_check.current];
}
bool motor_failed_sync(Memory* memory){
return memory->boundedbuffer_motor_failed.values[memory->boundedbuffer_motor_failed.current];
}
char* trigger_0_sync(Memory* memory){
return memory->boundedbuffer_trigger_0.values[memory->boundedbuffer_trigger_0.current];
}
bool battery_failed_sync(Memory* memory){
return memory->boundedbuffer_battery_failed.values[memory->boundedbuffer_battery_failed.current];
}
char* trigger_1_sync(Memory* memory){
return memory->boundedbuffer_trigger_1.values[memory->boundedbuffer_trigger_1.current];
}
bool x_drift_exceeded_sync(Memory* memory){
return memory->boundedbuffer_x_drift_exceeded.values[memory->boundedbuffer_x_drift_exceeded.current];
}
char* trigger_2_sync(Memory* memory){
return memory->boundedbuffer_trigger_2.values[memory->boundedbuffer_trigger_2.current];
}
bool y_drift_exceeded_sync(Memory* memory){
return memory->boundedbuffer_y_drift_exceeded.values[memory->boundedbuffer_y_drift_exceeded.current];
}
char* trigger_3_sync(Memory* memory){
return memory->boundedbuffer_trigger_3.values[memory->boundedbuffer_trigger_3.current];
}
bool z_drift_exceeded_sync(Memory* memory){
return memory->boundedbuffer_z_drift_exceeded.values[memory->boundedbuffer_z_drift_exceeded.current];
}
char* trigger_4_sync(Memory* memory){
return memory->boundedbuffer_trigger_4.values[memory->boundedbuffer_trigger_4.current];
}
bool pitch_exceeded_sync(Memory* memory){
return memory->boundedbuffer_pitch_exceeded.values[memory->boundedbuffer_pitch_exceeded.current];
}
char* trigger_5_sync(Memory* memory){
return memory->boundedbuffer_trigger_5.values[memory->boundedbuffer_trigger_5.current];
}
bool roll_exceeded_sync(Memory* memory){
return memory->boundedbuffer_roll_exceeded.values[memory->boundedbuffer_roll_exceeded.current];
}
char* trigger_6_sync(Memory* memory){
return memory->boundedbuffer_trigger_6.values[memory->boundedbuffer_trigger_6.current];
}
bool yaw_exceeded_sync(Memory* memory){
return memory->boundedbuffer_yaw_exceeded.values[memory->boundedbuffer_yaw_exceeded.current];
}
char* trigger_7_sync(Memory* memory){
return memory->boundedbuffer_trigger_7.values[memory->boundedbuffer_trigger_7.current];
}
bool battery_low_sync(Memory* memory){
return memory->boundedbuffer_battery_low.values[memory->boundedbuffer_battery_low.current];
}
char* trigger_8_sync(Memory* memory){
return memory->boundedbuffer_trigger_8.values[memory->boundedbuffer_trigger_8.current];
}
bool abs_x_drift_is_fresh(Memory* memory){
return memory->boundedbuffer_abs_x_drift.is_fresh;
}
bool abs_y_drift_is_fresh(Memory* memory){
return memory->boundedbuffer_abs_y_drift.is_fresh;
}
bool abs_z_drift_is_fresh(Memory* memory){
return memory->boundedbuffer_abs_z_drift.is_fresh;
}
bool abs_pitch_is_fresh(Memory* memory){
return memory->boundedbuffer_abs_pitch.is_fresh;
}
bool abs_roll_is_fresh(Memory* memory){
return memory->boundedbuffer_abs_roll.is_fresh;
}
bool abs_yaw_is_fresh(Memory* memory){
return memory->boundedbuffer_abs_yaw.is_fresh;
}
bool motor_check_is_fresh(Memory* memory){
return memory->boundedbuffer_motor_check.is_fresh;
}
bool motor_failed_is_fresh(Memory* memory){
return memory->boundedbuffer_motor_failed.is_fresh;
}
bool trigger_0_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_0.is_fresh;
}
bool battery_failed_is_fresh(Memory* memory){
return memory->boundedbuffer_battery_failed.is_fresh;
}
bool trigger_1_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_1.is_fresh;
}
bool x_drift_exceeded_is_fresh(Memory* memory){
return memory->boundedbuffer_x_drift_exceeded.is_fresh;
}
bool trigger_2_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_2.is_fresh;
}
bool y_drift_exceeded_is_fresh(Memory* memory){
return memory->boundedbuffer_y_drift_exceeded.is_fresh;
}
bool trigger_3_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_3.is_fresh;
}
bool z_drift_exceeded_is_fresh(Memory* memory){
return memory->boundedbuffer_z_drift_exceeded.is_fresh;
}
bool trigger_4_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_4.is_fresh;
}
bool pitch_exceeded_is_fresh(Memory* memory){
return memory->boundedbuffer_pitch_exceeded.is_fresh;
}
bool trigger_5_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_5.is_fresh;
}
bool roll_exceeded_is_fresh(Memory* memory){
return memory->boundedbuffer_roll_exceeded.is_fresh;
}
bool trigger_6_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_6.is_fresh;
}
bool yaw_exceeded_is_fresh(Memory* memory){
return memory->boundedbuffer_yaw_exceeded.is_fresh;
}
bool trigger_7_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_7.is_fresh;
}
bool battery_low_is_fresh(Memory* memory){
return memory->boundedbuffer_battery_low.is_fresh;
}
bool trigger_8_is_fresh(Memory* memory){
return memory->boundedbuffer_trigger_8.is_fresh;
}
void input_motor_pass(Memory* memory, int64_t new_value){
memory->boundedbuffer_motor_pass.values[memory->boundedbuffer_motor_pass.current] = new_value;
memory->boundedbuffer_motor_pass.valid[memory->boundedbuffer_motor_pass.current] = 1;
memory->boundedbuffer_motor_pass.is_fresh = 1;
}
void input_battery_pass(Memory* memory, int64_t new_value){
memory->boundedbuffer_battery_pass.values[memory->boundedbuffer_battery_pass.current] = new_value;
memory->boundedbuffer_battery_pass.valid[memory->boundedbuffer_battery_pass.current] = 1;
memory->boundedbuffer_battery_pass.is_fresh = 1;
}
void input_x_drift(Memory* memory, double new_value){
memory->boundedbuffer_x_drift.values[memory->boundedbuffer_x_drift.current] = new_value;
memory->boundedbuffer_x_drift.valid[memory->boundedbuffer_x_drift.current] = 1;
memory->boundedbuffer_x_drift.is_fresh = 1;
}
void input_y_drift(Memory* memory, double new_value){
memory->boundedbuffer_y_drift.values[memory->boundedbuffer_y_drift.current] = new_value;
memory->boundedbuffer_y_drift.valid[memory->boundedbuffer_y_drift.current] = 1;
memory->boundedbuffer_y_drift.is_fresh = 1;
}
void input_z_drift(Memory* memory, double new_value){
memory->boundedbuffer_z_drift.values[memory->boundedbuffer_z_drift.current] = new_value;
memory->boundedbuffer_z_drift.valid[memory->boundedbuffer_z_drift.current] = 1;
memory->boundedbuffer_z_drift.is_fresh = 1;
}
void input_pitch(Memory* memory, double new_value){
memory->boundedbuffer_pitch.values[memory->boundedbuffer_pitch.current] = new_value;
memory->boundedbuffer_pitch.valid[memory->boundedbuffer_pitch.current] = 1;
memory->boundedbuffer_pitch.is_fresh = 1;
}
void input_roll(Memory* memory, double new_value){
memory->boundedbuffer_roll.values[memory->boundedbuffer_roll.current] = new_value;
memory->boundedbuffer_roll.valid[memory->boundedbuffer_roll.current] = 1;
memory->boundedbuffer_roll.is_fresh = 1;
}
void input_yaw(Memory* memory, double new_value){
memory->boundedbuffer_yaw.values[memory->boundedbuffer_yaw.current] = new_value;
memory->boundedbuffer_yaw.valid[memory->boundedbuffer_yaw.current] = 1;
memory->boundedbuffer_yaw.is_fresh = 1;
}
void input_vbat(Memory* memory, double new_value){
memory->boundedbuffer_vbat.values[memory->boundedbuffer_vbat.current] = new_value;
memory->boundedbuffer_vbat.valid[memory->boundedbuffer_vbat.current] = 1;
memory->boundedbuffer_vbat.is_fresh = 1;
}
void eval_abs_x_drift_0(Memory* memory){
double new_value = ((x_drift_sync(memory) < 0)?(-x_drift_sync(memory)):x_drift_sync(memory));
memory->boundedbuffer_abs_x_drift.values[memory->boundedbuffer_abs_x_drift.current] = new_value;
memory->boundedbuffer_abs_x_drift.valid[memory->boundedbuffer_abs_x_drift.current] = 1;
memory->boundedbuffer_abs_x_drift.is_fresh = 1;
}
void eval_abs_y_drift_0(Memory* memory){
double new_value = ((y_drift_sync(memory) < 0)?(-y_drift_sync(memory)):y_drift_sync(memory));
memory->boundedbuffer_abs_y_drift.values[memory->boundedbuffer_abs_y_drift.current] = new_value;
memory->boundedbuffer_abs_y_drift.valid[memory->boundedbuffer_abs_y_drift.current] = 1;
memory->boundedbuffer_abs_y_drift.is_fresh = 1;
}
void eval_abs_z_drift_0(Memory* memory){
double new_value = ((z_drift_sync(memory) < 0)?(-z_drift_sync(memory)):z_drift_sync(memory));
memory->boundedbuffer_abs_z_drift.values[memory->boundedbuffer_abs_z_drift.current] = new_value;
memory->boundedbuffer_abs_z_drift.valid[memory->boundedbuffer_abs_z_drift.current] = 1;
memory->boundedbuffer_abs_z_drift.is_fresh = 1;
}
void eval_abs_pitch_0(Memory* memory){
double new_value = ((pitch_sync(memory) < 0)?(-pitch_sync(memory)):pitch_sync(memory));
memory->boundedbuffer_abs_pitch.values[memory->boundedbuffer_abs_pitch.current] = new_value;
memory->boundedbuffer_abs_pitch.valid[memory->boundedbuffer_abs_pitch.current] = 1;
memory->boundedbuffer_abs_pitch.is_fresh = 1;
}
void eval_abs_roll_0(Memory* memory){
double new_value = ((roll_sync(memory) < 0)?(-roll_sync(memory)):roll_sync(memory));
memory->boundedbuffer_abs_roll.values[memory->boundedbuffer_abs_roll.current] = new_value;
memory->boundedbuffer_abs_roll.valid[memory->boundedbuffer_abs_roll.current] = 1;
memory->boundedbuffer_abs_roll.is_fresh = 1;
}
void eval_abs_yaw_0(Memory* memory){
double new_value = ((yaw_sync(memory) < 0)?(-yaw_sync(memory)):yaw_sync(memory));
memory->boundedbuffer_abs_yaw.values[memory->boundedbuffer_abs_yaw.current] = new_value;
memory->boundedbuffer_abs_yaw.valid[memory->boundedbuffer_abs_yaw.current] = 1;
memory->boundedbuffer_abs_yaw.is_fresh = 1;
}
void eval_motor_check_0(Memory* memory){
int64_t new_value = (motor_pass_sync(memory) & 15);
memory->boundedbuffer_motor_check.values[memory->boundedbuffer_motor_check.current] = new_value;
memory->boundedbuffer_motor_check.valid[memory->boundedbuffer_motor_check.current] = 1;
memory->boundedbuffer_motor_check.is_fresh = 1;
}
void eval_motor_failed_0(Memory* memory){
bool new_value = (motor_check_sync(memory) != 15);
memory->boundedbuffer_motor_failed.values[memory->boundedbuffer_motor_failed.current] = new_value;
memory->boundedbuffer_motor_failed.valid[memory->boundedbuffer_motor_failed.current] = 1;
memory->boundedbuffer_motor_failed.is_fresh = 1;
}
void eval_trigger_0_0(Memory* memory){
char* new_value = STR_CONSTANT_2;
memory->boundedbuffer_trigger_0.values[memory->boundedbuffer_trigger_0.current] = new_value;
memory->boundedbuffer_trigger_0.valid[memory->boundedbuffer_trigger_0.current] = 1;
memory->boundedbuffer_trigger_0.is_fresh = 1;
}
void eval_battery_failed_0(Memory* memory){
bool new_value = (battery_pass_sync(memory) == 0);
memory->boundedbuffer_battery_failed.values[memory->boundedbuffer_battery_failed.current] = new_value;
memory->boundedbuffer_battery_failed.valid[memory->boundedbuffer_battery_failed.current] = 1;
memory->boundedbuffer_battery_failed.is_fresh = 1;
}
void eval_trigger_1_0(Memory* memory){
char* new_value = STR_CONSTANT_0;
memory->boundedbuffer_trigger_1.values[memory->boundedbuffer_trigger_1.current] = new_value;
memory->boundedbuffer_trigger_1.valid[memory->boundedbuffer_trigger_1.current] = 1;
memory->boundedbuffer_trigger_1.is_fresh = 1;
}
void eval_x_drift_exceeded_0(Memory* memory){
bool new_value = (abs_x_drift_sync(memory) > 0.05);
memory->boundedbuffer_x_drift_exceeded.values[memory->boundedbuffer_x_drift_exceeded.current] = new_value;
memory->boundedbuffer_x_drift_exceeded.valid[memory->boundedbuffer_x_drift_exceeded.current] = 1;
memory->boundedbuffer_x_drift_exceeded.is_fresh = 1;
}
void eval_trigger_2_0(Memory* memory){
char* new_value = STR_CONSTANT_3;
memory->boundedbuffer_trigger_2.values[memory->boundedbuffer_trigger_2.current] = new_value;
memory->boundedbuffer_trigger_2.valid[memory->boundedbuffer_trigger_2.current] = 1;
memory->boundedbuffer_trigger_2.is_fresh = 1;
}
void eval_y_drift_exceeded_0(Memory* memory){
bool new_value = (abs_y_drift_sync(memory) > 0.05);
memory->boundedbuffer_y_drift_exceeded.values[memory->boundedbuffer_y_drift_exceeded.current] = new_value;
memory->boundedbuffer_y_drift_exceeded.valid[memory->boundedbuffer_y_drift_exceeded.current] = 1;
memory->boundedbuffer_y_drift_exceeded.is_fresh = 1;
}
void eval_trigger_3_0(Memory* memory){
char* new_value = STR_CONSTANT_4;
memory->boundedbuffer_trigger_3.values[memory->boundedbuffer_trigger_3.current] = new_value;
memory->boundedbuffer_trigger_3.valid[memory->boundedbuffer_trigger_3.current] = 1;
memory->boundedbuffer_trigger_3.is_fresh = 1;
}
void eval_z_drift_exceeded_0(Memory* memory){
bool new_value = (abs_z_drift_sync(memory) > 0.05);
memory->boundedbuffer_z_drift_exceeded.values[memory->boundedbuffer_z_drift_exceeded.current] = new_value;
memory->boundedbuffer_z_drift_exceeded.valid[memory->boundedbuffer_z_drift_exceeded.current] = 1;
memory->boundedbuffer_z_drift_exceeded.is_fresh = 1;
}
void eval_trigger_4_0(Memory* memory){
char* new_value = STR_CONSTANT_5;
memory->boundedbuffer_trigger_4.values[memory->boundedbuffer_trigger_4.current] = new_value;
memory->boundedbuffer_trigger_4.valid[memory->boundedbuffer_trigger_4.current] = 1;
memory->boundedbuffer_trigger_4.is_fresh = 1;
}
void eval_pitch_exceeded_0(Memory* memory){
bool new_value = (abs_pitch_sync(memory) > 0.3);
memory->boundedbuffer_pitch_exceeded.values[memory->boundedbuffer_pitch_exceeded.current] = new_value;
memory->boundedbuffer_pitch_exceeded.valid[memory->boundedbuffer_pitch_exceeded.current] = 1;
memory->boundedbuffer_pitch_exceeded.is_fresh = 1;
}
void eval_trigger_5_0(Memory* memory){
char* new_value = STR_CONSTANT_6;
memory->boundedbuffer_trigger_5.values[memory->boundedbuffer_trigger_5.current] = new_value;
memory->boundedbuffer_trigger_5.valid[memory->boundedbuffer_trigger_5.current] = 1;
memory->boundedbuffer_trigger_5.is_fresh = 1;
}
void eval_roll_exceeded_0(Memory* memory){
bool new_value = (abs_roll_sync(memory) > 0.3);
memory->boundedbuffer_roll_exceeded.values[memory->boundedbuffer_roll_exceeded.current] = new_value;
memory->boundedbuffer_roll_exceeded.valid[memory->boundedbuffer_roll_exceeded.current] = 1;
memory->boundedbuffer_roll_exceeded.is_fresh = 1;
}
void eval_trigger_6_0(Memory* memory){
char* new_value = STR_CONSTANT_7;
memory->boundedbuffer_trigger_6.values[memory->boundedbuffer_trigger_6.current] = new_value;
memory->boundedbuffer_trigger_6.valid[memory->boundedbuffer_trigger_6.current] = 1;
memory->boundedbuffer_trigger_6.is_fresh = 1;
}
void eval_yaw_exceeded_0(Memory* memory){
bool new_value = (abs_yaw_sync(memory) > 0.3);
memory->boundedbuffer_yaw_exceeded.values[memory->boundedbuffer_yaw_exceeded.current] = new_value;
memory->boundedbuffer_yaw_exceeded.valid[memory->boundedbuffer_yaw_exceeded.current] = 1;
memory->boundedbuffer_yaw_exceeded.is_fresh = 1;
}
void eval_trigger_7_0(Memory* memory){
char* new_value = STR_CONSTANT_8;
memory->boundedbuffer_trigger_7.values[memory->boundedbuffer_trigger_7.current] = new_value;
memory->boundedbuffer_trigger_7.valid[memory->boundedbuffer_trigger_7.current] = 1;
memory->boundedbuffer_trigger_7.is_fresh = 1;
}
void eval_battery_low_0(Memory* memory){
bool new_value = (vbat_sync(memory) < 3.3);
memory->boundedbuffer_battery_low.values[memory->boundedbuffer_battery_low.current] = new_value;
memory->boundedbuffer_battery_low.valid[memory->boundedbuffer_battery_low.current] = 1;
memory->boundedbuffer_battery_low.is_fresh = 1;
}
void eval_trigger_8_0(Memory* memory){
char* new_value = STR_CONSTANT_1;
memory->boundedbuffer_trigger_8.values[memory->boundedbuffer_trigger_8.current] = new_value;
memory->boundedbuffer_trigger_8.valid[memory->boundedbuffer_trigger_8.current] = 1;
memory->boundedbuffer_trigger_8.is_fresh = 1;
}
void shift_motor_pass(Memory* memory){
memory->boundedbuffer_motor_pass.current = (memory->boundedbuffer_motor_pass.current + 1) % 1;
}
void shift_battery_pass(Memory* memory){
memory->boundedbuffer_battery_pass.current = (memory->boundedbuffer_battery_pass.current + 1) % 1;
}
void shift_x_drift(Memory* memory){
memory->boundedbuffer_x_drift.current = (memory->boundedbuffer_x_drift.current + 1) % 1;
}
void shift_y_drift(Memory* memory){
memory->boundedbuffer_y_drift.current = (memory->boundedbuffer_y_drift.current + 1) % 1;
}
void shift_z_drift(Memory* memory){
memory->boundedbuffer_z_drift.current = (memory->boundedbuffer_z_drift.current + 1) % 1;
}
void shift_pitch(Memory* memory){
memory->boundedbuffer_pitch.current = (memory->boundedbuffer_pitch.current + 1) % 1;
}
void shift_roll(Memory* memory){
memory->boundedbuffer_roll.current = (memory->boundedbuffer_roll.current + 1) % 1;
}
void shift_yaw(Memory* memory){
memory->boundedbuffer_yaw.current = (memory->boundedbuffer_yaw.current + 1) % 1;
}
void shift_vbat(Memory* memory){
memory->boundedbuffer_vbat.current = (memory->boundedbuffer_vbat.current + 1) % 1;
}
void shift_abs_x_drift(Memory* memory){
memory->boundedbuffer_abs_x_drift.current = (memory->boundedbuffer_abs_x_drift.current + 1) % 1;
}
void shift_abs_y_drift(Memory* memory){
memory->boundedbuffer_abs_y_drift.current = (memory->boundedbuffer_abs_y_drift.current + 1) % 1;
}
void shift_abs_z_drift(Memory* memory){
memory->boundedbuffer_abs_z_drift.current = (memory->boundedbuffer_abs_z_drift.current + 1) % 1;
}
void shift_abs_pitch(Memory* memory){
memory->boundedbuffer_abs_pitch.current = (memory->boundedbuffer_abs_pitch.current + 1) % 1;
}
void shift_abs_roll(Memory* memory){
memory->boundedbuffer_abs_roll.current = (memory->boundedbuffer_abs_roll.current + 1) % 1;
}
void shift_abs_yaw(Memory* memory){
memory->boundedbuffer_abs_yaw.current = (memory->boundedbuffer_abs_yaw.current + 1) % 1;
}
void shift_motor_check(Memory* memory){
memory->boundedbuffer_motor_check.current = (memory->boundedbuffer_motor_check.current + 1) % 1;
}
void shift_motor_failed(Memory* memory){
memory->boundedbuffer_motor_failed.current = (memory->boundedbuffer_motor_failed.current + 1) % 1;
}
void shift_trigger_0(Memory* memory){
memory->boundedbuffer_trigger_0.current = (memory->boundedbuffer_trigger_0.current + 1) % 1;
}
void shift_battery_failed(Memory* memory){
memory->boundedbuffer_battery_failed.current = (memory->boundedbuffer_battery_failed.current + 1) % 1;
}
void shift_trigger_1(Memory* memory){
memory->boundedbuffer_trigger_1.current = (memory->boundedbuffer_trigger_1.current + 1) % 1;
}
void shift_x_drift_exceeded(Memory* memory){
memory->boundedbuffer_x_drift_exceeded.current = (memory->boundedbuffer_x_drift_exceeded.current + 1) % 1;
}
void shift_trigger_2(Memory* memory){
memory->boundedbuffer_trigger_2.current = (memory->boundedbuffer_trigger_2.current + 1) % 1;
}
void shift_y_drift_exceeded(Memory* memory){
memory->boundedbuffer_y_drift_exceeded.current = (memory->boundedbuffer_y_drift_exceeded.current + 1) % 1;
}
void shift_trigger_3(Memory* memory){
memory->boundedbuffer_trigger_3.current = (memory->boundedbuffer_trigger_3.current + 1) % 1;
}
void shift_z_drift_exceeded(Memory* memory){
memory->boundedbuffer_z_drift_exceeded.current = (memory->boundedbuffer_z_drift_exceeded.current + 1) % 1;
}
void shift_trigger_4(Memory* memory){
memory->boundedbuffer_trigger_4.current = (memory->boundedbuffer_trigger_4.current + 1) % 1;
}
void shift_pitch_exceeded(Memory* memory){
memory->boundedbuffer_pitch_exceeded.current = (memory->boundedbuffer_pitch_exceeded.current + 1) % 1;
}
void shift_trigger_5(Memory* memory){
memory->boundedbuffer_trigger_5.current = (memory->boundedbuffer_trigger_5.current + 1) % 1;
}
void shift_roll_exceeded(Memory* memory){
memory->boundedbuffer_roll_exceeded.current = (memory->boundedbuffer_roll_exceeded.current + 1) % 1;
}
void shift_trigger_6(Memory* memory){
memory->boundedbuffer_trigger_6.current = (memory->boundedbuffer_trigger_6.current + 1) % 1;
}
void shift_yaw_exceeded(Memory* memory){
memory->boundedbuffer_yaw_exceeded.current = (memory->boundedbuffer_yaw_exceeded.current + 1) % 1;
}
void shift_trigger_7(Memory* memory){
memory->boundedbuffer_trigger_7.current = (memory->boundedbuffer_trigger_7.current + 1) % 1;
}
void shift_battery_low(Memory* memory){
memory->boundedbuffer_battery_low.current = (memory->boundedbuffer_battery_low.current + 1) % 1;
}
void shift_trigger_8(Memory* memory){
memory->boundedbuffer_trigger_8.current = (memory->boundedbuffer_trigger_8.current + 1) % 1;
}
bool expr_0(Memory* memory){
return battery_failed_sync(memory);
}
bool expr_1(Memory* memory){
return battery_low_sync(memory);
}
bool expr_2(Memory* memory){
return motor_failed_sync(memory);
}
bool expr_3(Memory* memory){
return x_drift_exceeded_sync(memory);
}
bool expr_4(Memory* memory){
return y_drift_exceeded_sync(memory);
}
bool expr_5(Memory* memory){
return z_drift_exceeded_sync(memory);
}
bool expr_6(Memory* memory){
return pitch_exceeded_sync(memory);
}
bool expr_7(Memory* memory){
return roll_exceeded_sync(memory);
}
bool expr_8(Memory* memory){
return yaw_exceeded_sync(memory);
}Verdict build_verdict(Memory* memory) {
    Verdict verdict;
    memset(&verdict, 0, sizeof(verdict));

    if (abs_x_drift_is_fresh(memory)) {
        verdict.abs_x_drift = abs_x_drift_sync(memory);
        verdict.abs_x_drift_is_present = 1;
    }
    if (abs_y_drift_is_fresh(memory)) {
        verdict.abs_y_drift = abs_y_drift_sync(memory);
        verdict.abs_y_drift_is_present = 1;
    }
    if (abs_z_drift_is_fresh(memory)) {
        verdict.abs_z_drift = abs_z_drift_sync(memory);
        verdict.abs_z_drift_is_present = 1;
    }
    if (abs_pitch_is_fresh(memory)) {
        verdict.abs_pitch = abs_pitch_sync(memory);
        verdict.abs_pitch_is_present = 1;
    }
    if (abs_roll_is_fresh(memory)) {
        verdict.abs_roll = abs_roll_sync(memory);
        verdict.abs_roll_is_present = 1;
    }
    if (abs_yaw_is_fresh(memory)) {
        verdict.abs_yaw = abs_yaw_sync(memory);
        verdict.abs_yaw_is_present = 1;
    }

    if (motor_check_is_fresh(memory)) {
        verdict.motor_check = motor_check_sync(memory);
        verdict.motor_check_is_present = 1;
    }
    if (motor_failed_is_fresh(memory)) {
        verdict.motor_failed = motor_failed_sync(memory);
        verdict.motor_failed_is_present = 1;
    }
    if (trigger_0_is_fresh(memory) && expr_2(memory)) {
        verdict.trigger_0 = trigger_0_sync(memory);
        verdict.trigger_0_is_present = 1;
    }

    if (battery_failed_is_fresh(memory)) {
        verdict.battery_failed = battery_failed_sync(memory);
        verdict.battery_failed_is_present = 1;
    }
    if (trigger_1_is_fresh(memory) && expr_0(memory)) {
        verdict.trigger_1 = trigger_1_sync(memory);
        verdict.trigger_1_is_present = 1;
    }

    if (x_drift_exceeded_is_fresh(memory)) {
        verdict.x_drift_exceeded = x_drift_exceeded_sync(memory);
        verdict.x_drift_exceeded_is_present = 1;
    }
    if (trigger_2_is_fresh(memory) && expr_3(memory)) {
        verdict.trigger_2 = trigger_2_sync(memory);
        verdict.trigger_2_is_present = 1;
    }

    if (y_drift_exceeded_is_fresh(memory)) {
        verdict.y_drift_exceeded = y_drift_exceeded_sync(memory);
        verdict.y_drift_exceeded_is_present = 1;
    }
    if (trigger_3_is_fresh(memory) && expr_4(memory)) {
        verdict.trigger_3 = trigger_3_sync(memory);
        verdict.trigger_3_is_present = 1;
    }

    if (z_drift_exceeded_is_fresh(memory)) {
        verdict.z_drift_exceeded = z_drift_exceeded_sync(memory);
        verdict.z_drift_exceeded_is_present = 1;
    }
    if (trigger_4_is_fresh(memory) && expr_5(memory)) {
        verdict.trigger_4 = trigger_4_sync(memory);
        verdict.trigger_4_is_present = 1;
    }

    if (pitch_exceeded_is_fresh(memory)) {
        verdict.pitch_exceeded = pitch_exceeded_sync(memory);
        verdict.pitch_exceeded_is_present = 1;
    }
    if (trigger_5_is_fresh(memory) && expr_6(memory)) {
        verdict.trigger_5 = trigger_5_sync(memory);
        verdict.trigger_5_is_present = 1;
    }

    if (roll_exceeded_is_fresh(memory)) {
        verdict.roll_exceeded = roll_exceeded_sync(memory);
        verdict.roll_exceeded_is_present = 1;
    }
    if (trigger_6_is_fresh(memory) && expr_7(memory)) {
        verdict.trigger_6 = trigger_6_sync(memory);
        verdict.trigger_6_is_present = 1;
    }

    if (yaw_exceeded_is_fresh(memory)) {
        verdict.yaw_exceeded = yaw_exceeded_sync(memory);
        verdict.yaw_exceeded_is_present = 1;
    }
    if (trigger_7_is_fresh(memory) && expr_8(memory)) {
        verdict.trigger_7 = trigger_7_sync(memory);
        verdict.trigger_7_is_present = 1;
    }

    if (battery_low_is_fresh(memory)) {
        verdict.battery_low = battery_low_sync(memory);
        verdict.battery_low_is_present = 1;
    }
    if (trigger_8_is_fresh(memory) && expr_1(memory)) {
        verdict.trigger_8 = trigger_8_sync(memory);
        verdict.trigger_8_is_present = 1;
    }

    verdict.time = memory->time;
    return verdict;
}
void clear_activation(Memory* memory){
memory->boundedbuffer_motor_pass.is_fresh = 0;
memory->boundedbuffer_battery_pass.is_fresh = 0;
memory->boundedbuffer_x_drift.is_fresh = 0;
memory->boundedbuffer_y_drift.is_fresh = 0;
memory->boundedbuffer_z_drift.is_fresh = 0;
memory->boundedbuffer_pitch.is_fresh = 0;
memory->boundedbuffer_roll.is_fresh = 0;
memory->boundedbuffer_yaw.is_fresh = 0;
memory->boundedbuffer_vbat.is_fresh = 0;
memory->boundedbuffer_abs_x_drift.is_fresh = 0;
memory->boundedbuffer_abs_y_drift.is_fresh = 0;
memory->boundedbuffer_abs_z_drift.is_fresh = 0;
memory->boundedbuffer_abs_pitch.is_fresh = 0;
memory->boundedbuffer_abs_roll.is_fresh = 0;
memory->boundedbuffer_abs_yaw.is_fresh = 0;
memory->boundedbuffer_motor_check.is_fresh = 0;
memory->boundedbuffer_motor_failed.is_fresh = 0;
memory->boundedbuffer_trigger_0.is_fresh = 0;
memory->boundedbuffer_battery_failed.is_fresh = 0;
memory->boundedbuffer_trigger_1.is_fresh = 0;
memory->boundedbuffer_x_drift_exceeded.is_fresh = 0;
memory->boundedbuffer_trigger_2.is_fresh = 0;
memory->boundedbuffer_y_drift_exceeded.is_fresh = 0;
memory->boundedbuffer_trigger_3.is_fresh = 0;
memory->boundedbuffer_z_drift_exceeded.is_fresh = 0;
memory->boundedbuffer_trigger_4.is_fresh = 0;
memory->boundedbuffer_pitch_exceeded.is_fresh = 0;
memory->boundedbuffer_trigger_5.is_fresh = 0;
memory->boundedbuffer_roll_exceeded.is_fresh = 0;
memory->boundedbuffer_trigger_6.is_fresh = 0;
memory->boundedbuffer_yaw_exceeded.is_fresh = 0;
memory->boundedbuffer_trigger_7.is_fresh = 0;
memory->boundedbuffer_battery_low.is_fresh = 0;
memory->boundedbuffer_trigger_8.is_fresh = 0;
}
Verdict cycle(Memory* memory, InternalEvent internalevent){
memory->time = internalevent.time;
if (internalevent.motor_pass_is_present) {
shift_motor_pass(memory);
input_motor_pass(memory, internalevent.motor_pass);
}
if (internalevent.battery_pass_is_present) {
shift_battery_pass(memory);
input_battery_pass(memory, internalevent.battery_pass);
}
if (internalevent.x_drift_is_present) {
shift_x_drift(memory);
input_x_drift(memory, internalevent.x_drift);
}
if (internalevent.y_drift_is_present) {
shift_y_drift(memory);
input_y_drift(memory, internalevent.y_drift);
}
if (internalevent.z_drift_is_present) {
shift_z_drift(memory);
input_z_drift(memory, internalevent.z_drift);
}
if (internalevent.pitch_is_present) {
shift_pitch(memory);
input_pitch(memory, internalevent.pitch);
}
if (internalevent.roll_is_present) {
shift_roll(memory);
input_roll(memory, internalevent.roll);
}
if (internalevent.yaw_is_present) {
shift_yaw(memory);
input_yaw(memory, internalevent.yaw);
}
if (internalevent.vbat_is_present) {
shift_vbat(memory);
input_vbat(memory, internalevent.vbat);
}
if (internalevent.x_drift_is_present) {
shift_abs_x_drift(memory);
}
if (internalevent.y_drift_is_present) {
shift_abs_y_drift(memory);
}
if (internalevent.z_drift_is_present) {
shift_abs_z_drift(memory);
}
if (internalevent.pitch_is_present) {
shift_abs_pitch(memory);
}
if (internalevent.roll_is_present) {
shift_abs_roll(memory);
}
if (internalevent.yaw_is_present) {
shift_abs_yaw(memory);
}
if (internalevent.motor_pass_is_present) {
shift_motor_check(memory);
}
if (internalevent.motor_pass_is_present) {
shift_motor_failed(memory);
}
if (internalevent.battery_pass_is_present) {
shift_battery_failed(memory);
}
if (internalevent.x_drift_is_present) {
shift_x_drift_exceeded(memory);
}
if (internalevent.y_drift_is_present) {
shift_y_drift_exceeded(memory);
}
if (internalevent.z_drift_is_present) {
shift_z_drift_exceeded(memory);
}
if (internalevent.pitch_is_present) {
shift_pitch_exceeded(memory);
}
if (internalevent.roll_is_present) {
shift_roll_exceeded(memory);
}
if (internalevent.yaw_is_present) {
shift_yaw_exceeded(memory);
}
if (internalevent.vbat_is_present) {
shift_battery_low(memory);
}
if (internalevent.x_drift_is_present) {
eval_abs_x_drift_0(memory);
}
if (internalevent.y_drift_is_present) {
eval_abs_y_drift_0(memory);
}
if (internalevent.z_drift_is_present) {
eval_abs_z_drift_0(memory);
}
if (internalevent.pitch_is_present) {
eval_abs_pitch_0(memory);
}
if (internalevent.roll_is_present) {
eval_abs_roll_0(memory);
}
if (internalevent.yaw_is_present) {
eval_abs_yaw_0(memory);
}
if (internalevent.motor_pass_is_present) {
eval_motor_check_0(memory);
}
if (internalevent.battery_pass_is_present) {
eval_battery_failed_0(memory);
}
if (internalevent.vbat_is_present) {
eval_battery_low_0(memory);
}
if (internalevent.motor_pass_is_present) {
eval_motor_failed_0(memory);
}
if ((internalevent.battery_pass_is_present && expr_0(memory))) {
shift_trigger_1(memory);
}
if (internalevent.x_drift_is_present) {
eval_x_drift_exceeded_0(memory);
}
if (internalevent.y_drift_is_present) {
eval_y_drift_exceeded_0(memory);
}
if (internalevent.z_drift_is_present) {
eval_z_drift_exceeded_0(memory);
}
if (internalevent.pitch_is_present) {
eval_pitch_exceeded_0(memory);
}
if (internalevent.roll_is_present) {
eval_roll_exceeded_0(memory);
}
if (internalevent.yaw_is_present) {
eval_yaw_exceeded_0(memory);
}
if ((internalevent.vbat_is_present && expr_1(memory))) {
shift_trigger_8(memory);
}
if ((internalevent.motor_pass_is_present && expr_2(memory))) {
shift_trigger_0(memory);
}
if ((internalevent.battery_pass_is_present && expr_0(memory))) {
eval_trigger_1_0(memory);
}
if ((internalevent.x_drift_is_present && expr_3(memory))) {
shift_trigger_2(memory);
}
if ((internalevent.y_drift_is_present && expr_4(memory))) {
shift_trigger_3(memory);
}
if ((internalevent.z_drift_is_present && expr_5(memory))) {
shift_trigger_4(memory);
}
if ((internalevent.pitch_is_present && expr_6(memory))) {
shift_trigger_5(memory);
}
if ((internalevent.roll_is_present && expr_7(memory))) {
shift_trigger_6(memory);
}
if ((internalevent.yaw_is_present && expr_8(memory))) {
shift_trigger_7(memory);
}
if ((internalevent.vbat_is_present && expr_1(memory))) {
eval_trigger_8_0(memory);
}
if ((internalevent.motor_pass_is_present && expr_2(memory))) {
eval_trigger_0_0(memory);
}
if ((internalevent.x_drift_is_present && expr_3(memory))) {
eval_trigger_2_0(memory);
}
if ((internalevent.y_drift_is_present && expr_4(memory))) {
eval_trigger_3_0(memory);
}
if ((internalevent.z_drift_is_present && expr_5(memory))) {
eval_trigger_4_0(memory);
}
if ((internalevent.pitch_is_present && expr_6(memory))) {
eval_trigger_5_0(memory);
}
if ((internalevent.roll_is_present && expr_7(memory))) {
eval_trigger_6_0(memory);
}
if ((internalevent.yaw_is_present && expr_8(memory))) {
eval_trigger_7_0(memory);
}
Verdict verdict = build_verdict(memory);
clear_activation(memory);
return verdict;
}