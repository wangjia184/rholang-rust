/* A Bison parser, made by GNU Bison 2.3.  */

/* Skeleton implementation for Bison's Yacc-like parsers in C

   Copyright (C) 1984, 1989, 1990, 2000, 2001, 2002, 2003, 2004, 2005, 2006
   Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor,
   Boston, MA 02110-1301, USA.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Identify Bison output.  */
#define YYBISON 1

/* Bison version.  */
#define YYBISON_VERSION "2.3"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 0

/* Using locations.  */
#define YYLSP_NEEDED 1

/* Substitute the variable and function names.  */
#define yyparse rholang_mercury_parse
#define yylex   rholang_mercury_lex
#define yyerror rholang_mercury_error
#define yylval  rholang_mercury_lval
#define yychar  rholang_mercury_char
#define yydebug rholang_mercury_debug
#define yynerrs rholang_mercury_nerrs
#define yylloc rholang_mercury_lloc

/* Tokens.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
   /* Put the tokens into the symbol table, so that GDB and other debuggers
      know about them.  */
   enum yytokentype {
     _ERROR_ = 258,
     _SYMB_32 = 259,
     _SYMB_33 = 260,
     _SYMB_21 = 261,
     _SYMB_11 = 262,
     _SYMB_12 = 263,
     _SYMB_7 = 264,
     _SYMB_8 = 265,
     _SYMB_5 = 266,
     _SYMB_13 = 267,
     _SYMB_14 = 268,
     _SYMB_24 = 269,
     _SYMB_38 = 270,
     _SYMB_9 = 271,
     _SYMB_15 = 272,
     _SYMB_6 = 273,
     _SYMB_39 = 274,
     _SYMB_10 = 275,
     _SYMB_3 = 276,
     _SYMB_37 = 277,
     _SYMB_30 = 278,
     _SYMB_16 = 279,
     _SYMB_29 = 280,
     _SYMB_31 = 281,
     _SYMB_17 = 282,
     _SYMB_22 = 283,
     _SYMB_20 = 284,
     _SYMB_34 = 285,
     _SYMB_18 = 286,
     _SYMB_19 = 287,
     _SYMB_26 = 288,
     _SYMB_40 = 289,
     _SYMB_41 = 290,
     _SYMB_42 = 291,
     _SYMB_43 = 292,
     _SYMB_44 = 293,
     _SYMB_45 = 294,
     _SYMB_46 = 295,
     _SYMB_35 = 296,
     _SYMB_4 = 297,
     _SYMB_36 = 298,
     _SYMB_25 = 299,
     _SYMB_47 = 300,
     _SYMB_48 = 301,
     _SYMB_27 = 302,
     _SYMB_28 = 303,
     _SYMB_49 = 304,
     _SYMB_50 = 305,
     _SYMB_51 = 306,
     _SYMB_52 = 307,
     _SYMB_53 = 308,
     _SYMB_54 = 309,
     _SYMB_55 = 310,
     _SYMB_56 = 311,
     _SYMB_57 = 312,
     _SYMB_58 = 313,
     _SYMB_59 = 314,
     _SYMB_60 = 315,
     _SYMB_61 = 316,
     _SYMB_62 = 317,
     _SYMB_0 = 318,
     _SYMB_23 = 319,
     _SYMB_1 = 320,
     _SYMB_2 = 321,
     _SYMB_63 = 322,
     _SYMB_64 = 323,
     _SYMB_65 = 324,
     _SYMB_66 = 325
   };
#endif
/* Tokens.  */
#define _ERROR_ 258
#define _SYMB_32 259
#define _SYMB_33 260
#define _SYMB_21 261
#define _SYMB_11 262
#define _SYMB_12 263
#define _SYMB_7 264
#define _SYMB_8 265
#define _SYMB_5 266
#define _SYMB_13 267
#define _SYMB_14 268
#define _SYMB_24 269
#define _SYMB_38 270
#define _SYMB_9 271
#define _SYMB_15 272
#define _SYMB_6 273
#define _SYMB_39 274
#define _SYMB_10 275
#define _SYMB_3 276
#define _SYMB_37 277
#define _SYMB_30 278
#define _SYMB_16 279
#define _SYMB_29 280
#define _SYMB_31 281
#define _SYMB_17 282
#define _SYMB_22 283
#define _SYMB_20 284
#define _SYMB_34 285
#define _SYMB_18 286
#define _SYMB_19 287
#define _SYMB_26 288
#define _SYMB_40 289
#define _SYMB_41 290
#define _SYMB_42 291
#define _SYMB_43 292
#define _SYMB_44 293
#define _SYMB_45 294
#define _SYMB_46 295
#define _SYMB_35 296
#define _SYMB_4 297
#define _SYMB_36 298
#define _SYMB_25 299
#define _SYMB_47 300
#define _SYMB_48 301
#define _SYMB_27 302
#define _SYMB_28 303
#define _SYMB_49 304
#define _SYMB_50 305
#define _SYMB_51 306
#define _SYMB_52 307
#define _SYMB_53 308
#define _SYMB_54 309
#define _SYMB_55 310
#define _SYMB_56 311
#define _SYMB_57 312
#define _SYMB_58 313
#define _SYMB_59 314
#define _SYMB_60 315
#define _SYMB_61 316
#define _SYMB_62 317
#define _SYMB_0 318
#define _SYMB_23 319
#define _SYMB_1 320
#define _SYMB_2 321
#define _SYMB_63 322
#define _SYMB_64 323
#define _SYMB_65 324
#define _SYMB_66 325




/* Copy the first part of user declarations.  */
#line 6 "rholang_mercury.y"

/* Begin C preamble code */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "Absyn.h"

#define YYMAXDEPTH 10000000

typedef struct rholang_mercury__buffer_state *YY_BUFFER_STATE;
YY_BUFFER_STATE rholang_mercury__scan_string(const char *str);
void rholang_mercury__delete_buffer(YY_BUFFER_STATE buf);
extern int yyparse(void);
extern int yylex(void);
extern int rholang_mercury__init_lexer(FILE * inp);
extern void yyerror(const char *str);

ListProc reverseListProc(ListProc l)
{
  ListProc prev = 0;
  ListProc tmp = 0;
  while (l)
  {
    tmp = l->listproc_;
    l->listproc_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListName reverseListName(ListName l)
{
  ListName prev = 0;
  ListName tmp = 0;
  while (l)
  {
    tmp = l->listname_;
    l->listname_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListLinearBind reverseListLinearBind(ListLinearBind l)
{
  ListLinearBind prev = 0;
  ListLinearBind tmp = 0;
  while (l)
  {
    tmp = l->listlinearbind_;
    l->listlinearbind_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListRepeatedBind reverseListRepeatedBind(ListRepeatedBind l)
{
  ListRepeatedBind prev = 0;
  ListRepeatedBind tmp = 0;
  while (l)
  {
    tmp = l->listrepeatedbind_;
    l->listrepeatedbind_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListPeekBind reverseListPeekBind(ListPeekBind l)
{
  ListPeekBind prev = 0;
  ListPeekBind tmp = 0;
  while (l)
  {
    tmp = l->listpeekbind_;
    l->listpeekbind_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListBranch reverseListBranch(ListBranch l)
{
  ListBranch prev = 0;
  ListBranch tmp = 0;
  while (l)
  {
    tmp = l->listbranch_;
    l->listbranch_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListCase reverseListCase(ListCase l)
{
  ListCase prev = 0;
  ListCase tmp = 0;
  while (l)
  {
    tmp = l->listcase_;
    l->listcase_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListNameDecl reverseListNameDecl(ListNameDecl l)
{
  ListNameDecl prev = 0;
  ListNameDecl tmp = 0;
  while (l)
  {
    tmp = l->listnamedecl_;
    l->listnamedecl_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}
ListKeyValuePair reverseListKeyValuePair(ListKeyValuePair l)
{
  ListKeyValuePair prev = 0;
  ListKeyValuePair tmp = 0;
  while (l)
  {
    tmp = l->listkeyvaluepair_;
    l->listkeyvaluepair_ = prev;
    prev = l;
    l = tmp;
  }
  return prev;
}

/* Global variables holding parse results for entrypoints. */
Proc YY_RESULT_Proc_ = 0;
ListProc YY_RESULT_ListProc_ = 0;
ProcVar YY_RESULT_ProcVar_ = 0;
Name YY_RESULT_Name_ = 0;
ListName YY_RESULT_ListName_ = 0;
Bundle YY_RESULT_Bundle_ = 0;
Receipt YY_RESULT_Receipt_ = 0;
ReceiptLinearImpl YY_RESULT_ReceiptLinearImpl_ = 0;
LinearBind YY_RESULT_LinearBind_ = 0;
ListLinearBind YY_RESULT_ListLinearBind_ = 0;
ReceiptRepeatedImpl YY_RESULT_ReceiptRepeatedImpl_ = 0;
RepeatedBind YY_RESULT_RepeatedBind_ = 0;
ListRepeatedBind YY_RESULT_ListRepeatedBind_ = 0;
ReceiptPeekImpl YY_RESULT_ReceiptPeekImpl_ = 0;
PeekBind YY_RESULT_PeekBind_ = 0;
ListPeekBind YY_RESULT_ListPeekBind_ = 0;
Send YY_RESULT_Send_ = 0;
Branch YY_RESULT_Branch_ = 0;
ListBranch YY_RESULT_ListBranch_ = 0;
Case YY_RESULT_Case_ = 0;
ListCase YY_RESULT_ListCase_ = 0;
NameDecl YY_RESULT_NameDecl_ = 0;
ListNameDecl YY_RESULT_ListNameDecl_ = 0;
BoolLiteral YY_RESULT_BoolLiteral_ = 0;
Ground YY_RESULT_Ground_ = 0;
Collection YY_RESULT_Collection_ = 0;
KeyValuePair YY_RESULT_KeyValuePair_ = 0;
ListKeyValuePair YY_RESULT_ListKeyValuePair_ = 0;
Tuple YY_RESULT_Tuple_ = 0;
ProcRemainder YY_RESULT_ProcRemainder_ = 0;
NameRemainder YY_RESULT_NameRemainder_ = 0;
VarRefKind YY_RESULT_VarRefKind_ = 0;
SimpleType YY_RESULT_SimpleType_ = 0;

/* Entrypoint: parse Proc from file. */
Proc pProc(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc1(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc1(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc2(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc2(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc3(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc3(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc4(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc4(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc5(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc5(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc6(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc6(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc7(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc7(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc8(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc8(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc9(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc9(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc10(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc10(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc11(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc11(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc12(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc12(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc13(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc13(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc14(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc14(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc15(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc15(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc16(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc16(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Proc_;
  }
}

/* Entrypoint: parse ListProc from file. */
ListProc pListProc(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListProc_;
  }
}

/* Entrypoint: parse ListProc from string. */
ListProc psListProc(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListProc_;
  }
}

/* Entrypoint: parse ProcVar from file. */
ProcVar pProcVar(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ProcVar_;
  }
}

/* Entrypoint: parse ProcVar from string. */
ProcVar psProcVar(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ProcVar_;
  }
}

/* Entrypoint: parse Name from file. */
Name pName(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Name_;
  }
}

/* Entrypoint: parse Name from string. */
Name psName(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Name_;
  }
}

/* Entrypoint: parse ListName from file. */
ListName pListName(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListName_;
  }
}

/* Entrypoint: parse ListName from string. */
ListName psListName(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListName_;
  }
}

/* Entrypoint: parse Bundle from file. */
Bundle pBundle(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Bundle_;
  }
}

/* Entrypoint: parse Bundle from string. */
Bundle psBundle(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Bundle_;
  }
}

/* Entrypoint: parse Receipt from file. */
Receipt pReceipt(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Receipt_;
  }
}

/* Entrypoint: parse Receipt from string. */
Receipt psReceipt(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Receipt_;
  }
}

/* Entrypoint: parse ReceiptLinearImpl from file. */
ReceiptLinearImpl pReceiptLinearImpl(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ReceiptLinearImpl_;
  }
}

/* Entrypoint: parse ReceiptLinearImpl from string. */
ReceiptLinearImpl psReceiptLinearImpl(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ReceiptLinearImpl_;
  }
}

/* Entrypoint: parse LinearBind from file. */
LinearBind pLinearBind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_LinearBind_;
  }
}

/* Entrypoint: parse LinearBind from string. */
LinearBind psLinearBind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_LinearBind_;
  }
}

/* Entrypoint: parse ListLinearBind from file. */
ListLinearBind pListLinearBind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListLinearBind_;
  }
}

/* Entrypoint: parse ListLinearBind from string. */
ListLinearBind psListLinearBind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListLinearBind_;
  }
}

/* Entrypoint: parse ReceiptRepeatedImpl from file. */
ReceiptRepeatedImpl pReceiptRepeatedImpl(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ReceiptRepeatedImpl_;
  }
}

/* Entrypoint: parse ReceiptRepeatedImpl from string. */
ReceiptRepeatedImpl psReceiptRepeatedImpl(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ReceiptRepeatedImpl_;
  }
}

/* Entrypoint: parse RepeatedBind from file. */
RepeatedBind pRepeatedBind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_RepeatedBind_;
  }
}

/* Entrypoint: parse RepeatedBind from string. */
RepeatedBind psRepeatedBind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_RepeatedBind_;
  }
}

/* Entrypoint: parse ListRepeatedBind from file. */
ListRepeatedBind pListRepeatedBind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListRepeatedBind_;
  }
}

/* Entrypoint: parse ListRepeatedBind from string. */
ListRepeatedBind psListRepeatedBind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListRepeatedBind_;
  }
}

/* Entrypoint: parse ReceiptPeekImpl from file. */
ReceiptPeekImpl pReceiptPeekImpl(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ReceiptPeekImpl_;
  }
}

/* Entrypoint: parse ReceiptPeekImpl from string. */
ReceiptPeekImpl psReceiptPeekImpl(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ReceiptPeekImpl_;
  }
}

/* Entrypoint: parse PeekBind from file. */
PeekBind pPeekBind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_PeekBind_;
  }
}

/* Entrypoint: parse PeekBind from string. */
PeekBind psPeekBind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_PeekBind_;
  }
}

/* Entrypoint: parse ListPeekBind from file. */
ListPeekBind pListPeekBind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListPeekBind_;
  }
}

/* Entrypoint: parse ListPeekBind from string. */
ListPeekBind psListPeekBind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListPeekBind_;
  }
}

/* Entrypoint: parse Send from file. */
Send pSend(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Send_;
  }
}

/* Entrypoint: parse Send from string. */
Send psSend(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Send_;
  }
}

/* Entrypoint: parse Branch from file. */
Branch pBranch(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Branch_;
  }
}

/* Entrypoint: parse Branch from string. */
Branch psBranch(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Branch_;
  }
}

