#ifndef __STATE_H
#define __STATE_H

#include <stdbool.h> // For `bool` type
#include <time.h>
// Might need to include any other encoding or custom data that this part
// requires.
#include <stdlib.h> // For `NULL`

#include <logic.h>

typedef struct {
  time_t time;
} Timestamp;

// by convention if end_t is before start_t then it's considered infinite, i.e.
// a halfopen interval
typedef struct {
  Timestamp start_t;
  Timestamp end_t;
  bool has_end;
} TrueWhen;

// Defines or function announces.
Timestamp Timestamp_new(
    Timestamp *possibleDate); // Simulating an option, passing NULL as `None`
TrueWhen TrueWelcome_new(
    Timestamp start_t,
    Timestamp end_t); // Assuming every Timestamp comes in a defined setting
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