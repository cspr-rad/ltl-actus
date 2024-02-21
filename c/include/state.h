#ifndef __STATE_H
#define __STATE_H

#include <stdbool.h>
#include <stdlib.h>
#include <time.h>

#include "logic.h"

typedef struct {
  time_t time;
} Timestamp;

typedef struct {
  Timestamp start_t;
  Timestamp *end_t;
} TrueWhen;

Timestamp Timestamp_new(
    Timestamp *possibleDate); // Simulating an option, passing NULL as `None`
TrueWhen TrueWhen_new(Timestamp start_t, Timestamp *end_t);
bool TrueWhen_contains(const TrueWhen *self, const Timestamp *timestamp);

typedef struct {
  Prop key;
  TrueWhen *value;
} StateStoreEntry;

typedef struct {
  StateStoreEntry *entries;
  int len;
} StateStore;

#endif // __STATE_H