/* Entrypoint: parse ListBranch from file. */
ListBranch pListBranch(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListBranch_;
  }
}

/* Entrypoint: parse ListBranch from string. */
ListBranch psListBranch(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListBranch_;
  }
}

/* Entrypoint: parse Case from file. */
Case pCase(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Case_;
  }
}

/* Entrypoint: parse Case from string. */
Case psCase(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Case_;
  }
}

/* Entrypoint: parse ListCase from file. */
ListCase pListCase(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListCase_;
  }
}

/* Entrypoint: parse ListCase from string. */
ListCase psListCase(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListCase_;
  }
}

/* Entrypoint: parse NameDecl from file. */
NameDecl pNameDecl(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_NameDecl_;
  }
}

/* Entrypoint: parse NameDecl from string. */
NameDecl psNameDecl(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_NameDecl_;
  }
}

/* Entrypoint: parse ListNameDecl from file. */
ListNameDecl pListNameDecl(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListNameDecl_;
  }
}

/* Entrypoint: parse ListNameDecl from string. */
ListNameDecl psListNameDecl(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListNameDecl_;
  }
}

/* Entrypoint: parse BoolLiteral from file. */
BoolLiteral pBoolLiteral(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_BoolLiteral_;
  }
}

/* Entrypoint: parse BoolLiteral from string. */
BoolLiteral psBoolLiteral(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_BoolLiteral_;
  }
}

/* Entrypoint: parse Ground from file. */
Ground pGround(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Ground_;
  }
}

/* Entrypoint: parse Ground from string. */
Ground psGround(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Ground_;
  }
}

/* Entrypoint: parse Collection from file. */
Collection pCollection(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Collection_;
  }
}

/* Entrypoint: parse Collection from string. */
Collection psCollection(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Collection_;
  }
}

/* Entrypoint: parse KeyValuePair from file. */
KeyValuePair pKeyValuePair(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_KeyValuePair_;
  }
}

/* Entrypoint: parse KeyValuePair from string. */
KeyValuePair psKeyValuePair(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_KeyValuePair_;
  }
}

/* Entrypoint: parse ListKeyValuePair from file. */
ListKeyValuePair pListKeyValuePair(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListKeyValuePair_;
  }
}

/* Entrypoint: parse ListKeyValuePair from string. */
ListKeyValuePair psListKeyValuePair(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ListKeyValuePair_;
  }
}

/* Entrypoint: parse Tuple from file. */
Tuple pTuple(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Tuple_;
  }
}

/* Entrypoint: parse Tuple from string. */
Tuple psTuple(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_Tuple_;
  }
}

/* Entrypoint: parse ProcRemainder from file. */
ProcRemainder pProcRemainder(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ProcRemainder_;
  }
}

/* Entrypoint: parse ProcRemainder from string. */
ProcRemainder psProcRemainder(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_ProcRemainder_;
  }
}

/* Entrypoint: parse NameRemainder from file. */
NameRemainder pNameRemainder(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_NameRemainder_;
  }
}

/* Entrypoint: parse NameRemainder from string. */
NameRemainder psNameRemainder(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_NameRemainder_;
  }
}

/* Entrypoint: parse VarRefKind from file. */
VarRefKind pVarRefKind(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_VarRefKind_;
  }
}

/* Entrypoint: parse VarRefKind from string. */
VarRefKind psVarRefKind(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_VarRefKind_;
  }
}

/* Entrypoint: parse SimpleType from file. */
SimpleType pSimpleType(FILE *inp)
{
  rholang_mercury__init_lexer(inp);
  int result = yyparse();
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_SimpleType_;
  }
}

/* Entrypoint: parse SimpleType from string. */
SimpleType psSimpleType(const char *str)
{
  YY_BUFFER_STATE buf;
  rholang_mercury__init_lexer(0);
  buf = rholang_mercury__scan_string(str);
  int result = yyparse();
  rholang_mercury__delete_buffer(buf);
  if (result)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return YY_RESULT_SimpleType_;
  }
}


/* End C preamble code */


/* Enabling traces.  */
#ifndef YYDEBUG
# define YYDEBUG 1
#endif

/* Enabling verbose error messages.  */
#ifdef YYERROR_VERBOSE
# undef YYERROR_VERBOSE
# define YYERROR_VERBOSE 1
#else
# define YYERROR_VERBOSE 0
#endif

/* Enabling the token table.  */
#ifndef YYTOKEN_TABLE
# define YYTOKEN_TABLE 0
#endif

#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
typedef union YYSTYPE
#line 1800 "rholang_mercury.y"
{
  int    _int;
  char   _char;
  double _double;
  char*  _string;
  Proc proc_;
  ListProc listproc_;
  ProcVar procvar_;
  Name name_;
  ListName listname_;
  Bundle bundle_;
  Receipt receipt_;
  ReceiptLinearImpl receiptlinearimpl_;
  LinearBind linearbind_;
  ListLinearBind listlinearbind_;
  ReceiptRepeatedImpl receiptrepeatedimpl_;
  RepeatedBind repeatedbind_;
  ListRepeatedBind listrepeatedbind_;
  ReceiptPeekImpl receiptpeekimpl_;
  PeekBind peekbind_;
  ListPeekBind listpeekbind_;
  Send send_;
  Branch branch_;
  ListBranch listbranch_;
  Case case_;
  ListCase listcase_;
  NameDecl namedecl_;
  ListNameDecl listnamedecl_;
  BoolLiteral boolliteral_;
  Ground ground_;
  Collection collection_;
  KeyValuePair keyvaluepair_;
  ListKeyValuePair listkeyvaluepair_;
  Tuple tuple_;
  ProcRemainder procremainder_;
  NameRemainder nameremainder_;
  VarRefKind varrefkind_;
  SimpleType simpletype_;
}
/* Line 193 of yacc.c.  */
#line 2076 "Parser.c"
	YYSTYPE;
# define yystype YYSTYPE /* obsolescent; will be withdrawn */
# define YYSTYPE_IS_DECLARED 1
# define YYSTYPE_IS_TRIVIAL 1
#endif

#if ! defined YYLTYPE && ! defined YYLTYPE_IS_DECLARED
typedef struct YYLTYPE
{
  int first_line;
  int first_column;
  int last_line;
  int last_column;
} YYLTYPE;
# define yyltype YYLTYPE /* obsolescent; will be withdrawn */
# define YYLTYPE_IS_DECLARED 1
# define YYLTYPE_IS_TRIVIAL 1
#endif


/* Copy the second part of user declarations.  */


/* Line 216 of yacc.c.  */
#line 2101 "Parser.c"

#ifdef short
# undef short
#endif

#ifdef YYTYPE_UINT8
typedef YYTYPE_UINT8 yytype_uint8;
#else
typedef unsigned char yytype_uint8;
#endif

#ifdef YYTYPE_INT8
typedef YYTYPE_INT8 yytype_int8;
#elif (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
typedef signed char yytype_int8;
#else
typedef short int yytype_int8;
#endif

#ifdef YYTYPE_UINT16
typedef YYTYPE_UINT16 yytype_uint16;
#else
typedef unsigned short int yytype_uint16;
#endif

