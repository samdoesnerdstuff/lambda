#include "lm_lexer.h"
#include "lm_errors.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

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

/* Advances the lexer by a column or line. */
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

/* Returns the infrmation on the current character. */
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
    lm_lexer_t *lexer = (lm_lexer_t *)malloc(sizeof(lm_lexer_t));
    if (!lexer) return NULL;

    lexer->src = src;
    lexer->line = 1;
    lexer->column = 0;
    lexer->has_peeked = 0;
    lexer->peeked_tok.type = LMTK_EOF;

    lexer->current = lm_lexer_advance(lexer);
    return lexer;
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

    /* Literal strings */
    if (ch == '"') {
        lm_lexer_advance(lexer);
        size_t len = 0;
        while ((ch = lm_lexer_peek_char(lexer)) != '"' && ch != EOF) {
            if (len < LM_STRING_MAX - 1) {
                tok.str[len++] = (char)ch;
            }
            lm_lexer_advance(lexer);
        }
        tok.str[len] = '\0';
        /* skip closing quote */
        lm_lexer_advance(lexer);
        tok.type = LMTK_STRING;
        return tok;
    }

    /* Builtin math/arithmetic operators */
    switch (ch) {
        case '+': tok.type = LMTK_ARITH_ADD; break;
        case '-': tok.type = LMTK_ARITH_SUB; break;
        case '*': tok.type = LMTK_ARITH_MUL; break;
        case '/': tok.type = LMTK_ARITH_DIV; break;
        case '%': tok.type = LMTK_ARITH_MOD; break;
        case '^': tok.type = LMTK_ARITH_POW; break;
        case '=': tok.type = LMTK_ASSIGN; break;
        default:
            tok.type = LMTK_UNKNOWN;
            fprintf(stderr, "Error %i @ %zu:%zu: Unknown Character '%c'\n", 
                LM_LEX_UNKNOWN_OPERAND, lexer->line, lexer->column, (char)ch);
            exit(EXIT_FAILURE);
    }
    lm_lexer_advance(lexer);
    return tok;
}

lm_token_t lm_lexer_peek(lm_lexer_t *lexer) {
    if (!lexer->has_peeked) {
        lexer->peeked_tok = lm_lexer_next(lexer);
        lexer->has_peeked = true;
    }
    return lexer->peeked_tok;
}

void lm_lexer_reset(lm_lexer_t *lexer) {
    if (!lexer || !lexer->src) return;
    fseek(lexer->src, 0, SEEK_SET);
    lexer->line = 1;
    lexer->column = 0;
    lexer->has_peeked = 0;
    lexer->peeked_tok.type = LMTK_EOF;
    lexer->current = lm_lexer_advance(lexer);
}

const char *lm_token_type_str(lm_token_type_t type) {
    switch (type) {
        case LMTK_EOF:          return "EOF";
        case LMTK_IDENTIFIER:   return "IDENTIFIER";
        case LMTK_FUNC_DESIG:   return "FUNC_DESIG";
        case LMTK_INTEGER:      return "INTEGER";
        case LMTK_FLOAT:        return "FLOAT";
        case LMTK_STRING:       return "STRING";
        case LMTK_ARITH_ADD:      return "ADD";
        case LMTK_ARITH_SUB:      return "SUB";
        case LMTK_ARITH_MUL:      return "MUL";
        case LMTK_ARITH_DIV:      return "DIV";
        case LMTK_ARITH_MOD:      return "MOD";
        case LMTK_ARITH_POW:      return "POW";
        case LMTK_LOGICAL_AND:  return "LOGICAL_AND";        
        case LMTK_LOGICAL_OR:   return "LOGICAL_OR";
        case LMTK_LOGICAL_NOT:  return "LOGICAL_NOT";
        case LMTK_ASSIGN:       return "ASSIGN";
        case LMTK_UNKNOWN:      return "UNKNOWN";
        default:                return "???";
    }
}

void lm_token_print(lm_token_t *tok, FILE *out) {
    fprintf(out, "[%d:%d] %s",
        tok->line, tok->column, lm_token_type_str(tok->type));

    if (tok->str[0] != '\0') {
        fprintf(out, " (%s)", tok->str);
    }
    fprintf(out, "\n");
}
