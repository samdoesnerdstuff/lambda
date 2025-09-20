#ifndef LM_PARSER_H
#define LM_PARSER_H

/**
 * include/lm_parser.h
 * 
 * Header for the parser, where tokens come to get processed for the AST.
 * 
 * (c) 2025 samdoesnerdstuff - Licensed under BSD-3-Clause
 */

#include "lm_lexer.h"
#include "lm_ast.h"

typedef struct {
    lm_lexer_t *lexer;
    lm_token_t current;
} lm_parser_t;

/* Creates a new parser. */
lm_parser_t *lm_parser_new(lm_lexer_t *lexer);

/* Deletes the `*parser` passed to it and frees resources. */
void lm_parser_free(lm_parser_t *parser);

#endif /* include/lm_parser.h */