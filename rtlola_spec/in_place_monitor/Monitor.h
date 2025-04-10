#ifndef MONITOR_H
        #define MONITOR_H
#include "Monitor.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct { bool valid[1]; double values[1]; int current; bool is_fresh; } Memory_x_drift;
typedef struct { bool valid[1]; double values[1]; int current; bool is_fresh; } Memory_y_drift;
typedef struct { bool valid[1]; char* values[1]; int current; bool is_fresh; } Memory_trigger_0;
typedef struct { bool valid[1]; char* values[1]; int current; bool is_fresh; } Memory_trigger_1;
typedef struct {
Memory_x_drift x_drift;
Memory_y_drift y_drift;
Memory_trigger_0 trigger_0;
Memory_trigger_1 trigger_1;
} StreamMemory;
typedef struct {
StreamMemory stream_memory;
double time;
} Memory;
void init_stream_memory(StreamMemory *m);
void memory_init(Memory *m, double start_time);
typedef struct {
bool has_x_drift;
double x_drift;
bool has_y_drift;
double y_drift;
} RTLola_Event;
typedef struct {
bool has_x_drift;
double x_drift;
bool has_y_drift;
double y_drift;

double time;} InternalEvent;
typedef struct {
bool has_trigger_0;
char* trigger_0;
bool has_trigger_1;
char* trigger_1;
double time;
} Verdict;
Verdict accept_event(Memory *memory, RTLola_Event event, double time);
#endif
