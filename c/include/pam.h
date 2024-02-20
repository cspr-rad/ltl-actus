#ifndef __PAM_H
#define __PAM_H

#include <stddef.h> // For size_t

typedef size_t InterestRate;
typedef size_t Money;
typedef size_t NumMonths;

typedef struct {
  Money principal;
  InterestRate interest_rate;
  NumMonths months;
} PamTerms;

typedef enum {
  Maturity,
  PrincipalRepayment,
  InterestPayment,
} PamEvent;

typedef struct {
  Money total_repayment;
} PamState;

typedef enum {
  Terms,
  Event,
  State,
} PamType;

typedef struct {
  PamType type;
  union {
    PamTerms terms;
    PamEvent event;
    PamState state;
  } data;
} Pam;

#endif // __PAM_H