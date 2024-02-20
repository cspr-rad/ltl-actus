#ifndef __LOGIC_H
#define __LOGIC_H

#include "pam.h"    // Make sure to include the path to the definition of Pam
#include <stdlib.h> // For dynamic memory mgmt

typedef enum {
  VAR,
  EQ,
  NOT,
  OR,
} PropType;

typedef enum {
  TERM,
  AND,
  ALWAYS,
  EVENTUALLY,
  RELEASE,
  UNTIL,
} TemporalType;

typedef struct PropStruct Prop;
typedef struct TemporalPropStruct TemporalProp;

struct PropStruct {
  PropType type;
  union {
    Pam var; // VAR
    struct {
      Pam left;
      Pam right;
    } eq;        // EQ
    Prop * not ; // NOT
    struct {
      Prop *left;
      Prop *right;
    } or ; // OR
  } data;
};

struct TemporalPropStruct {
  TemporalType type;
  union {
    Prop *term; // TERM
    struct {
      TemporalProp *left;
      TemporalProp *right;
    } and;                    // AND
    TemporalProp *always;     // ALWAYS
    TemporalProp *eventually; // EVENTUALLY
    struct {
      TemporalProp *left;
      TemporalProp *right;
    } release; // RELEASE
    struct {
      TemporalProp *left;
      TemporalProp *right;
    } until; // UNTIL
  } data;
};

#endif // __LOGIC_H
