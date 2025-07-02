#include "Monitor.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
char* STR_0 = "X-Drift! Fix immediately!";
char* STR_1 = "Y-Drift! Fix immediately!";
void reset_fresh(Memory *memory) {
memory->stream_memory.x_drift.is_fresh = false;
memory->stream_memory.y_drift.is_fresh = false;
memory->stream_memory.trigger_0.is_fresh = false;
memory->stream_memory.trigger_1.is_fresh = false;
}
double *memory_get_x_drift(Memory *memory, unsigned int offset) {
	Memory_x_drift *buffer = &memory->stream_memory.x_drift;
	unsigned int index = (buffer->current + 1 - offset) % 1;
	if (buffer->valid[index]) {
		return &buffer->values[index];
	} else {
		return NULL;
	}
}

void memory_shift_x_drift(Memory *memory) {
	Memory_x_drift *buffer = &memory->stream_memory.x_drift;
	buffer->current += 1;
	if (buffer->current == 1) {
		buffer->current = 0;
	}
}

void memory_update_x_drift(Memory *memory, double new_value) {
	Memory_x_drift *buffer = &memory->stream_memory.x_drift;
	buffer->values[buffer->current] = new_value;
	buffer->valid[buffer->current] = true;
	buffer->is_fresh = true;
}
double *memory_get_y_drift(Memory *memory, unsigned int offset) {
	Memory_y_drift *buffer = &memory->stream_memory.y_drift;
	unsigned int index = (buffer->current + 1 - offset) % 1;
	if (buffer->valid[index]) {
		return &buffer->values[index];
	} else {
		return NULL;
	}
}

void memory_shift_y_drift(Memory *memory) {
	Memory_y_drift *buffer = &memory->stream_memory.y_drift;
	buffer->current += 1;
	if (buffer->current == 1) {
		buffer->current = 0;
	}
}

void memory_update_y_drift(Memory *memory, double new_value) {
	Memory_y_drift *buffer = &memory->stream_memory.y_drift;
	buffer->values[buffer->current] = new_value;
	buffer->valid[buffer->current] = true;
	buffer->is_fresh = true;
}
char* *memory_get_trigger_0(Memory *memory, unsigned int offset) {
	Memory_trigger_0 *buffer = &memory->stream_memory.trigger_0;
	unsigned int index = (buffer->current + 1 - offset) % 1;
	if (buffer->valid[index]) {
		return &buffer->values[index];
	} else {
		return NULL;
	}
}

void memory_shift_trigger_0(Memory *memory) {
	Memory_trigger_0 *buffer = &memory->stream_memory.trigger_0;
	buffer->current += 1;
	if (buffer->current == 1) {
		buffer->current = 0;
	}
}

void memory_update_trigger_0(Memory *memory, char* new_value) {
	Memory_trigger_0 *buffer = &memory->stream_memory.trigger_0;
	buffer->values[buffer->current] = new_value;
	buffer->valid[buffer->current] = true;
	buffer->is_fresh = true;
}
char* *memory_get_trigger_1(Memory *memory, unsigned int offset) {
	Memory_trigger_1 *buffer = &memory->stream_memory.trigger_1;
	unsigned int index = (buffer->current + 1 - offset) % 1;
	if (buffer->valid[index]) {
		return &buffer->values[index];
	} else {
		return NULL;
	}
}

void memory_shift_trigger_1(Memory *memory) {
	Memory_trigger_1 *buffer = &memory->stream_memory.trigger_1;
	buffer->current += 1;
	if (buffer->current == 1) {
		buffer->current = 0;
	}
}