#ifdef YYTYPE_INT16
typedef YYTYPE_INT16 yytype_int16;
#else
typedef short int yytype_int16;
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif ! defined YYSIZE_T && (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned int
# endif
#endif

#define YYSIZE_MAXIMUM ((YYSIZE_T) -1)

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(msgid) dgettext ("bison-runtime", msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(msgid) msgid
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(e) ((void) (e))
#else
# define YYUSE(e) /* empty */
#endif

/* Identity function, used to suppress warnings about constant conditions.  */
#ifndef lint
# define YYID(n) (n)
#else
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static int
YYID (int i)
#else
static int
YYID (i)
    int i;
#endif
{
  return i;
}
#endif

#if ! defined yyoverflow || YYERROR_VERBOSE

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined _STDLIB_H && (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#     ifndef _STDLIB_H
#      define _STDLIB_H 1
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's `empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (YYID (0))
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined _STDLIB_H \
       && ! ((defined YYMALLOC || defined malloc) \
	     && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef _STDLIB_H
#    define _STDLIB_H 1
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined _STDLIB_H && (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined _STDLIB_H && (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* ! defined yyoverflow || YYERROR_VERBOSE */


#if (! defined yyoverflow \
     && (! defined __cplusplus \
	 || (defined YYLTYPE_IS_TRIVIAL && YYLTYPE_IS_TRIVIAL \
	     && defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yytype_int16 yyss;
  YYSTYPE yyvs;
    YYLTYPE yyls;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (sizeof (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (sizeof (yytype_int16) + sizeof (YYSTYPE) + sizeof (YYLTYPE)) \
      + 2 * YYSTACK_GAP_MAXIMUM)

/* Copy COUNT objects from FROM to TO.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(To, From, Count) \
      __builtin_memcpy (To, From, (Count) * sizeof (*(From)))
#  else
#   define YYCOPY(To, From, Count)		\
      do					\
	{					\
	  YYSIZE_T yyi;				\
	  for (yyi = 0; yyi < (Count); yyi++)	\
	    (To)[yyi] = (From)[yyi];		\
	}					\
      while (YYID (0))
#  endif
# endif

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack)					\
    do									\
      {									\
	YYSIZE_T yynewbytes;						\
	YYCOPY (&yyptr->Stack, Stack, yysize);				\
	Stack = &yyptr->Stack;						\
	yynewbytes = yystacksize * sizeof (*Stack) + YYSTACK_GAP_MAXIMUM; \
	yyptr += yynewbytes / sizeof (*yyptr);				\
      }									\
    while (YYID (0))

#endif

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  87
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   474

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  71
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  50
/* YYNRULES -- Number of rules.  */
#define YYNRULES  128
/* YYNRULES -- Number of states.  */
#define YYNSTATES  240

/* YYTRANSLATE(YYLEX) -- Bison symbol number corresponding to YYLEX.  */
#define YYUNDEFTOK  2
#define YYMAXUTOK   325

#define YYTRANSLATE(YYX)						\
  ((unsigned int) (YYX) <= YYMAXUTOK ? yytranslate[YYX] : YYUNDEFTOK)

/* YYTRANSLATE[YYLEX] -- Bison symbol number corresponding to YYLEX.  */
static const yytype_uint8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,    41,    42,    43,    44,
      45,    46,    47,    48,    49,    50,    51,    52,    53,    54,
      55,    56,    57,    58,    59,    60,    61,    62,    63,    64,
      65,    66,    67,    68,    69,    70
};

#if YYDEBUG
/* YYPRHS[YYN] -- Index of the first RHS symbol of rule number YYN in
   YYRHS.  */
static const yytype_uint16 yyprhs[] =
{
       0,     0,     3,     5,     9,    11,    17,    25,    30,    32,
      43,    51,    56,    62,    67,    69,    75,    77,    81,    83,
      87,    89,    93,    97,   101,   103,   107,   111,   115,   119,
     121,   125,   129,   133,   137,   139,   143,   147,   151,   155,
     157,   160,   163,   165,   172,   176,   178,   181,   183,   186,
     190,   192,   196,   198,   201,   205,   207,   209,   211,   213,
     215,   216,   218,   222,   224,   226,   228,   230,   233,   234,
     236,   240,   242,   244,   246,   248,   250,   252,   254,   256,
     261,   263,   267,   269,   274,   276,   280,   282,   287,   289,
     293,   295,   297,   301,   303,   306,   310,   312,   315,   317,
     322,   324,   328,   330,   332,   334,   336,   338,   340,   345,
     347,   353,   358,   362,   363,   365,   369,   373,   379,   382,
     383,   387,   388,   390,   393,   395,   397,   399,   401
};

/* YYRHS -- A `-1'-separated list of the rules' RHS.  */
static const yytype_int8 yyrhs[] =
{
      72,     0,    -1,    73,    -1,    72,    64,    73,    -1,    74,
      -1,    54,     9,    72,    10,    74,    -1,    54,     9,    72,
      10,    74,    51,    73,    -1,    58,   110,    55,    73,    -1,
      75,    -1,    50,    91,     9,    92,   118,    10,    28,    63,
      72,    65,    -1,    53,     9,    94,    10,    63,    72,    65,
      -1,    61,    63,   106,    65,    -1,    56,    76,    63,   108,
      65,    -1,    93,    63,    72,    65,    -1,    76,    -1,    91,
     104,     9,    89,    10,    -1,    77,    -1,    76,    60,    77,
      -1,    78,    -1,    77,    45,    78,    -1,    79,    -1,    79,
      57,    79,    -1,    78,    29,    79,    -1,    78,     6,    79,
      -1,    80,    -1,    79,    24,    80,    -1,    79,    27,    80,
      -1,    79,    31,    80,    -1,    79,    32,    80,    -1,    81,
      -1,    80,    12,    81,    -1,    80,    16,    81,    -1,    80,
      13,    81,    -1,    80,    17,    81,    -1,    82,    -1,    81,
      11,    82,    -1,    81,    20,    82,    -1,    81,     7,    82,
      -1,    81,     8,    82,    -1,    83,    -1,    59,    82,    -1,
      16,    82,    -1,    84,    -1,    83,    18,    70,     9,    89,
      10,    -1,     9,    76,    10,    -1,    85,    -1,    11,    91,
      -1,    86,    -1,   119,    70,    -1,    85,    42,    86,    -1,
      87,    -1,    86,    21,    87,    -1,    88,    -1,    66,    87,
      -1,    63,    72,    65,    -1,   112,    -1,   113,    -1,    90,
      -1,    37,    -1,   120,    -1,    -1,    72,    -1,    72,    14,
      89,    -1,    44,    -1,    70,    -1,    44,    -1,    70,    -1,
      33,    84,    -1,    -1,    91,    -1,    91,    14,    92,    -1,
      47,    -1,    48,    -1,    49,    -1,    46,    -1,    95,    -1,
      98,    -1,   101,    -1,    97,    -1,    92,   118,    25,    91,
      -1,    96,    -1,    96,    23,    97,    -1,   100,    -1,    92,
     118,    27,    91,    -1,    99,    -1,    99,    23,   100,    -1,
     103,    -1,    92,   118,    26,    91,    -1,   102,    -1,   102,
      23,   103,    -1,     4,    -1,     5,    -1,    95,    30,    75,
      -1,   105,    -1,   105,   106,    -1,    85,    30,    75,    -1,
     107,    -1,   107,   108,    -1,    70,    -1,    70,     9,    69,
      10,    -1,   109,    -1,   109,    14,   110,    -1,    62,    -1,
      52,    -1,   111,    -1,    67,    -1,    68,    -1,    69,    -1,
      41,    89,   117,    43,    -1,   116,    -1,    38,     9,    89,
     117,    10,    -1,    63,   115,   117,    65,    -1,    72,    22,
      72,    -1,    -1,   114,    -1,   114,    14,   115,    -1,     9,
      72,    15,    -1,     9,    72,    14,    89,    10,    -1,    19,
      90,    -1,    -1,    19,    33,    90,    -1,    -1,    28,    -1,
      28,    11,    -1,    34,    -1,    36,    -1,    39,    -1,    40,
      -1,    35,    -1
};

/* YYRLINE[YYN] -- source line where rule number YYN was defined.  */
static const yytype_uint16 yyrline[] =
{
       0,  1964,  1964,  1965,  1967,  1968,  1969,  1970,  1972,  1973,
    1974,  1975,  1976,  1977,  1979,  1980,  1982,  1983,  1985,  1986,
    1988,  1989,  1990,  1991,  1993,  1994,  1995,  1996,  1997,  1999,
    2000,  2001,  2002,  2003,  2005,  2006,  2007,  2008,  2009,  2011,
    2012,  2013,  2015,  2016,  2017,  2019,  2020,  2022,  2023,  2024,
    2026,  2027,  2029,  2030,  2032,  2033,  2034,  2035,  2036,  2037,
    2039,  2040,  2041,  2043,  2044,  2046,  2047,  2048,  2050,  2051,
    2052,  2054,  2055,  2056,  2057,  2059,  2060,  2061,  2063,  2065,
    2067,  2068,  2070,  2072,  2074,  2075,  2077,  2079,  2081,  2082,
    2084,  2085,  2087,  2089,  2090,  2092,  2094,  2095,  2097,  2098,
    2100,  2101,  2103,  2104,  2106,  2107,  2108,  2109,  2111,  2112,
    2113,  2114,  2116,  2118,  2119,  2120,  2122,  2123,  2125,  2126,
    2128,  2129,  2131,  2132,  2134,  2135,  2136,  2137,  2138
};
#endif

#if YYDEBUG || YYERROR_VERBOSE || YYTOKEN_TABLE
/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "$end", "error", "$undefined", "_ERROR_", "_SYMB_32", "_SYMB_33",
  "_SYMB_21", "_SYMB_11", "_SYMB_12", "_SYMB_7", "_SYMB_8", "_SYMB_5",
  "_SYMB_13", "_SYMB_14", "_SYMB_24", "_SYMB_38", "_SYMB_9", "_SYMB_15",
  "_SYMB_6", "_SYMB_39", "_SYMB_10", "_SYMB_3", "_SYMB_37", "_SYMB_30",
  "_SYMB_16", "_SYMB_29", "_SYMB_31", "_SYMB_17", "_SYMB_22", "_SYMB_20",
  "_SYMB_34", "_SYMB_18", "_SYMB_19", "_SYMB_26", "_SYMB_40", "_SYMB_41",
  "_SYMB_42", "_SYMB_43", "_SYMB_44", "_SYMB_45", "_SYMB_46", "_SYMB_35",
  "_SYMB_4", "_SYMB_36", "_SYMB_25", "_SYMB_47", "_SYMB_48", "_SYMB_27",
  "_SYMB_28", "_SYMB_49", "_SYMB_50", "_SYMB_51", "_SYMB_52", "_SYMB_53",
  "_SYMB_54", "_SYMB_55", "_SYMB_56", "_SYMB_57", "_SYMB_58", "_SYMB_59",
  "_SYMB_60", "_SYMB_61", "_SYMB_62", "_SYMB_0", "_SYMB_23", "_SYMB_1",
  "_SYMB_2", "_SYMB_63", "_SYMB_64", "_SYMB_65", "_SYMB_66", "$accept",
  "Proc", "Proc1", "Proc2", "Proc3", "Proc4", "Proc5", "Proc6", "Proc7",
  "Proc8", "Proc9", "Proc10", "Proc11", "Proc12", "Proc13", "Proc14",
  "Proc15", "Proc16", "ListProc", "ProcVar", "Name", "ListName", "Bundle",
  "Receipt", "ReceiptLinearImpl", "LinearBind", "ListLinearBind",
  "ReceiptRepeatedImpl", "RepeatedBind", "ListRepeatedBind",
  "ReceiptPeekImpl", "PeekBind", "ListPeekBind", "Send", "Branch",
  "ListBranch", "Case", "ListCase", "NameDecl", "ListNameDecl",
  "BoolLiteral", "Ground", "Collection", "KeyValuePair",
  "ListKeyValuePair", "Tuple", "ProcRemainder", "NameRemainder",
  "VarRefKind", "SimpleType", 0
};
#endif

# ifdef YYPRINT
/* YYTOKNUM[YYLEX-NUM] -- Internal token number corresponding to
   token YYLEX-NUM.  */
static const yytype_uint16 yytoknum[] =
{
       0,   256,   257,   258,   259,   260,   261,   262,   263,   264,
     265,   266,   267,   268,   269,   270,   271,   272,   273,   274,
     275,   276,   277,   278,   279,   280,   281,   282,   283,   284,
     285,   286,   287,   288,   289,   290,   291,   292,   293,   294,
     295,   296,   297,   298,   299,   300,   301,   302,   303,   304,
     305,   306,   307,   308,   309,   310,   311,   312,   313,   314,
     315,   316,   317,   318,   319,   320,   321,   322,   323,   324,
     325
};
# endif

/* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_uint8 yyr1[] =
{
       0,    71,    72,    72,    73,    73,    73,    73,    74,    74,
      74,    74,    74,    74,    75,    75,    76,    76,    77,    77,
      78,    78,    78,    78,    79,    79,    79,    79,    79,    80,
      80,    80,    80,    80,    81,    81,    81,    81,    81,    82,
      82,    82,    83,    83,    83,    84,    84,    85,    85,    85,
      86,    86,    87,    87,    88,    88,    88,    88,    88,    88,
      89,    89,    89,    90,    90,    91,    91,    91,    92,    92,
      92,    93,    93,    93,    93,    94,    94,    94,    95,    96,
      97,    97,    98,    99,   100,   100,   101,   102,   103,   103,
     104,   104,   105,   106,   106,   107,   108,   108,   109,   109,
     110,   110,   111,   111,   112,   112,   112,   112,   113,   113,
     113,   113,   114,   115,   115,   115,   116,   116,   117,   117,
     118,   118,   119,   119,   120,   120,   120,   120,   120
};

/* YYR2[YYN] -- Number of symbols composing right hand side of rule YYN.  */
static const yytype_uint8 yyr2[] =
{
       0,     2,     1,     3,     1,     5,     7,     4,     1,    10,
       7,     4,     5,     4,     1,     5,     1,     3,     1,     3,
       1,     3,     3,     3,     1,     3,     3,     3,     3,     1,
       3,     3,     3,     3,     1,     3,     3,     3,     3,     1,
       2,     2,     1,     6,     3,     1,     2,     1,     2,     3,
       1,     3,     1,     2,     3,     1,     1,     1,     1,     1,
       0,     1,     3,     1,     1,     1,     1,     2,     0,     1,
       3,     1,     1,     1,     1,     1,     1,     1,     1,     4,
       1,     3,     1,     4,     1,     3,     1,     4,     1,     3,
       1,     1,     3,     1,     2,     3,     1,     2,     1,     4,
       1,     3,     1,     1,     1,     1,     1,     1,     4,     1,
       5,     4,     3,     0,     1,     3,     3,     5,     2,     0,
       3,     0,     1,     2,     1,     1,     1,     1,     1
};

/* YYDEFACT[STATE-NAME] -- Default rule to reduce with in state
   STATE-NUM when YYTABLE doesn't specify something else to do.  Zero
   means the default is an error.  */
static const yytype_uint8 yydefact[] =
{
       0,     0,     0,     0,   122,     0,   124,   128,   125,    58,
       0,   126,   127,    60,    63,    74,    71,    72,    73,     0,
     103,     0,     0,     0,     0,     0,     0,   102,   113,     0,
     105,   106,   107,    64,     0,     2,     4,     8,    14,    16,
      18,    20,    24,    29,    34,    39,    42,    45,    47,    50,
      52,    57,     0,     0,   104,    55,    56,   109,     0,    59,
       0,    14,    65,    66,    46,    63,    64,    41,   123,     0,
      67,    60,    61,   119,     0,    68,     0,     0,    98,   100,
       0,    40,    68,     0,   114,   119,    53,     1,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    90,
      91,     0,     0,    48,    60,   116,    44,   119,    60,     0,
       0,    68,    69,   121,     0,    75,    80,    78,    76,    84,
      82,    77,    88,    86,     0,     0,     0,     0,     0,   121,
       0,    68,     0,     0,    54,   113,     0,     3,    17,    19,
      23,    22,    25,    26,    27,    28,    21,    30,    32,    31,
      33,    37,    38,    35,    36,     0,    49,    51,    60,     0,
       0,     0,    62,   118,   108,   121,    68,     0,     0,     0,
      68,    68,    68,     0,     0,    96,     0,     0,   101,     7,
       0,     0,    94,    11,   112,     0,   115,   111,    60,     0,
      13,   117,   110,     0,    70,     0,     0,     0,     0,     0,
      81,   121,    85,   121,    89,     5,     0,    97,    12,    99,
      92,     0,    15,     0,   120,    79,    87,    83,     0,     0,
       0,     0,    95,    43,     0,    10,     6,     0,     0,     9
};

/* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
      -1,    72,    35,    36,    37,    38,    39,    40,    41,    42,
      43,    44,    45,    46,    47,    48,    49,    50,    73,    51,
      52,   139,    53,   124,   140,   126,   127,   128,   129,   130,
     131,   132,   133,   111,   141,   142,   185,   186,    79,    80,
      54,    55,    56,    84,    85,    57,   120,   178,    58,    59
};

/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
   STATE-NUM.  */
#define YYPACT_NINF -169
static const yytype_int16 yypact[] =
{
     182,   182,   -11,   330,    23,   100,  -169,  -169,  -169,  -169,
      17,  -169,  -169,   182,    73,  -169,  -169,  -169,  -169,   -11,
    -169,    31,    41,   330,    -9,   330,    10,  -169,   182,   404,
    -169,  -169,  -169,   109,     8,  -169,  -169,  -169,    21,    18,
      15,    11,    81,    63,  -169,    57,  -169,    44,    69,  -169,
    -169,  -169,   120,    29,  -169,  -169,  -169,  -169,    38,  -169,
      -2,     4,  -169,  -169,  -169,  -169,  -169,  -169,  -169,   182,
    -169,   182,     3,   108,   133,   -11,   182,   -24,   141,   137,
      98,  -169,   -11,    -7,   140,   108,  -169,  -169,   182,   330,
     330,   330,   330,   330,   330,   330,   330,   330,   330,   330,
     330,   330,   330,   330,   330,   330,    85,   404,   404,  -169,
    -169,   147,   182,  -169,   182,  -169,  -169,   108,   182,     9,
     114,   -11,   144,   142,   149,  -169,   148,  -169,  -169,   150,
    -169,  -169,   151,  -169,     1,   367,    91,    -9,   182,   142,
     134,   -14,   107,   182,  -169,   182,   110,  -169,    18,    15,
      64,    64,    81,    81,    81,    81,    64,    63,    63,    63,
      63,  -169,  -169,  -169,  -169,   156,    69,  -169,   182,    66,
     166,   167,  -169,  -169,  -169,   142,   -11,   145,    96,   116,
     -11,   -11,   -11,   244,    -1,   367,   115,   172,  -169,  -169,
     158,   293,  -169,  -169,   124,    -4,  -169,  -169,   182,   174,
    -169,  -169,  -169,   179,  -169,     9,   -11,   -11,   -11,   182,
    -169,   142,  -169,   142,  -169,   139,   293,  -169,  -169,  -169,
    -169,   184,  -169,   164,  -169,  -169,  -169,  -169,    68,   168,
     170,   182,  -169,  -169,   136,  -169,  -169,   182,    84,  -169
};

/* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -169,     0,   -84,    14,  -168,    24,   111,   112,   -60,     7,
      19,     2,  -169,   196,  -119,    97,   -23,  -169,   -69,  -116,
       5,   -66,  -169,  -169,   128,  -169,    25,  -169,  -169,    26,
    -169,  -169,    32,  -169,  -169,    65,  -169,    39,  -169,    71,
    -169,  -169,  -169,  -169,    80,  -169,   -65,  -129,  -169,  -169
};

/* YYTABLE[YYPACT[STATE-NUM]].  What to do in state STATE-NUM.  If
   positive, shift that token.  If negative, reduce the rule which
   number is the opposite.  If zero, do what YYDEFACT says.
   If YYTABLE_NINF, syntax error.  */
#define YYTABLE_NINF -94
static const yytype_int16 yytable[] =
{
      34,    60,   117,   173,   147,    67,    86,    64,    87,   123,
     190,   183,   114,   115,   116,   143,   184,   118,   143,     5,
     146,    91,     5,   220,    74,    61,    71,    81,    83,   216,
      62,   150,   151,    62,    68,    93,    89,   156,    94,   135,
      75,   107,    95,    96,    92,   170,   203,    77,   232,   172,
      76,   -93,   171,    65,   189,   175,    63,    88,   144,    63,
      88,    78,    88,    90,    89,    88,   184,    88,    97,    60,
     102,   103,    88,    82,   104,   106,   134,   -65,   -65,    66,
     122,    89,   229,   105,   230,   167,   107,   122,    93,   224,
     108,    94,   112,    98,    99,    95,    96,   100,   101,   199,
     152,   153,   154,   155,   161,   162,   163,   164,   113,    69,
     204,     2,   169,   -66,   -66,   211,   213,   157,   158,   159,
     160,   206,   207,   208,   109,   110,   122,   119,     4,   221,
      88,   200,    88,   235,     6,     7,     8,     9,    10,    11,
      12,    13,   121,   194,    65,   195,   122,   236,    88,   239,
     136,   137,    20,   138,   145,   165,   168,   174,   176,   179,
     187,   177,    27,    28,   191,   198,    29,    30,    31,    32,
      66,   180,   193,   181,   182,   197,   201,   202,   205,   209,
     218,   122,   219,   206,   222,   122,   122,   122,    88,   223,
     231,     1,   234,     2,   233,   208,   207,   215,     3,   237,
     148,    70,   149,   125,   166,   210,   192,   212,   188,   228,
       4,   225,   226,   227,   214,     5,     6,     7,     8,     9,
      10,    11,    12,    13,   217,   196,    14,     0,    15,    16,
      17,    18,    19,     0,    20,    21,    22,   238,    23,     0,
      24,    25,     0,    26,    27,    28,     0,     0,    29,    30,
      31,    32,    33,     1,     0,     2,     0,     0,     0,     0,
       3,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     4,     0,     0,     0,     0,     5,     6,     7,
       8,     9,    10,    11,    12,    13,     0,     0,    14,     0,
      15,    16,    17,    18,    19,     0,    20,    21,     0,     0,
      23,     0,     1,    25,     2,    26,    27,    28,     0,     3,
      29,    30,    31,    32,    33,     0,     0,     0,     0,     0,
       0,     4,     0,     0,     0,     0,     5,     6,     7,     8,
       9,    10,    11,    12,    13,     0,     0,    14,     0,     1,
       0,     2,     0,     0,     0,    20,     3,     0,     0,     0,
       0,     0,    25,     0,     0,    27,    28,     0,     4,    29,
      30,    31,    32,    33,     6,     7,     8,     9,    10,    11,
      12,    13,     0,     0,    65,     0,    69,     0,     0,     0,
       0,     0,    20,     0,     0,     0,     0,     0,     0,    25,
       0,     0,    27,    28,     0,     4,    29,    30,    31,    32,
      66,     6,     7,     8,     9,    10,    11,    12,    13,     0,
       0,    65,     0,    69,     0,     0,     0,     0,     0,    20,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    27,
      28,     0,     0,    29,    30,    31,    32,    66,     6,     7,
       8,     9,    10,    11,    12,    13,     0,     0,    65,     0,
       0,     0,     0,     0,     0,     0,    20,     0,     0,     0,
       0,     0,     0,     0,     0,     0,    27,    28,     0,     0,
      29,    30,    31,    32,    66
};

static const yytype_int16 yycheck[] =
{
       0,     1,    71,   119,    88,     3,    29,     2,     0,    75,
     139,    10,    14,    15,    10,    22,   135,    14,    22,    33,
      85,     6,    33,   191,    19,     1,     9,    25,    28,    30,
      44,    91,    92,    44,    11,    24,    60,    97,    27,    63,
       9,    42,    31,    32,    29,   114,   175,    23,   216,   118,
       9,    65,   117,    44,   138,   121,    70,    64,    65,    70,
      64,    70,    64,    45,    60,    64,   185,    64,    57,    69,
       7,     8,    64,    63,    11,    18,    76,     4,     5,    70,
      75,    60,   211,    20,   213,   108,    42,    82,    24,   205,
      21,    27,    63,    12,    13,    31,    32,    16,    17,   168,
      93,    94,    95,    96,   102,   103,   104,   105,    70,     9,
     176,    11,   112,     4,     5,   181,   182,    98,    99,   100,
     101,    25,    26,    27,     4,     5,   121,    19,    28,   198,
      64,    65,    64,    65,    34,    35,    36,    37,    38,    39,
      40,    41,     9,   143,    44,   145,   141,   231,    64,    65,
       9,    14,    52,    55,    14,    70,     9,    43,    14,    10,
      69,    19,    62,    63,    30,     9,    66,    67,    68,    69,
      70,    23,    65,    23,    23,    65,    10,    10,    33,    63,
      65,   176,    10,    25,    10,   180,   181,   182,    64,    10,
      51,     9,    28,    11,    10,    27,    26,   183,    16,    63,
      89,     5,    90,    75,   107,   180,   141,   181,   137,   209,
      28,   206,   207,   208,   182,    33,    34,    35,    36,    37,
      38,    39,    40,    41,   185,   145,    44,    -1,    46,    47,
      48,    49,    50,    -1,    52,    53,    54,   237,    56,    -1,
      58,    59,    -1,    61,    62,    63,    -1,    -1,    66,    67,
      68,    69,    70,     9,    -1,    11,    -1,    -1,    -1,    -1,
      16,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    28,    -1,    -1,    -1,    -1,    33,    34,    35,
      36,    37,    38,    39,    40,    41,    -1,    -1,    44,    -1,
      46,    47,    48,    49,    50,    -1,    52,    53,    -1,    -1,
      56,    -1,     9,    59,    11,    61,    62,    63,    -1,    16,
      66,    67,    68,    69,    70,    -1,    -1,    -1,    -1,    -1,
      -1,    28,    -1,    -1,    -1,    -1,    33,    34,    35,    36,
      37,    38,    39,    40,    41,    -1,    -1,    44,    -1,     9,
      -1,    11,    -1,    -1,    -1,    52,    16,    -1,    -1,    -1,
      -1,    -1,    59,    -1,    -1,    62,    63,    -1,    28,    66,
      67,    68,    69,    70,    34,    35,    36,    37,    38,    39,
      40,    41,    -1,    -1,    44,    -1,     9,    -1,    -1,    -1,
      -1,    -1,    52,    -1,    -1,    -1,    -1,    -1,    -1,    59,
      -1,    -1,    62,    63,    -1,    28,    66,    67,    68,    69,
      70,    34,    35,    36,    37,    38,    39,    40,    41,    -1,
      -1,    44,    -1,     9,    -1,    -1,    -1,    -1,    -1,    52,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    62,
      63,    -1,    -1,    66,    67,    68,    69,    70,    34,    35,
      36,    37,    38,    39,    40,    41,    -1,    -1,    44,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    52,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    62,    63,    -1,    -1,
      66,    67,    68,    69,    70
};

/* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
   symbol of state STATE-NUM.  */
static const yytype_uint8 yystos[] =
{
       0,     9,    11,    16,    28,    33,    34,    35,    36,    37,
      38,    39,    40,    41,    44,    46,    47,    48,    49,    50,
      52,    53,    54,    56,    58,    59,    61,    62,    63,    66,
      67,    68,    69,    70,    72,    73,    74,    75,    76,    77,
      78,    79,    80,    81,    82,    83,    84,    85,    86,    87,
      88,    90,    91,    93,   111,   112,   113,   116,   119,   120,
      72,    76,    44,    70,    91,    44,    70,    82,    11,     9,
      84,     9,    72,    89,    91,     9,     9,    76,    70,   109,
     110,    82,    63,    72,   114,   115,    87,     0,    64,    60,
      45,     6,    29,    24,    27,    31,    32,    57,    12,    13,
      16,    17,     7,     8,    11,    20,    18,    42,    21,     4,
       5,   104,    63,    70,    14,    15,    10,    89,    14,    19,
     117,     9,    91,    92,    94,    95,    96,    97,    98,    99,
     100,   101,   102,   103,    72,    63,     9,    14,    55,    92,
      95,   105,   106,    22,    65,    14,   117,    73,    77,    78,
      79,    79,    80,    80,    80,    80,    79,    81,    81,    81,
      81,    82,    82,    82,    82,    70,    86,    87,     9,    72,
      89,   117,    89,    90,    43,    92,    14,    19,   118,    10,
      23,    23,    23,    10,    85,   107,   108,    69,   110,    73,
     118,    30,   106,    65,    72,    72,   115,    65,     9,    89,
      65,    10,    10,   118,    92,    33,    25,    26,    27,    63,
      97,    92,   100,    92,   103,    74,    30,   108,    65,    10,
      75,    89,    10,    10,    90,    91,    91,    91,    72,   118,
     118,    51,    75,    10,    28,    65,    73,    63,    72,    65
};

#define yyerrok		(yyerrstatus = 0)
#define yyclearin	(yychar = YYEMPTY)
#define YYEMPTY		(-2)
#define YYEOF		0

#define YYACCEPT	goto yyacceptlab
#define YYABORT		goto yyabortlab
#define YYERROR		goto yyerrorlab


/* Like YYERROR except do call yyerror.  This remains here temporarily
   to ease the transition to the new meaning of YYERROR, for GCC.
   Once GCC version 2 has supplanted version 1, this can go.  */

#define YYFAIL		goto yyerrlab

#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)					\
do								\
  if (yychar == YYEMPTY && yylen == 1)				\
    {								\
      yychar = (Token);						\
      yylval = (Value);						\
      yytoken = YYTRANSLATE (yychar);				\
      YYPOPSTACK (1);						\
      goto yybackup;						\
    }								\
  else								\
    {								\
      yyerror (YY_("syntax error: cannot back up")); \
      YYERROR;							\
    }								\
while (YYID (0))


#define YYTERROR	1
#define YYERRCODE	256


/* YYLLOC_DEFAULT -- Set CURRENT to span from RHS[1] to RHS[N].
   If N is 0, then set CURRENT to the empty location which ends
   the previous symbol: RHS[0] (always defined).  */

#define YYRHSLOC(Rhs, K) ((Rhs)[K])
#ifndef YYLLOC_DEFAULT
# define YYLLOC_DEFAULT(Current, Rhs, N)				\
    do									\
      if (YYID (N))                                                    \
	{								\
	  (Current).first_line   = YYRHSLOC (Rhs, 1).first_line;	\
	  (Current).first_column = YYRHSLOC (Rhs, 1).first_column;	\
	  (Current).last_line    = YYRHSLOC (Rhs, N).last_line;		\
	  (Current).last_column  = YYRHSLOC (Rhs, N).last_column;	\
	}								\
      else								\
	{								\
	  (Current).first_line   = (Current).last_line   =		\
	    YYRHSLOC (Rhs, 0).last_line;				\
	  (Current).first_column = (Current).last_column =		\
	    YYRHSLOC (Rhs, 0).last_column;				\
	}								\
    while (YYID (0))
#endif


/* YY_LOCATION_PRINT -- Print the location on the stream.
   This macro was not mandated originally: define only if we know
   we won't break user code: when these are the locations we know.  */

#ifndef YY_LOCATION_PRINT
# if defined YYLTYPE_IS_TRIVIAL && YYLTYPE_IS_TRIVIAL
#  define YY_LOCATION_PRINT(File, Loc)			\
     fprintf (File, "%d.%d-%d.%d",			\
	      (Loc).first_line, (Loc).first_column,	\
	      (Loc).last_line,  (Loc).last_column)
# else
#  define YY_LOCATION_PRINT(File, Loc) ((void) 0)
# endif
#endif


/* YYLEX -- calling `yylex' with the right arguments.  */

#ifdef YYLEX_PARAM
# define YYLEX yylex (YYLEX_PARAM)
#else
# define YYLEX yylex ()
#endif

/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)			\
do {						\
  if (yydebug)					\
    YYFPRINTF Args;				\
} while (YYID (0))

# define YY_SYMBOL_PRINT(Title, Type, Value, Location)			  \
do {									  \
  if (yydebug)								  \
    {									  \
      YYFPRINTF (stderr, "%s ", Title);					  \
      yy_symbol_print (stderr,						  \
		  Type, Value, Location); \
      YYFPRINTF (stderr, "\n");						  \
    }									  \
} while (YYID (0))


/*--------------------------------.
| Print this symbol on YYOUTPUT.  |
`--------------------------------*/

/*ARGSUSED*/
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static void
yy_symbol_value_print (FILE *yyoutput, int yytype, YYSTYPE const * const yyvaluep, YYLTYPE const * const yylocationp)
#else
static void
yy_symbol_value_print (yyoutput, yytype, yyvaluep, yylocationp)
    FILE *yyoutput;
    int yytype;
    YYSTYPE const * const yyvaluep;
    YYLTYPE const * const yylocationp;
#endif
{
  if (!yyvaluep)
    return;
  YYUSE (yylocationp);
# ifdef YYPRINT
  if (yytype < YYNTOKENS)
    YYPRINT (yyoutput, yytoknum[yytype], *yyvaluep);
# else
  YYUSE (yyoutput);
# endif
  switch (yytype)
    {
      default:
	break;
    }
}


/*--------------------------------.
| Print this symbol on YYOUTPUT.  |
`--------------------------------*/

#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static void
yy_symbol_print (FILE *yyoutput, int yytype, YYSTYPE const * const yyvaluep, YYLTYPE const * const yylocationp)
#else
static void
yy_symbol_print (yyoutput, yytype, yyvaluep, yylocationp)
    FILE *yyoutput;
    int yytype;
    YYSTYPE const * const yyvaluep;
    YYLTYPE const * const yylocationp;
#endif
{
  if (yytype < YYNTOKENS)
    YYFPRINTF (yyoutput, "token %s (", yytname[yytype]);
  else
    YYFPRINTF (yyoutput, "nterm %s (", yytname[yytype]);

  YY_LOCATION_PRINT (yyoutput, *yylocationp);
  YYFPRINTF (yyoutput, ": ");
  yy_symbol_value_print (yyoutput, yytype, yyvaluep, yylocationp);
  YYFPRINTF (yyoutput, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static void
yy_stack_print (yytype_int16 *bottom, yytype_int16 *top)
#else
static void
yy_stack_print (bottom, top)
    yytype_int16 *bottom;
    yytype_int16 *top;
#endif
{
  YYFPRINTF (stderr, "Stack now");
  for (; bottom <= top; ++bottom)
    YYFPRINTF (stderr, " %d", *bottom);
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)				\
do {								\
  if (yydebug)							\
    yy_stack_print ((Bottom), (Top));				\
} while (YYID (0))


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static void
yy_reduce_print (YYSTYPE *yyvsp, YYLTYPE *yylsp, int yyrule)
#else
static void
yy_reduce_print (yyvsp, yylsp, yyrule)
    YYSTYPE *yyvsp;
    YYLTYPE *yylsp;
    int yyrule;
#endif
{
  int yynrhs = yyr2[yyrule];
  int yyi;
  unsigned long int yylno = yyrline[yyrule];
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %lu):\n",
	     yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      fprintf (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr, yyrhs[yyprhs[yyrule] + yyi],
		       &(yyvsp[(yyi + 1) - (yynrhs)])
		       , &(yylsp[(yyi + 1) - (yynrhs)])		       );
      fprintf (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)		\
do {					\
  if (yydebug)				\
    yy_reduce_print (yyvsp, yylsp, Rule); \
} while (YYID (0))

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args)
# define YY_SYMBOL_PRINT(Title, Type, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef	YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif



#if YYERROR_VERBOSE

# ifndef yystrlen
#  if defined __GLIBC__ && defined _STRING_H
#   define yystrlen strlen
#  else
/* Return the length of YYSTR.  */
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static YYSIZE_T
yystrlen (const char *yystr)
#else
static YYSIZE_T
yystrlen (yystr)
    const char *yystr;
#endif
{
  YYSIZE_T yylen;
  for (yylen = 0; yystr[yylen]; yylen++)
    continue;
  return yylen;
}
#  endif
# endif

# ifndef yystpcpy
#  if defined __GLIBC__ && defined _STRING_H && defined _GNU_SOURCE
#   define yystpcpy stpcpy
#  else
/* Copy YYSRC to YYDEST, returning the address of the terminating '\0' in
   YYDEST.  */
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static char *
yystpcpy (char *yydest, const char *yysrc)
#else
static char *
yystpcpy (yydest, yysrc)
    char *yydest;
    const char *yysrc;
#endif
{
  char *yyd = yydest;
  const char *yys = yysrc;

  while ((*yyd++ = *yys++) != '\0')
    continue;

  return yyd - 1;
}
#  endif
# endif

# ifndef yytnamerr
/* Copy to YYRES the contents of YYSTR after stripping away unnecessary
   quotes and backslashes, so that it's suitable for yyerror.  The
   heuristic is that double-quoting is unnecessary unless the string
   contains an apostrophe, a comma, or backslash (other than
   backslash-backslash).  YYSTR is taken from yytname.  If YYRES is
   null, do not copy; instead, return the length of what the result
   would have been.  */
static YYSIZE_T
yytnamerr (char *yyres, const char *yystr)
{
  if (*yystr == '"')
    {
      YYSIZE_T yyn = 0;
      char const *yyp = yystr;

      for (;;)
	switch (*++yyp)
	  {
	  case '\'':
	  case ',':
	    goto do_not_strip_quotes;

	  case '\\':
	    if (*++yyp != '\\')
	      goto do_not_strip_quotes;
	    /* Fall through.  */
	  default:
	    if (yyres)
	      yyres[yyn] = *yyp;
	    yyn++;
	    break;

	  case '"':
	    if (yyres)
	      yyres[yyn] = '\0';
	    return yyn;
	  }
    do_not_strip_quotes: ;
    }

  if (! yyres)
    return yystrlen (yystr);

  return yystpcpy (yyres, yystr) - yyres;
}
# endif

/* Copy into YYRESULT an error message about the unexpected token
   YYCHAR while in state YYSTATE.  Return the number of bytes copied,
   including the terminating null byte.  If YYRESULT is null, do not
   copy anything; just return the number of bytes that would be
   copied.  As a special case, return 0 if an ordinary "syntax error"
   message will do.  Return YYSIZE_MAXIMUM if overflow occurs during
   size calculation.  */
static YYSIZE_T
yysyntax_error (char *yyresult, int yystate, int yychar)
{
  int yyn = yypact[yystate];

  if (! (YYPACT_NINF < yyn && yyn <= YYLAST))
    return 0;
  else
    {
      int yytype = YYTRANSLATE (yychar);
      YYSIZE_T yysize0 = yytnamerr (0, yytname[yytype]);
      YYSIZE_T yysize = yysize0;
      YYSIZE_T yysize1;
      int yysize_overflow = 0;
      enum { YYERROR_VERBOSE_ARGS_MAXIMUM = 5 };
      char const *yyarg[YYERROR_VERBOSE_ARGS_MAXIMUM];
      int yyx;

# if 0
      /* This is so xgettext sees the translatable formats that are
	 constructed on the fly.  */
      YY_("syntax error, unexpected %s");
      YY_("syntax error, unexpected %s, expecting %s");
      YY_("syntax error, unexpected %s, expecting %s or %s");
      YY_("syntax error, unexpected %s, expecting %s or %s or %s");
      YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s");
# endif
      char *yyfmt;
      char const *yyf;
      static char const yyunexpected[] = "syntax error, unexpected %s";
      static char const yyexpecting[] = ", expecting %s";
      static char const yyor[] = " or %s";
      char yyformat[sizeof yyunexpected
		    + sizeof yyexpecting - 1
		    + ((YYERROR_VERBOSE_ARGS_MAXIMUM - 2)
		       * (sizeof yyor - 1))];
      char const *yyprefix = yyexpecting;

      /* Start YYX at -YYN if negative to avoid negative indexes in
	 YYCHECK.  */
      int yyxbegin = yyn < 0 ? -yyn : 0;

      /* Stay within bounds of both yycheck and yytname.  */
      int yychecklim = YYLAST - yyn + 1;
      int yyxend = yychecklim < YYNTOKENS ? yychecklim : YYNTOKENS;
      int yycount = 1;

      yyarg[0] = yytname[yytype];
      yyfmt = yystpcpy (yyformat, yyunexpected);

      for (yyx = yyxbegin; yyx < yyxend; ++yyx)
	if (yycheck[yyx + yyn] == yyx && yyx != YYTERROR)
	  {
	    if (yycount == YYERROR_VERBOSE_ARGS_MAXIMUM)
	      {
		yycount = 1;
		yysize = yysize0;
		yyformat[sizeof yyunexpected - 1] = '\0';
		break;
	      }
	    yyarg[yycount++] = yytname[yyx];
	    yysize1 = yysize + yytnamerr (0, yytname[yyx]);
	    yysize_overflow |= (yysize1 < yysize);
	    yysize = yysize1;
	    yyfmt = yystpcpy (yyfmt, yyprefix);
	    yyprefix = yyor;
	  }

      yyf = YY_(yyformat);
      yysize1 = yysize + yystrlen (yyf);
      yysize_overflow |= (yysize1 < yysize);
      yysize = yysize1;

      if (yysize_overflow)
	return YYSIZE_MAXIMUM;

      if (yyresult)
	{
	  /* Avoid sprintf, as that infringes on the user's name space.
	     Don't have undefined behavior even if the translation
	     produced a string with the wrong number of "%s"s.  */
	  char *yyp = yyresult;
	  int yyi = 0;
	  while ((*yyp = *yyf) != '\0')
	    {
	      if (*yyp == '%' && yyf[1] == 's' && yyi < yycount)
		{
		  yyp += yytnamerr (yyp, yyarg[yyi++]);
		  yyf += 2;
		}
	      else
		{
		  yyp++;
		  yyf++;
		}
	    }
	}
      return yysize;
    }
}
#endif /* YYERROR_VERBOSE */


/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

/*ARGSUSED*/
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
static void
yydestruct (const char *yymsg, int yytype, YYSTYPE *yyvaluep, YYLTYPE *yylocationp)
#else
static void
yydestruct (yymsg, yytype, yyvaluep, yylocationp)
    const char *yymsg;
    int yytype;
    YYSTYPE *yyvaluep;
    YYLTYPE *yylocationp;
#endif
{
  YYUSE (yyvaluep);
  YYUSE (yylocationp);

  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yytype, yyvaluep, yylocationp);

  switch (yytype)
    {

      default:
	break;
    }
}


/* Prevent warnings from -Wmissing-prototypes.  */

#ifdef YYPARSE_PARAM
#if defined __STDC__ || defined __cplusplus
int yyparse (void *YYPARSE_PARAM);
#else
int yyparse ();
#endif
#else /* ! YYPARSE_PARAM */
#if defined __STDC__ || defined __cplusplus
int yyparse (void);
#else
int yyparse ();
#endif
#endif /* ! YYPARSE_PARAM */



/* The look-ahead symbol.  */
int yychar;

/* The semantic value of the look-ahead symbol.  */
YYSTYPE yylval;

/* Number of syntax errors so far.  */
int yynerrs;
/* Location data for the look-ahead symbol.  */
YYLTYPE yylloc;



/*----------.
| yyparse.  |
`----------*/

#ifdef YYPARSE_PARAM
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
int
yyparse (void *YYPARSE_PARAM)
#else
int
yyparse (YYPARSE_PARAM)
    void *YYPARSE_PARAM;
#endif
#else /* ! YYPARSE_PARAM */
#if (defined __STDC__ || defined __C99__FUNC__ \
     || defined __cplusplus || defined _MSC_VER)
int
yyparse (void)
#else
int
yyparse ()

#endif
#endif
{
  
  int yystate;
  int yyn;
  int yyresult;
  /* Number of tokens to shift before error messages enabled.  */
  int yyerrstatus;
  /* Look-ahead token as an internal (translated) token number.  */
  int yytoken = 0;
#if YYERROR_VERBOSE
  /* Buffer for error messages, and its allocated size.  */
  char yymsgbuf[128];
  char *yymsg = yymsgbuf;
  YYSIZE_T yymsg_alloc = sizeof yymsgbuf;
#endif

  /* Three stacks and their tools:
     `yyss': related to states,
     `yyvs': related to semantic values,
     `yyls': related to locations.

     Refer to the stacks thru separate pointers, to allow yyoverflow
     to reallocate them elsewhere.  */

  /* The state stack.  */
  yytype_int16 yyssa[YYINITDEPTH];
  yytype_int16 *yyss = yyssa;
  yytype_int16 *yyssp;

  /* The semantic value stack.  */
  YYSTYPE yyvsa[YYINITDEPTH];
  YYSTYPE *yyvs = yyvsa;
  YYSTYPE *yyvsp;

  /* The location stack.  */
  YYLTYPE yylsa[YYINITDEPTH];
  YYLTYPE *yyls = yylsa;
  YYLTYPE *yylsp;
  /* The locations where the error started and ended.  */
  YYLTYPE yyerror_range[2];

#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N), yylsp -= (N))

  YYSIZE_T yystacksize = YYINITDEPTH;

  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;
  YYLTYPE yyloc;

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yystate = 0;
  yyerrstatus = 0;
  yynerrs = 0;
  yychar = YYEMPTY;		/* Cause a token to be read.  */

  /* Initialize stack pointers.
     Waste one element of value and location stack
     so that they stay on the same level as the state stack.
     The wasted elements are never initialized.  */

  yyssp = yyss;
  yyvsp = yyvs;
  yylsp = yyls;
#if defined YYLTYPE_IS_TRIVIAL && YYLTYPE_IS_TRIVIAL
  /* Initialize the default location before parsing starts.  */
  yylloc.first_line   = yylloc.last_line   = 1;
  yylloc.first_column = yylloc.last_column = 0;
#endif

  goto yysetstate;

/*------------------------------------------------------------.
| yynewstate -- Push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
 yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;

 yysetstate:
  *yyssp = yystate;

  if (yyss + yystacksize - 1 <= yyssp)
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYSIZE_T yysize = yyssp - yyss + 1;

#ifdef yyoverflow
      {
	/* Give user a chance to reallocate the stack.  Use copies of
	   these so that the &'s don't force the real ones into
	   memory.  */
	YYSTYPE *yyvs1 = yyvs;
	yytype_int16 *yyss1 = yyss;
	YYLTYPE *yyls1 = yyls;

	/* Each stack pointer address is followed by the size of the
	   data in use in that stack, in bytes.  This used to be a
	   conditional around just the two extra args, but that might
	   be undefined if yyoverflow is a macro.  */
	yyoverflow (YY_("memory exhausted"),
		    &yyss1, yysize * sizeof (*yyssp),
		    &yyvs1, yysize * sizeof (*yyvsp),
		    &yyls1, yysize * sizeof (*yylsp),
		    &yystacksize);
	yyls = yyls1;
	yyss = yyss1;
	yyvs = yyvs1;
      }
#else /* no yyoverflow */
# ifndef YYSTACK_RELOCATE
      goto yyexhaustedlab;
# else
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
	goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
	yystacksize = YYMAXDEPTH;

      {
	yytype_int16 *yyss1 = yyss;
	union yyalloc *yyptr =
	  (union yyalloc *) YYSTACK_ALLOC (YYSTACK_BYTES (yystacksize));
	if (! yyptr)
	  goto yyexhaustedlab;
	YYSTACK_RELOCATE (yyss);
	YYSTACK_RELOCATE (yyvs);
	YYSTACK_RELOCATE (yyls);
#  undef YYSTACK_RELOCATE
	if (yyss1 != yyssa)
	  YYSTACK_FREE (yyss1);
      }
# endif
#endif /* no yyoverflow */

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;
      yylsp = yyls + yysize - 1;

      YYDPRINTF ((stderr, "Stack size increased to %lu\n",
		  (unsigned long int) yystacksize));

      if (yyss + yystacksize - 1 <= yyssp)
	YYABORT;
    }

  YYDPRINTF ((stderr, "Entering state %d\n", yystate));

  goto yybackup;

/*-----------.
| yybackup.  |
`-----------*/
yybackup:

  /* Do appropriate processing given the current state.  Read a
     look-ahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to look-ahead token.  */
  yyn = yypact[yystate];
  if (yyn == YYPACT_NINF)
    goto yydefault;

  /* Not known => get a look-ahead token if don't already have one.  */

  /* YYCHAR is either YYEMPTY or YYEOF or a valid look-ahead symbol.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token: "));
      yychar = YYLEX;
    }

  if (yychar <= YYEOF)
    {
      yychar = yytoken = YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yyn == 0 || yyn == YYTABLE_NINF)
	goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  if (yyn == YYFINAL)
    YYACCEPT;

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the look-ahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);

  /* Discard the shifted token unless it is eof.  */
  if (yychar != YYEOF)
    yychar = YYEMPTY;

  yystate = yyn;
  *++yyvsp = yylval;
  *++yylsp = yylloc;
  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- Do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     `$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];

  /* Default location.  */
  YYLLOC_DEFAULT (yyloc, (yylsp - yylen), yylen);
  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
        case 2:
#line 1964 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 3:
#line 1965 "rholang_mercury.y"
    { (yyval.proc_) = make_PPar((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 4:
#line 1967 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 5:
#line 1968 "rholang_mercury.y"
    { (yyval.proc_) = make_PIf((yyvsp[(3) - (5)].proc_), (yyvsp[(5) - (5)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 6:
#line 1969 "rholang_mercury.y"
    { (yyval.proc_) = make_PIfElse((yyvsp[(3) - (7)].proc_), (yyvsp[(5) - (7)].proc_), (yyvsp[(7) - (7)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 7:
#line 1970 "rholang_mercury.y"
    { (yyval.proc_) = make_PNew((yyvsp[(2) - (4)].listnamedecl_), (yyvsp[(4) - (4)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 8:
#line 1972 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 9:
#line 1973 "rholang_mercury.y"
    { (yyval.proc_) = make_PContr((yyvsp[(2) - (10)].name_), (yyvsp[(4) - (10)].listname_), (yyvsp[(5) - (10)].nameremainder_), (yyvsp[(9) - (10)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 10:
#line 1974 "rholang_mercury.y"
    { (yyval.proc_) = make_PInput((yyvsp[(3) - (7)].receipt_), (yyvsp[(6) - (7)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 11:
#line 1975 "rholang_mercury.y"
    { (yyval.proc_) = make_PChoice((yyvsp[(3) - (4)].listbranch_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 12:
#line 1976 "rholang_mercury.y"
    { (yyval.proc_) = make_PMatch((yyvsp[(2) - (5)].proc_), (yyvsp[(4) - (5)].listcase_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 13:
#line 1977 "rholang_mercury.y"
    { (yyval.proc_) = make_PBundle((yyvsp[(1) - (4)].bundle_), (yyvsp[(3) - (4)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 14:
#line 1979 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 15:
#line 1980 "rholang_mercury.y"
    { (yyval.proc_) = make_PSend((yyvsp[(1) - (5)].name_), (yyvsp[(2) - (5)].send_), (yyvsp[(4) - (5)].listproc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 16:
#line 1982 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 17:
#line 1983 "rholang_mercury.y"
    { (yyval.proc_) = make_POr((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 18:
#line 1985 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 19:
#line 1986 "rholang_mercury.y"
    { (yyval.proc_) = make_PAnd((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 20:
#line 1988 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 21:
#line 1989 "rholang_mercury.y"
    { (yyval.proc_) = make_PMatches((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 22:
#line 1990 "rholang_mercury.y"
    { (yyval.proc_) = make_PEq((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 23:
#line 1991 "rholang_mercury.y"
    { (yyval.proc_) = make_PNeq((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 24:
#line 1993 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 25:
#line 1994 "rholang_mercury.y"
    { (yyval.proc_) = make_PLt((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 26:
#line 1995 "rholang_mercury.y"
    { (yyval.proc_) = make_PLte((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 27:
#line 1996 "rholang_mercury.y"
    { (yyval.proc_) = make_PGt((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 28:
#line 1997 "rholang_mercury.y"
    { (yyval.proc_) = make_PGte((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 29:
#line 1999 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 30:
#line 2000 "rholang_mercury.y"
    { (yyval.proc_) = make_PAdd((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 31:
#line 2001 "rholang_mercury.y"
    { (yyval.proc_) = make_PMinus((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 32:
#line 2002 "rholang_mercury.y"
    { (yyval.proc_) = make_PPlusPlus((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 33:
#line 2003 "rholang_mercury.y"
    { (yyval.proc_) = make_PMinusMinus((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 34:
#line 2005 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 35:
#line 2006 "rholang_mercury.y"
    { (yyval.proc_) = make_PMult((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 36:
#line 2007 "rholang_mercury.y"
    { (yyval.proc_) = make_PDiv((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 37:
#line 2008 "rholang_mercury.y"
    { (yyval.proc_) = make_PMod((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 38:
#line 2009 "rholang_mercury.y"
    { (yyval.proc_) = make_PPercentPercent((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 39:
#line 2011 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 40:
#line 2012 "rholang_mercury.y"
    { (yyval.proc_) = make_PNot((yyvsp[(2) - (2)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 41:
#line 2013 "rholang_mercury.y"
    { (yyval.proc_) = make_PNeg((yyvsp[(2) - (2)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 42:
#line 2015 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 43:
#line 2016 "rholang_mercury.y"
    { (yyval.proc_) = make_PMethod((yyvsp[(1) - (6)].proc_), (yyvsp[(3) - (6)]._string), (yyvsp[(5) - (6)].listproc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 44:
#line 2017 "rholang_mercury.y"
    { (yyval.proc_) = make_PExprs((yyvsp[(2) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 45:
#line 2019 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 46:
#line 2020 "rholang_mercury.y"
    { (yyval.proc_) = make_PEval((yyvsp[(2) - (2)].name_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 47:
#line 2022 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 48:
#line 2023 "rholang_mercury.y"
    { (yyval.proc_) = make_PVarRef((yyvsp[(1) - (2)].varrefkind_), (yyvsp[(2) - (2)]._string)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 49:
#line 2024 "rholang_mercury.y"
    { (yyval.proc_) = make_PDisjunction((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 50:
#line 2026 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 51:
#line 2027 "rholang_mercury.y"
    { (yyval.proc_) = make_PConjunction((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 52:
#line 2029 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(1) - (1)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 53:
#line 2030 "rholang_mercury.y"
    { (yyval.proc_) = make_PNegation((yyvsp[(2) - (2)].proc_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 54:
#line 2032 "rholang_mercury.y"
    { (yyval.proc_) = (yyvsp[(2) - (3)].proc_); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 55:
#line 2033 "rholang_mercury.y"
    { (yyval.proc_) = make_PGround((yyvsp[(1) - (1)].ground_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 56:
#line 2034 "rholang_mercury.y"
    { (yyval.proc_) = make_PCollect((yyvsp[(1) - (1)].collection_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 57:
#line 2035 "rholang_mercury.y"
    { (yyval.proc_) = make_PVar((yyvsp[(1) - (1)].procvar_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 58:
#line 2036 "rholang_mercury.y"
    { (yyval.proc_) = make_PNil(); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 59:
#line 2037 "rholang_mercury.y"
    { (yyval.proc_) = make_PSimpleType((yyvsp[(1) - (1)].simpletype_)); YY_RESULT_Proc_= (yyval.proc_); ;}
    break;

  case 60:
#line 2039 "rholang_mercury.y"
    { (yyval.listproc_) = 0; YY_RESULT_ListProc_= (yyval.listproc_); ;}
    break;

  case 61:
#line 2040 "rholang_mercury.y"
    { (yyval.listproc_) = make_ListProc((yyvsp[(1) - (1)].proc_), 0); YY_RESULT_ListProc_= (yyval.listproc_); ;}
    break;

  case 62:
#line 2041 "rholang_mercury.y"
    { (yyval.listproc_) = make_ListProc((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].listproc_)); YY_RESULT_ListProc_= (yyval.listproc_); ;}
    break;

  case 63:
#line 2043 "rholang_mercury.y"
    { (yyval.procvar_) = make_ProcVarWildcard(); YY_RESULT_ProcVar_= (yyval.procvar_); ;}
    break;

  case 64:
#line 2044 "rholang_mercury.y"
    { (yyval.procvar_) = make_ProcVarVar((yyvsp[(1) - (1)]._string)); YY_RESULT_ProcVar_= (yyval.procvar_); ;}
    break;

  case 65:
#line 2046 "rholang_mercury.y"
    { (yyval.name_) = make_NameWildcard(); YY_RESULT_Name_= (yyval.name_); ;}
    break;

  case 66:
#line 2047 "rholang_mercury.y"
    { (yyval.name_) = make_NameVar((yyvsp[(1) - (1)]._string)); YY_RESULT_Name_= (yyval.name_); ;}
    break;

  case 67:
#line 2048 "rholang_mercury.y"
    { (yyval.name_) = make_NameQuote((yyvsp[(2) - (2)].proc_)); YY_RESULT_Name_= (yyval.name_); ;}
    break;

  case 68:
#line 2050 "rholang_mercury.y"
    { (yyval.listname_) = 0; YY_RESULT_ListName_= (yyval.listname_); ;}
    break;

  case 69:
#line 2051 "rholang_mercury.y"
    { (yyval.listname_) = make_ListName((yyvsp[(1) - (1)].name_), 0); YY_RESULT_ListName_= (yyval.listname_); ;}
    break;

  case 70:
#line 2052 "rholang_mercury.y"
    { (yyval.listname_) = make_ListName((yyvsp[(1) - (3)].name_), (yyvsp[(3) - (3)].listname_)); YY_RESULT_ListName_= (yyval.listname_); ;}
    break;

  case 71:
#line 2054 "rholang_mercury.y"
    { (yyval.bundle_) = make_BundleWrite(); YY_RESULT_Bundle_= (yyval.bundle_); ;}
    break;

  case 72:
#line 2055 "rholang_mercury.y"
    { (yyval.bundle_) = make_BundleRead(); YY_RESULT_Bundle_= (yyval.bundle_); ;}
    break;

  case 73:
#line 2056 "rholang_mercury.y"
    { (yyval.bundle_) = make_BundleEquiv(); YY_RESULT_Bundle_= (yyval.bundle_); ;}
    break;

  case 74:
#line 2057 "rholang_mercury.y"
    { (yyval.bundle_) = make_BundleReadWrite(); YY_RESULT_Bundle_= (yyval.bundle_); ;}
    break;

  case 75:
#line 2059 "rholang_mercury.y"
    { (yyval.receipt_) = make_ReceiptLinear((yyvsp[(1) - (1)].receiptlinearimpl_)); YY_RESULT_Receipt_= (yyval.receipt_); ;}
    break;

  case 76:
#line 2060 "rholang_mercury.y"
    { (yyval.receipt_) = make_ReceiptRepeated((yyvsp[(1) - (1)].receiptrepeatedimpl_)); YY_RESULT_Receipt_= (yyval.receipt_); ;}
    break;

  case 77:
#line 2061 "rholang_mercury.y"
    { (yyval.receipt_) = make_ReceiptPeek((yyvsp[(1) - (1)].receiptpeekimpl_)); YY_RESULT_Receipt_= (yyval.receipt_); ;}
    break;

  case 78:
#line 2063 "rholang_mercury.y"
    { (yyval.receiptlinearimpl_) = make_LinearSimple((yyvsp[(1) - (1)].listlinearbind_)); YY_RESULT_ReceiptLinearImpl_= (yyval.receiptlinearimpl_); ;}
    break;

  case 79:
#line 2065 "rholang_mercury.y"
    { (yyval.linearbind_) = make_LinearBindImpl((yyvsp[(1) - (4)].listname_), (yyvsp[(2) - (4)].nameremainder_), (yyvsp[(4) - (4)].name_)); YY_RESULT_LinearBind_= (yyval.linearbind_); ;}
    break;

  case 80:
#line 2067 "rholang_mercury.y"
    { (yyval.listlinearbind_) = make_ListLinearBind((yyvsp[(1) - (1)].linearbind_), 0); YY_RESULT_ListLinearBind_= (yyval.listlinearbind_); ;}
    break;

  case 81:
#line 2068 "rholang_mercury.y"
    { (yyval.listlinearbind_) = make_ListLinearBind((yyvsp[(1) - (3)].linearbind_), (yyvsp[(3) - (3)].listlinearbind_)); YY_RESULT_ListLinearBind_= (yyval.listlinearbind_); ;}
    break;

  case 82:
#line 2070 "rholang_mercury.y"
    { (yyval.receiptrepeatedimpl_) = make_RepeatedSimple((yyvsp[(1) - (1)].listrepeatedbind_)); YY_RESULT_ReceiptRepeatedImpl_= (yyval.receiptrepeatedimpl_); ;}
    break;

  case 83:
#line 2072 "rholang_mercury.y"
    { (yyval.repeatedbind_) = make_RepeatedBindImpl((yyvsp[(1) - (4)].listname_), (yyvsp[(2) - (4)].nameremainder_), (yyvsp[(4) - (4)].name_)); YY_RESULT_RepeatedBind_= (yyval.repeatedbind_); ;}
    break;

  case 84:
#line 2074 "rholang_mercury.y"
    { (yyval.listrepeatedbind_) = make_ListRepeatedBind((yyvsp[(1) - (1)].repeatedbind_), 0); YY_RESULT_ListRepeatedBind_= (yyval.listrepeatedbind_); ;}
    break;

  case 85:
#line 2075 "rholang_mercury.y"
    { (yyval.listrepeatedbind_) = make_ListRepeatedBind((yyvsp[(1) - (3)].repeatedbind_), (yyvsp[(3) - (3)].listrepeatedbind_)); YY_RESULT_ListRepeatedBind_= (yyval.listrepeatedbind_); ;}
    break;

  case 86:
#line 2077 "rholang_mercury.y"
    { (yyval.receiptpeekimpl_) = make_PeekSimple((yyvsp[(1) - (1)].listpeekbind_)); YY_RESULT_ReceiptPeekImpl_= (yyval.receiptpeekimpl_); ;}
    break;

  case 87:
#line 2079 "rholang_mercury.y"
    { (yyval.peekbind_) = make_PeekBindImpl((yyvsp[(1) - (4)].listname_), (yyvsp[(2) - (4)].nameremainder_), (yyvsp[(4) - (4)].name_)); YY_RESULT_PeekBind_= (yyval.peekbind_); ;}
    break;

  case 88:
#line 2081 "rholang_mercury.y"
    { (yyval.listpeekbind_) = make_ListPeekBind((yyvsp[(1) - (1)].peekbind_), 0); YY_RESULT_ListPeekBind_= (yyval.listpeekbind_); ;}
    break;

  case 89:
#line 2082 "rholang_mercury.y"
    { (yyval.listpeekbind_) = make_ListPeekBind((yyvsp[(1) - (3)].peekbind_), (yyvsp[(3) - (3)].listpeekbind_)); YY_RESULT_ListPeekBind_= (yyval.listpeekbind_); ;}
    break;

  case 90:
#line 2084 "rholang_mercury.y"
    { (yyval.send_) = make_SendSingle(); YY_RESULT_Send_= (yyval.send_); ;}
    break;

  case 91:
#line 2085 "rholang_mercury.y"
    { (yyval.send_) = make_SendMultiple(); YY_RESULT_Send_= (yyval.send_); ;}
    break;

  case 92:
#line 2087 "rholang_mercury.y"
    { (yyval.branch_) = make_BranchImpl((yyvsp[(1) - (3)].receiptlinearimpl_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Branch_= (yyval.branch_); ;}
    break;

  case 93:
#line 2089 "rholang_mercury.y"
    { (yyval.listbranch_) = make_ListBranch((yyvsp[(1) - (1)].branch_), 0); YY_RESULT_ListBranch_= (yyval.listbranch_); ;}
    break;

  case 94:
#line 2090 "rholang_mercury.y"
    { (yyval.listbranch_) = make_ListBranch((yyvsp[(1) - (2)].branch_), (yyvsp[(2) - (2)].listbranch_)); YY_RESULT_ListBranch_= (yyval.listbranch_); ;}
    break;

  case 95:
#line 2092 "rholang_mercury.y"
    { (yyval.case_) = make_CaseImpl((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_Case_= (yyval.case_); ;}
    break;

  case 96:
#line 2094 "rholang_mercury.y"
    { (yyval.listcase_) = make_ListCase((yyvsp[(1) - (1)].case_), 0); YY_RESULT_ListCase_= (yyval.listcase_); ;}
    break;

  case 97:
#line 2095 "rholang_mercury.y"
    { (yyval.listcase_) = make_ListCase((yyvsp[(1) - (2)].case_), (yyvsp[(2) - (2)].listcase_)); YY_RESULT_ListCase_= (yyval.listcase_); ;}
    break;

  case 98:
#line 2097 "rholang_mercury.y"
    { (yyval.namedecl_) = make_NameDeclSimpl((yyvsp[(1) - (1)]._string)); YY_RESULT_NameDecl_= (yyval.namedecl_); ;}
    break;

  case 99:
#line 2098 "rholang_mercury.y"
    { (yyval.namedecl_) = make_NameDeclUrn((yyvsp[(1) - (4)]._string), (yyvsp[(3) - (4)]._string)); YY_RESULT_NameDecl_= (yyval.namedecl_); ;}
    break;

  case 100:
#line 2100 "rholang_mercury.y"
    { (yyval.listnamedecl_) = make_ListNameDecl((yyvsp[(1) - (1)].namedecl_), 0); YY_RESULT_ListNameDecl_= (yyval.listnamedecl_); ;}
    break;

  case 101:
#line 2101 "rholang_mercury.y"
    { (yyval.listnamedecl_) = make_ListNameDecl((yyvsp[(1) - (3)].namedecl_), (yyvsp[(3) - (3)].listnamedecl_)); YY_RESULT_ListNameDecl_= (yyval.listnamedecl_); ;}
    break;

  case 102:
#line 2103 "rholang_mercury.y"
    { (yyval.boolliteral_) = make_BoolTrue(); YY_RESULT_BoolLiteral_= (yyval.boolliteral_); ;}
    break;

  case 103:
#line 2104 "rholang_mercury.y"
    { (yyval.boolliteral_) = make_BoolFalse(); YY_RESULT_BoolLiteral_= (yyval.boolliteral_); ;}
    break;

  case 104:
#line 2106 "rholang_mercury.y"
    { (yyval.ground_) = make_GroundBool((yyvsp[(1) - (1)].boolliteral_)); YY_RESULT_Ground_= (yyval.ground_); ;}
    break;

  case 105:
#line 2107 "rholang_mercury.y"
    { (yyval.ground_) = make_GroundInt((yyvsp[(1) - (1)]._string)); YY_RESULT_Ground_= (yyval.ground_); ;}
    break;

  case 106:
#line 2108 "rholang_mercury.y"
    { (yyval.ground_) = make_GroundString((yyvsp[(1) - (1)]._string)); YY_RESULT_Ground_= (yyval.ground_); ;}
    break;

  case 107:
#line 2109 "rholang_mercury.y"
    { (yyval.ground_) = make_GroundUri((yyvsp[(1) - (1)]._string)); YY_RESULT_Ground_= (yyval.ground_); ;}
    break;

  case 108:
#line 2111 "rholang_mercury.y"
    { (yyval.collection_) = make_CollectList((yyvsp[(2) - (4)].listproc_), (yyvsp[(3) - (4)].procremainder_)); YY_RESULT_Collection_= (yyval.collection_); ;}
    break;

  case 109:
#line 2112 "rholang_mercury.y"
    { (yyval.collection_) = make_CollectTuple((yyvsp[(1) - (1)].tuple_)); YY_RESULT_Collection_= (yyval.collection_); ;}
    break;

  case 110:
#line 2113 "rholang_mercury.y"
    { (yyval.collection_) = make_CollectSet((yyvsp[(3) - (5)].listproc_), (yyvsp[(4) - (5)].procremainder_)); YY_RESULT_Collection_= (yyval.collection_); ;}
    break;

  case 111:
#line 2114 "rholang_mercury.y"
    { (yyval.collection_) = make_CollectMap((yyvsp[(2) - (4)].listkeyvaluepair_), (yyvsp[(3) - (4)].procremainder_)); YY_RESULT_Collection_= (yyval.collection_); ;}
    break;

  case 112:
#line 2116 "rholang_mercury.y"
    { (yyval.keyvaluepair_) = make_KeyValuePairImpl((yyvsp[(1) - (3)].proc_), (yyvsp[(3) - (3)].proc_)); YY_RESULT_KeyValuePair_= (yyval.keyvaluepair_); ;}
    break;

  case 113:
#line 2118 "rholang_mercury.y"
    { (yyval.listkeyvaluepair_) = 0; YY_RESULT_ListKeyValuePair_= (yyval.listkeyvaluepair_); ;}
    break;

  case 114:
#line 2119 "rholang_mercury.y"
    { (yyval.listkeyvaluepair_) = make_ListKeyValuePair((yyvsp[(1) - (1)].keyvaluepair_), 0); YY_RESULT_ListKeyValuePair_= (yyval.listkeyvaluepair_); ;}
    break;

  case 115:
#line 2120 "rholang_mercury.y"
    { (yyval.listkeyvaluepair_) = make_ListKeyValuePair((yyvsp[(1) - (3)].keyvaluepair_), (yyvsp[(3) - (3)].listkeyvaluepair_)); YY_RESULT_ListKeyValuePair_= (yyval.listkeyvaluepair_); ;}
    break;

  case 116:
#line 2122 "rholang_mercury.y"
    { (yyval.tuple_) = make_TupleSingle((yyvsp[(2) - (3)].proc_)); YY_RESULT_Tuple_= (yyval.tuple_); ;}
    break;

  case 117:
#line 2123 "rholang_mercury.y"
    { (yyval.tuple_) = make_TupleMultiple((yyvsp[(2) - (5)].proc_), (yyvsp[(4) - (5)].listproc_)); YY_RESULT_Tuple_= (yyval.tuple_); ;}
    break;

  case 118:
#line 2125 "rholang_mercury.y"
    { (yyval.procremainder_) = make_ProcRemainderVar((yyvsp[(2) - (2)].procvar_)); YY_RESULT_ProcRemainder_= (yyval.procremainder_); ;}
    break;

  case 119:
#line 2126 "rholang_mercury.y"
    { (yyval.procremainder_) = make_ProcRemainderEmpty(); YY_RESULT_ProcRemainder_= (yyval.procremainder_); ;}
    break;

  case 120:
#line 2128 "rholang_mercury.y"
    { (yyval.nameremainder_) = make_NameRemainderVar((yyvsp[(3) - (3)].procvar_)); YY_RESULT_NameRemainder_= (yyval.nameremainder_); ;}
    break;

  case 121:
#line 2129 "rholang_mercury.y"
    { (yyval.nameremainder_) = make_NameRemainderEmpty(); YY_RESULT_NameRemainder_= (yyval.nameremainder_); ;}
    break;

  case 122:
#line 2131 "rholang_mercury.y"
    { (yyval.varrefkind_) = make_VarRefKindProc(); YY_RESULT_VarRefKind_= (yyval.varrefkind_); ;}
    break;

  case 123:
#line 2132 "rholang_mercury.y"
    { (yyval.varrefkind_) = make_VarRefKindName(); YY_RESULT_VarRefKind_= (yyval.varrefkind_); ;}
    break;

  case 124:
#line 2134 "rholang_mercury.y"
    { (yyval.simpletype_) = make_SimpleTypeBool(); YY_RESULT_SimpleType_= (yyval.simpletype_); ;}
    break;

  case 125:
#line 2135 "rholang_mercury.y"
    { (yyval.simpletype_) = make_SimpleTypeInt(); YY_RESULT_SimpleType_= (yyval.simpletype_); ;}
    break;

  case 126:
#line 2136 "rholang_mercury.y"
    { (yyval.simpletype_) = make_SimpleTypeString(); YY_RESULT_SimpleType_= (yyval.simpletype_); ;}
    break;

  case 127:
#line 2137 "rholang_mercury.y"
    { (yyval.simpletype_) = make_SimpleTypeUri(); YY_RESULT_SimpleType_= (yyval.simpletype_); ;}
    break;

  case 128:
#line 2138 "rholang_mercury.y"
    { (yyval.simpletype_) = make_SimpleTypeByteArray(); YY_RESULT_SimpleType_= (yyval.simpletype_); ;}
    break;


/* Line 1267 of yacc.c.  */
#line 4232 "Parser.c"
      default: break;
    }
  YY_SYMBOL_PRINT ("-> $$ =", yyr1[yyn], &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);

  *++yyvsp = yyval;
  *++yylsp = yyloc;

  /* Now `shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */

  yyn = yyr1[yyn];

  yystate = yypgoto[yyn - YYNTOKENS] + *yyssp;
  if (0 <= yystate && yystate <= YYLAST && yycheck[yystate] == *yyssp)
    yystate = yytable[yystate];
  else
    yystate = yydefgoto[yyn - YYNTOKENS];

  goto yynewstate;


/*------------------------------------.
| yyerrlab -- here on detecting error |
`------------------------------------*/
yyerrlab:
  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
#if ! YYERROR_VERBOSE
      yyerror (YY_("syntax error"));
#else
      {
	YYSIZE_T yysize = yysyntax_error (0, yystate, yychar);
	if (yymsg_alloc < yysize && yymsg_alloc < YYSTACK_ALLOC_MAXIMUM)
	  {
	    YYSIZE_T yyalloc = 2 * yysize;
	    if (! (yysize <= yyalloc && yyalloc <= YYSTACK_ALLOC_MAXIMUM))
	      yyalloc = YYSTACK_ALLOC_MAXIMUM;
	    if (yymsg != yymsgbuf)
	      YYSTACK_FREE (yymsg);
	    yymsg = (char *) YYSTACK_ALLOC (yyalloc);
	    if (yymsg)
	      yymsg_alloc = yyalloc;
	    else
	      {
		yymsg = yymsgbuf;
		yymsg_alloc = sizeof yymsgbuf;
	      }
	  }

	if (0 < yysize && yysize <= yymsg_alloc)
	  {
	    (void) yysyntax_error (yymsg, yystate, yychar);
	    yyerror (yymsg);
	  }
	else
	  {
	    yyerror (YY_("syntax error"));
	    if (yysize != 0)
	      goto yyexhaustedlab;
	  }
      }
#endif
    }

  yyerror_range[0] = yylloc;

  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse look-ahead token after an
	 error, discard it.  */

      if (yychar <= YYEOF)
	{
	  /* Return failure if at end of input.  */
	  if (yychar == YYEOF)
	    YYABORT;
	}
      else
	{
	  yydestruct ("Error: discarding",
		      yytoken, &yylval, &yylloc);
	  yychar = YYEMPTY;
	}
    }

  /* Else will try to reuse look-ahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:

  /* Pacify compilers like GCC when the user code never invokes
     YYERROR and the label yyerrorlab therefore never appears in user
     code.  */
  if (/*CONSTCOND*/ 0)
     goto yyerrorlab;

  yyerror_range[0] = yylsp[1-yylen];
  /* Do not reclaim the symbols of the rule which action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;	/* Each real token shifted decrements this.  */

  for (;;)
    {
      yyn = yypact[yystate];
      if (yyn != YYPACT_NINF)
	{
	  yyn += YYTERROR;
	  if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == YYTERROR)
	    {
	      yyn = yytable[yyn];
	      if (0 < yyn)
		break;
	    }
	}

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
	YYABORT;

      yyerror_range[0] = *yylsp;
      yydestruct ("Error: popping",
		  yystos[yystate], yyvsp, yylsp);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  if (yyn == YYFINAL)
    YYACCEPT;

  *++yyvsp = yylval;

  yyerror_range[1] = yylloc;
  /* Using YYLLOC is tempting, but would change the location of
     the look-ahead.  YYLOC is available though.  */
  YYLLOC_DEFAULT (yyloc, (yyerror_range - 1), 2);
  *++yylsp = yyloc;

  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", yystos[yyn], yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturn;

/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturn;

#ifndef yyoverflow
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (YY_("memory exhausted"));
  yyresult = 2;
  /* Fall through.  */
#endif

yyreturn:
  if (yychar != YYEOF && yychar != YYEMPTY)
     yydestruct ("Cleanup: discarding lookahead",
		 yytoken, &yylval, &yylloc);
  /* Do not reclaim the symbols of the rule which action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
		  yystos[*yyssp], yyvsp, yylsp);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif
#if YYERROR_VERBOSE
  if (yymsg != yymsgbuf)
    YYSTACK_FREE (yymsg);
#endif
  /* Make sure YYID is used.  */
  return YYID (yyresult);
}


#line 2141 "rholang_mercury.y"


void yyerror(const char *str)
{
  extern char *rholang_mercury_text;
  fprintf(stderr,"error: %d,%d: %s at %s\n",
  rholang_mercury_lloc.first_line, rholang_mercury_lloc.first_column, str, rholang_mercury_text);
}


