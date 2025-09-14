#include "lm_tokenizer.h"

void lm_tk_tokenizer(lm_tokenizer *tokenizer, const char *source) {
    tokenizer->start = source;
    tokenizer->current = source;
    tokenizer->line = 1;
}