void memory_update_trigger_1(Memory *memory, char* new_value) {
	Memory_trigger_1 *buffer = &memory->stream_memory.trigger_1;
	buffer->values[buffer->current] = new_value;
	buffer->valid[buffer->current] = true;
	buffer->is_fresh = true;
}
// RtLola expression: x_drift
double sync_x_drift(Memory *memory) {
double *value = memory_get_x_drift(memory, 0);
// assert(value != NULL);
return *value;
}
// RtLola expression: y_drift
double sync_y_drift(Memory *memory) {
double *value = memory_get_y_drift(memory, 0);
// assert(value != NULL);
return *value;
}
// RtLola expression: trigger_0
char* sync_trigger_0(Memory *memory) {
char* *value = memory_get_trigger_0(memory, 0);
// assert(value != NULL);
return *value;
}
// RtLola expression: trigger_1
char* sync_trigger_1(Memory *memory) {
char* *value = memory_get_trigger_1(memory, 0);
// assert(value != NULL);
return *value;
}
bool is_fresh_trigger_0(Memory *memory) {
return memory->stream_memory.trigger_0.is_fresh;
}
bool is_fresh_trigger_1(Memory *memory) {
return memory->stream_memory.trigger_1.is_fresh;
}
// RtLola definition: input x_drift : Float64
void eval_x_drift(Memory *memory, double new_value) {
memory_update_x_drift(memory, new_value);
}
// RtLola definition: input y_drift : Float64
void eval_y_drift(Memory *memory, double new_value) {
memory_update_y_drift(memory, new_value);
}
void eval_trigger_0_0(Memory *memory) {
char* new_value = STR_0;
memory_update_trigger_0(memory, new_value);
}
void eval_trigger_1_0(Memory *memory) {
char* new_value = STR_1;
memory_update_trigger_1(memory, new_value);
}
Verdict new_verdict(Memory *memory, double time) {
Verdict v;
v.has_trigger_0 = is_fresh_trigger_0(memory);
if (v.has_trigger_0) {
v.trigger_0 = sync_trigger_0(memory);
}
v.has_trigger_1 = is_fresh_trigger_1(memory);
if (v.has_trigger_1) {
v.trigger_1 = sync_trigger_1(memory);
}
v.time = time;
return v;
}
void display_verdict(Verdict *v) {
if (v->has_trigger_0) { printf("Triggered (RTLola - X): %s, ", v->trigger_0); } else { printf("Not Triggered (X - Trigger 0), "); }
if (v->has_trigger_1) { printf("Triggered (RTLola - Y): %s, ", v->trigger_1); } else { printf("Not Triggered (Y - Trigger 1), "); }
printf("Timestamp (RTLola): %f\n", v->time);
}
Verdict cycle(Memory *memory, InternalEvent *event) {
if (event->has_x_drift) {
memory_shift_x_drift(memory);
}
if (event->has_x_drift) {
eval_x_drift(memory, event->x_drift);
}
if (event->has_y_drift) {
memory_shift_y_drift(memory);
}
if (event->has_y_drift) {
eval_y_drift(memory, event->y_drift);
}
if (event->has_x_drift && (sync_x_drift(memory)>0.05||sync_x_drift(memory)<(-0.05))) {
memory_shift_trigger_0(memory);
}
if (event->has_y_drift && (sync_y_drift(memory)>0.05||sync_y_drift(memory)<(-0.05))) {
memory_shift_trigger_1(memory);
}
if (event->has_x_drift && (sync_x_drift(memory)>0.05||sync_x_drift(memory)<(-0.05))) {
eval_trigger_0_0(memory);
}
if (event->has_y_drift && (sync_y_drift(memory)>0.05||sync_y_drift(memory)<(-0.05))) {
eval_trigger_1_0(memory);
}
Verdict v = new_verdict(memory, event->time); reset_fresh(memory);;return v;}
void init_stream_memory(StreamMemory *m) {memset(&m, 0, sizeof(m));}
void memory_init(Memory *m, double start_time) {
memset(m, 0, sizeof(*m));init_stream_memory(&m->stream_memory);
start_time;;
}
Verdict accept_event(Memory *memory, RTLola_Event event, double time) {
printf("RTLola X: %f, RTLola Has X: %d, RTLola Y: %f, RTLola Has Y: %d\n", event.x_drift, event.has_x_drift, event.y_drift, event.has_y_drift);
InternalEvent ievent = { .x_drift=event.x_drift, .has_x_drift=event.has_x_drift, .y_drift=event.y_drift, .has_y_drift=event.has_y_drift, .time=time };
return cycle(memory, &ievent);
}
// https://stackoverflow.com/a/12911465
char* getfield(char* line, int num){
    char* tok;
    for (tok = strtok(line, ",");
            tok && *tok;
            tok = strtok(NULL, ","))
    {
        if (!--num){
            if (!strcmp(tok, "#")) {
                return NULL;
            }
            return tok;
        }
    }
    return NULL;
}

int read_row(FILE* f, RTLola_Event *event, double *time){
    char line[1024];

    memset(event, 0, sizeof(*event));

    char* field;
    char* tmp;
    if (fgets(line, 1024, f)){

        tmp = strdup(line);
        field = getfield(tmp, 1);
        if (field != NULL) {
            event->has_x_drift = 1;
            event->x_drift = atof(field);
        }

        tmp = strdup(line);
        field = getfield(tmp, 2);
        if (field != NULL) {
            event->has_y_drift = 1;
            event->y_drift = atof(field);
        }
        tmp = strdup(line);
        field = getfield(tmp, 3);
        if (field != NULL) {
            *time = atof(field);
        } else {
            fprintf(stderr, "csv did not include time");
            exit(1);
        }
        return 1;
    } else {
        return 0;
    }
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "Give csv file as first and only argument.\n");
        return 1;
    }
    FILE *f = fopen(argv[1], "r");

    RTLola_Event e;
    Memory m;
    double time;
    int num_results;
    read_row(f, &e, &time); // drop header
    if (read_row(f, &e, &time)) {
        memory_init(&m, time);
        Verdict v = accept_event(&m, e, time);
        printf("trigger_0,trigger_1,time\n");
        display_verdict(&v);
    } else {
        return 1;
    }

    while(read_row(f, &e, &time)) {
        Verdict v = accept_event(&m, e, time);
        display_verdict(&v);
    }

    return 0;
}