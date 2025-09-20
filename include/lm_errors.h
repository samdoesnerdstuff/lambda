#ifndef LM_ERRORS_H
#define LM_ERRORS_H

/**
 * include/lm_errors.h
 * 
 * This just holds error codes, their descriptions and other relevant info.
 * Error *handling* is done in another file. Also all of these are handy
 * #macros because those are cool.
 * 
 * (c) 2025 samdoesnerdstuff - Licensed under BSD-3-Clause
 */

/* ---------------------- Lexer Errors -----------------------  */
/* The most basic of errors that are found on the initial pass. */

#define LM_LEX_UNKNOWN_OPERAND      0x0001
#define LM_LEX_STRING_UNTERMINATED  0x0002
#define LM_LEX_STRING_TOO_LONG      0x0003

/* ------------------------ Parsing Errors ----------------------- */
/* Clear cut, if stuff parses wrong then things start to explode.  */

/* --------------------- AST Errors --------------------- */
/* Errors that happen before or during AST generation.    */
/* Possibly also afterwards, we'll see...                 */

/* --------------------- Runtime Errors --------------------- */
/* Things that only really happen at runtime.                 */

/* ----------------------------- Other Errors ----------------------------- */
/* Errors that exist but are hard to categorize for one reason or another.  */

#endif /* include/lm_errors.h */