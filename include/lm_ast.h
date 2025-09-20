#ifndef LM_AST_H
#define LM_AST_H

/**
 * include/lm_ast.h
 * 
 * Header for the AST generation step, one of the last steps of interpreting.
 * Currently unfinished...
 * 
 * (c) 2025 samdoesnerdstuff - Licensed under BSD-3-Clause
 */

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

typedef enum { } lm_ast_node_type_t;

typedef struct lm_ast_node lm_ast_node_t;

struct lm_ast_node {
    lm_ast_node_type_t type;
    size_t line, column;

    union { /* assignment, funcdef, binary_expr, etc. */ };
};

/* Returns contents of the AST node. */
void lm_ast_print(lm_ast_node_t *node, int indent, FILE *out);

/* Removes AST node, frees memory. */
void lm_ast_free(lm_ast_node_t *node);

#endif