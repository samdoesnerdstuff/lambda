#include "lm_lexer.h"

/**
 * src/lm_lexer.c
 * 
 * Proper implementation of the lexer, the first step in execution.
 * This lexer takes in the source, breaks it down to tokens and passes
 * it along to the parser.
 * 
 * Contributions should respect the 80col limit! That is the hard limit for
 * lines of code and comments, anything longer should be wrapped or rewritten.
 * 
 * (c) 2025 samdoesnerdstuff - Licensed under BSD-3-Clause
 */

/* ---------------------- Internal API Functions ---------------------- */
static int lm_lexer_advance(lm_lexer_t *lx) {
    int c = getc(lx->src);
    lx->current = c;

    if (c == '\n') {
        lx->line++;
        lx->column = 0;
    } else {
        lx->column++;
    }

    return c;
}

static int lm_lexer_peek_char(lm_lexer_t *lx) {
    return lx->current;
}

/* TODO: return to implement comments */
static int lm_lexer_skip_ws(lm_lexer_t *lx) {
    while (lx->current == ' ' || lx->current == '\t' || 
        lx->current == '\n' || lx->current == '\r') {
        lm_lexer_advance(lx);
    }
}

/* ---------------------- End Internal API Functions ---------------------- */

/* ---------------------- Actual API Functions ---------------------- */
lm_lexer_t *lm_lexer_new(FILE *src) {

}

void lm_lexer_free(lm_lexer_t *lexer) {
    if (lexer) free(lexer);
}

lm_token_t lm_lexer_next(lm_lexer_t *lexer) {
    if (lexer->has_peeked) {
        lexer->has_peeked = false;
        return lexer->peeked_tok;
    }

    lm_token_t tok = {0};
    tok.line = lexer->line;
    tok.column = lexer->column;
    lm_lexer_skip_ws(lexer);

    int ch = lm_lexer_peek_char(lexer);
    if (ch == EOF) {
        tok.type = LMTK_EOF;
        return tok;
    }

    /* Identifiers / Keywords */
    if (isalpha(ch) || ch == '_') {
        size_t len = 0;
        while (isalnum(ch) || ch == '_') {
            if (len < LM_STRING_MAX - 1) {
                tok.str[len++] = (char)ch;
            }
            ch = lm_lexer_advance(lexer);
        }
        tok.str[len] = '\0';

        if (strcmp(tok.str, "func") == 0) {
            tok.type = LMTK_FUNC_DESIG;
        } else {
            tok.type = LMTK_IDENTIFIER;
        }

        return tok;
    }

    /* Numbers and stuff (floats) */
    if (isdigit(ch)) {
        size_t len = 0;
        bool is_float = false;

        while (isdigit(ch) || ch == '.') {
            if (ch == '.') {
                /* Stop parsing if we get here */
                if (is_float) break;
                is_float = true;
            }
            if (len < LM_STRING_MAX - 1) {
                tok.str[len++] = (char)ch;
            }
            ch = lm_lexer_advance(lexer);
        }
        tok.str[len] = '\0';

        if (is_float) {
            tok.type = LMTK_FLOAT;
            tok.value.floating = strtod(tok.str, NULL);
        } else {
            tok.type = LMTK_INTEGER;
            tok.value.integer = strtoll(tok.str, NULL, 10);
        }

        return tok;
    }
}

lm_token_t lm_lexer_peek(lm_lexer_t *lexer) {

}

void lm_lexer_reset(lm_lexer_t *lexer) {

}

const char *lm_token_type_str(lm_token_type_t type) {

}

void lm_token_print(lm_token_t *tok, FILE *out) {

}