/* A Bison parser, made by GNU Bison 3.5.1.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

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

/* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */

#ifndef YY_RHOLANG_MERCURY_BISON_H_INCLUDED
# define YY_RHOLANG_MERCURY_BISON_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 1
#endif
#if YYDEBUG
extern int rholang_mercury_debug;
#endif

/* Token type.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
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

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 166 "rholang_mercury.y"

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

#line 168 "Bison.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif

/* Location type.  */
#if ! defined YYLTYPE && ! defined YYLTYPE_IS_DECLARED
typedef struct YYLTYPE YYLTYPE;
struct YYLTYPE
{
  int first_line;
  int first_column;
  int last_line;
  int last_column;
};
# define YYLTYPE_IS_DECLARED 1
# define YYLTYPE_IS_TRIVIAL 1
#endif



int rholang_mercury_parse (yyscan_t scanner, YYSTYPE *result);

#endif /* !YY_RHOLANG_MERCURY_BISON_H_INCLUDED  */
