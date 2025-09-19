#ifndef LM_LEXER_H
#define LM_LEXER_H

/**
 * include/lm_lexer.h
 * 
 * Defining the structures, enums and foward decls for the lexer step of Lambda.
 * Written entirely with C99 and only the stdlib. Comments must follow C89.
 * 
 * Contributions should respect the 80col limit! That is the hard limit for
 * lines of code and comments, anything longer should be wrapped or rewritten.
 * 
 * (c) 2025 samdoesnerdstuff - Licensed under BSD-3-Clause
 */

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <stddef.h>

#define LM_STRING_MAX 2048 /* Limits individual strings to 2kb */

/* ---------------------- Token Types ---------------------- */

typedef enum {
    LMTK_UNKNOWN,       /* Used in errors when lexer encounters unknown data */
    LMTK_EOF,           /* End of File token - all srcs should end with it */
    LMTK_IDENTIFIER,    /* Identifier token - var names, func names, etc. */
    LMTK_INTEGER,       /* Number token - integer and floating-point numbers */
    LMTK_FLOAT,         /* Floating-point number token */
    LMTK_STRING,        /* Literal string */
    LMTK_ARITH_ADD,     /* Arithmetic '+' */
    LMTK_ARITH_SUB,     /* Arithmetic '-' */
    LMTK_ARITH_MUL,     /* Arithmetic '*' */
    LMTK_ARITH_DIV,     /* Arithmetic '/' */
    LMTK_ARITH_MOD,     /* Arithmetic '%' */
    LMTK_ARITH_POW,     /* Arithmetic '^' */
    LMTK_ASSIGN,        /* Arithmetic '=' */
    LMTK_FUNC_DESIG,    /* Function designation 'func' */
    LMTK_LOGICAL_AND,   /* Logical AND */
    LMTK_LOGICAL_OR,    /* Logical OR */
    LMTK_LOGICAL_NOT,   /* Logical NOT */
    LMTK_COMP_EQ,       /* Comparison EQ '==' */
    LMTK_COMP_NEQ,      /* Comparison NEQ '!=' */
    LMTK_COMP_LT,       /* Comparison LT '<' */
    LMTK_COMP_LTE,      /* Comparison LTE '<=' */
    LMTK_COMP_GT,       /* Comparison GT '>' */
    LMTK_COMP_GTE,      /* Comparison GTE '>=' */

    /* TODO: Implement more tokens on an "as-needed" basis! */
} lm_token_type_t;

/* ---------------------- Token struct ---------------------- */

typedef struct {
    lm_token_type_t type;           /* derived from `lm_token_type_t` */
    char str[LM_STRING_MAX];        /* The string repr of the token */
    union {
        int64_t integer;            /* Integer value (if applicable) */
        double floating;            /* Floating-point value (if applicable) */
    } value;                        /* The value of the token (if applicable) */
    size_t line;                    /* The line # where token was found */
    size_t column;                  /* The column # where the token starts */
} lm_token_t;

/* ---------------------- Lexer state ---------------------- */

typedef struct lm_lexer lm_lexer_t;

struct lm_lexer {
    FILE *src;              /* The source input stream */
    int current;            /* The current character being processed */
    size_t line;            /* Current line number */
    size_t column;          /* Current column number */
    bool has_peeked;        /* Whether we've peeked at the next token */
    lm_token_t peeked_tok;  /* The peeked token (if any) */
};

/* ---------------------- API Functions ---------------------- */

/* Create a lexer from a `FILE*` stream.
Lexer doesn't take ownership of the stream. */
lm_lexer_t *lm_lexer_new(FILE *src);

/* Destroy a lexer and free its used resources. */
void lm_lexer_free(lm_lexer_t *lexer);

/* Get next token from lexer. Returns fully populated `lm_token_t` */
lm_token_t lm_lexer_next(lm_lexer_t *lexer);

/* 'Peek' at the following token. Helpful for lookaheads. */
lm_token_t lm_lexer_peek(lm_lexer_t *lexer);

/* Reset the lexer back to start of the stream. */
void lm_lexer_reset(lm_lexer_t *lexer);

/* Returns the `lm_token_type_t` as a `char` array. */
const char *lm_token_type_str(lm_token_type_t type);

/* Prints out a token in a human-readable format. */
void lm_token_print(lm_token_t *tok, FILE *out);

#endif /* lm_lexer.h */