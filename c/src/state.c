#include "state.h"
#include <time.h>

Timestamp Timestamp_new(Timestamp *possibleDate) {
  Timestamp result;
  if (possibleDate != NULL) {
    result.time = possibleDate->time;
  } else {
    result.time = time(NULL);
  }
  return result;
}

TrueWhen TrueWhen_new(Timestamp start_t, Timestamp *end_t) {
  TrueWhen result;
  if (end_t != NULL) {
    result.end_t = malloc(sizeof(Timestamp));
    *result.end_t = *end_t;
  } else {
    result.end_t = NULL;
  }
  result.start_t = start_t;
  return result;
}
