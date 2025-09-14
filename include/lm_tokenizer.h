#include <string.h>
#include <ctype.h>
#include <stdio.h>

/**
 * Tokenizer, takes in the source code and parses it into tokens for
 * later parsing and execution.
 */

typedef enum {
    TK_EOF,
    TK_IDENTIFIER,
    TK_NUMBER,
    TK_STRING,
    TK_KEYWORD,
    TK_OPERATOR,
    TK_SYMBOL,
    TK_UNKNOWN
} lm_token_type;

typedef struct {
    lm_token_type type;
    const char *start;
    int length;
    int line;
} lm_token;

typedef struct {
    const char *start;
    const char *current;
    int line;
} lm_tokenizer;

void lm_tk_init(lm_tokenizer *tokenizer, const char *source);
lm_token lm_tk_next(lm_tokenizer *tokenizer);
