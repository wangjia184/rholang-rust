/* A Bison parser, made by GNU Bison 3.5.1.  */

/* Bison implementation for Yacc-like parsers in C

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

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */

/* Identify Bison output.  */
#define YYBISON 1

/* Bison version.  */
#define YYBISON_VERSION "3.5.1"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 1

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1


/* Substitute the variable and function names.  */
#define yyparse         rholang_mercury_parse
#define yylex           rholang_mercury_lex
#define yyerror         rholang_mercury_error
#define yydebug         rholang_mercury_debug
#define yynerrs         rholang_mercury_nerrs

/* First part of user prologue.  */
#line 17 "rholang_mercury.y"

/* Begin C preamble code */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "Absyn.h"

#define YYMAXDEPTH 10000000

/* The type yyscan_t is defined by flex, but we need it in the parser already. */
#ifndef YY_TYPEDEF_YY_SCANNER_T
#define YY_TYPEDEF_YY_SCANNER_T
typedef void* yyscan_t;
#endif

typedef struct rholang_mercury__buffer_state *YY_BUFFER_STATE;
YY_BUFFER_STATE rholang_mercury__scan_string(const char *str, yyscan_t scanner);
void rholang_mercury__delete_buffer(YY_BUFFER_STATE buf, yyscan_t scanner);

extern void rholang_mercury_lex_destroy(yyscan_t scanner);
extern char* rholang_mercury_get_text(yyscan_t scanner);

extern yyscan_t rholang_mercury__init_lexer(FILE * inp);

/* List reversal functions. */
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

/* End C preamble code */

#line 222 "Parser.c"

# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif

/* Enabling verbose error messages.  */
#ifdef YYERROR_VERBOSE
# undef YYERROR_VERBOSE
# define YYERROR_VERBOSE 1
#else
# define YYERROR_VERBOSE 0
#endif

/* Use api.header.include to #include this header
   instead of duplicating it here.  */
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

#line 385 "Parser.c"

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

/* Second part of user prologue.  */
#line 206 "rholang_mercury.y"

void yyerror(YYLTYPE *loc, yyscan_t scanner, YYSTYPE *result, const char *msg)
{
  fprintf(stderr, "error: %d,%d: %s at %s\n",
    loc->first_line, loc->first_column, msg, rholang_mercury_get_text(scanner));
}

int yyparse(yyscan_t scanner, YYSTYPE *result);

extern int yylex(YYSTYPE *lvalp, YYLTYPE *llocp, yyscan_t scanner);

#line 426 "Parser.c"


#ifdef short
# undef short
#endif

/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */

#ifndef __PTRDIFF_MAX__
# include <limits.h> /* INFRINGES ON USER NAME SPACE */
# if defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stdint.h> /* INFRINGES ON USER NAME SPACE */
#  define YY_STDINT_H
# endif
#endif

/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */

#ifdef __INT_LEAST8_MAX__
typedef __INT_LEAST8_TYPE__ yytype_int8;
#elif defined YY_STDINT_H
typedef int_least8_t yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef __INT_LEAST16_MAX__
typedef __INT_LEAST16_TYPE__ yytype_int16;
#elif defined YY_STDINT_H
typedef int_least16_t yytype_int16;
#else
typedef short yytype_int16;
#endif

#if defined __UINT_LEAST8_MAX__ && __UINT_LEAST8_MAX__ <= __INT_MAX__
typedef __UINT_LEAST8_TYPE__ yytype_uint8;
#elif (!defined __UINT_LEAST8_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST8_MAX <= INT_MAX)
typedef uint_least8_t yytype_uint8;
#elif !defined __UINT_LEAST8_MAX__ && UCHAR_MAX <= INT_MAX
typedef unsigned char yytype_uint8;
#else
typedef short yytype_uint8;
#endif

#if defined __UINT_LEAST16_MAX__ && __UINT_LEAST16_MAX__ <= __INT_MAX__
typedef __UINT_LEAST16_TYPE__ yytype_uint16;
#elif (!defined __UINT_LEAST16_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST16_MAX <= INT_MAX)
typedef uint_least16_t yytype_uint16;
#elif !defined __UINT_LEAST16_MAX__ && USHRT_MAX <= INT_MAX
typedef unsigned short yytype_uint16;
#else
typedef int yytype_uint16;
#endif

#ifndef YYPTRDIFF_T
# if defined __PTRDIFF_TYPE__ && defined __PTRDIFF_MAX__
#  define YYPTRDIFF_T __PTRDIFF_TYPE__
#  define YYPTRDIFF_MAXIMUM __PTRDIFF_MAX__
# elif defined PTRDIFF_MAX
#  ifndef ptrdiff_t
#   include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  endif
#  define YYPTRDIFF_T ptrdiff_t
#  define YYPTRDIFF_MAXIMUM PTRDIFF_MAX
# else
#  define YYPTRDIFF_T long
#  define YYPTRDIFF_MAXIMUM LONG_MAX
# endif
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned
# endif
#endif

#define YYSIZE_MAXIMUM                                  \
  YY_CAST (YYPTRDIFF_T,                                 \
           (YYPTRDIFF_MAXIMUM < YY_CAST (YYSIZE_T, -1)  \
            ? YYPTRDIFF_MAXIMUM                         \
            : YY_CAST (YYSIZE_T, -1)))

#define YYSIZEOF(X) YY_CAST (YYPTRDIFF_T, sizeof (X))

/* Stored state numbers (used for stacks). */
typedef yytype_uint8 yy_state_t;

/* State numbers in computations.  */
typedef int yy_state_fast_t;

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif

#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(E) ((void) (E))
#else
# define YYUSE(E) /* empty */
#endif

#if defined __GNUC__ && ! defined __ICC && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                            \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#define YY_ASSERT(E) ((void) (0 && (E)))

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
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
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
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
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
  yy_state_t yyss_alloc;
  YYSTYPE yyvs_alloc;
  YYLTYPE yyls_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (YYSIZEOF (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (YYSIZEOF (yy_state_t) + YYSIZEOF (YYSTYPE) \
             + YYSIZEOF (YYLTYPE)) \
      + 2 * YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYPTRDIFF_T yynewbytes;                                         \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * YYSIZEOF (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / YYSIZEOF (*yyptr);                        \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, YY_CAST (YYSIZE_T, (Count)) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYPTRDIFF_T yyi;                      \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

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
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  240

#define YYUNDEFTOK  2
#define YYMAXUTOK   325


/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
#define YYTRANSLATE(YYX)                                                \
  (0 <= (YYX) && (YYX) <= YYMAXUTOK ? yytranslate[YYX] : YYUNDEFTOK)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static const yytype_int8 yytranslate[] =
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
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_int16 yyrline[] =
{
       0,   342,   342,   343,   345,   346,   347,   348,   350,   351,
     352,   353,   354,   355,   357,   358,   360,   361,   363,   364,
     366,   367,   368,   369,   371,   372,   373,   374,   375,   377,
     378,   379,   380,   381,   383,   384,   385,   386,   387,   389,
     390,   391,   393,   394,   395,   397,   398,   400,   401,   402,
     404,   405,   407,   408,   410,   411,   412,   413,   414,   415,
     417,   418,   419,   421,   422,   424,   425,   426,   428,   429,
     430,   432,   433,   434,   435,   437,   438,   439,   441,   443,
     445,   446,   448,   450,   452,   453,   455,   457,   459,   460,
     462,   463,   465,   467,   468,   470,   472,   473,   475,   476,
     478,   479,   481,   482,   484,   485,   486,   487,   489,   490,
     491,   492,   494,   496,   497,   498,   500,   501,   503,   504,
     506,   507,   509,   510,   512,   513,   514,   515,   516
};
#endif

#if YYDEBUG || YYERROR_VERBOSE || 0
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
  "VarRefKind", "SimpleType", YY_NULLPTR
};
#endif

# ifdef YYPRINT
/* YYTOKNUM[NUM] -- (External) token number corresponding to the
   (internal) symbol number NUM (which must be that of a token).  */
static const yytype_int16 yytoknum[] =
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

#define YYPACT_NINF (-169)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-94)

#define yytable_value_is_error(Yyn) \
  0

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
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

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
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

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -169,     0,   -84,    14,  -168,    24,   111,   112,   -60,     7,
      19,     2,  -169,   196,  -119,    97,   -23,  -169,   -69,  -116,
       5,   -66,  -169,  -169,   128,  -169,    25,  -169,  -169,    26,
    -169,  -169,    32,  -169,  -169,    65,  -169,    39,  -169,    71,
    -169,  -169,  -169,  -169,    80,  -169,   -65,  -129,  -169,  -169
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

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
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
static const yytype_int8 yystos[] =
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

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_int8 yyr1[] =
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

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
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


#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = YYEMPTY)
#define YYEMPTY         (-2)
#define YYEOF           0

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                    \
  do                                                              \
    if (yychar == YYEMPTY)                                        \
      {                                                           \
        yychar = (Token);                                         \
        yylval = (Value);                                         \
        YYPOPSTACK (yylen);                                       \
        yystate = *yyssp;                                         \
        goto yybackup;                                            \
      }                                                           \
    else                                                          \
      {                                                           \
        yyerror (&yylloc, scanner, result, YY_("syntax error: cannot back up")); \
        YYERROR;                                                  \
      }                                                           \
  while (0)

/* Error token number */
#define YYTERROR        1
#define YYERRCODE       256


/* YYLLOC_DEFAULT -- Set CURRENT to span from RHS[1] to RHS[N].
   If N is 0, then set CURRENT to the empty location which ends
   the previous symbol: RHS[0] (always defined).  */

#ifndef YYLLOC_DEFAULT
# define YYLLOC_DEFAULT(Current, Rhs, N)                                \
    do                                                                  \
      if (N)                                                            \
        {                                                               \
          (Current).first_line   = YYRHSLOC (Rhs, 1).first_line;        \
          (Current).first_column = YYRHSLOC (Rhs, 1).first_column;      \
          (Current).last_line    = YYRHSLOC (Rhs, N).last_line;         \
          (Current).last_column  = YYRHSLOC (Rhs, N).last_column;       \
        }                                                               \
      else                                                              \
        {                                                               \
          (Current).first_line   = (Current).last_line   =              \
            YYRHSLOC (Rhs, 0).last_line;                                \
          (Current).first_column = (Current).last_column =              \
            YYRHSLOC (Rhs, 0).last_column;                              \
        }                                                               \
    while (0)
#endif

#define YYRHSLOC(Rhs, K) ((Rhs)[K])


/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)


/* YY_LOCATION_PRINT -- Print the location on the stream.
   This macro was not mandated originally: define only if we know
   we won't break user code: when these are the locations we know.  */

#ifndef YY_LOCATION_PRINT
# if defined YYLTYPE_IS_TRIVIAL && YYLTYPE_IS_TRIVIAL

/* Print *YYLOCP on YYO.  Private, do not rely on its existence. */

YY_ATTRIBUTE_UNUSED
static int
yy_location_print_ (FILE *yyo, YYLTYPE const * const yylocp)
{
  int res = 0;
  int end_col = 0 != yylocp->last_column ? yylocp->last_column - 1 : 0;
  if (0 <= yylocp->first_line)
    {
      res += YYFPRINTF (yyo, "%d", yylocp->first_line);
      if (0 <= yylocp->first_column)
        res += YYFPRINTF (yyo, ".%d", yylocp->first_column);
    }
  if (0 <= yylocp->last_line)
    {
      if (yylocp->first_line < yylocp->last_line)
        {
          res += YYFPRINTF (yyo, "-%d", yylocp->last_line);
          if (0 <= end_col)
            res += YYFPRINTF (yyo, ".%d", end_col);
        }
      else if (0 <= end_col && yylocp->first_column < end_col)
        res += YYFPRINTF (yyo, "-%d", end_col);
    }
  return res;
 }

#  define YY_LOCATION_PRINT(File, Loc)          \
  yy_location_print_ (File, &(Loc))

# else
#  define YY_LOCATION_PRINT(File, Loc) ((void) 0)
# endif
#endif


# define YY_SYMBOL_PRINT(Title, Type, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Type, Value, Location, scanner, result); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*-----------------------------------.
| Print this symbol's value on YYO.  |
`-----------------------------------*/

static void
yy_symbol_value_print (FILE *yyo, int yytype, YYSTYPE const * const yyvaluep, YYLTYPE const * const yylocationp, yyscan_t scanner, YYSTYPE *result)
{
  FILE *yyoutput = yyo;
  YYUSE (yyoutput);
  YYUSE (yylocationp);
  YYUSE (scanner);
  YYUSE (result);
  if (!yyvaluep)
    return;
# ifdef YYPRINT
  if (yytype < YYNTOKENS)
    YYPRINT (yyo, yytoknum[yytype], *yyvaluep);
# endif
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yytype);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/*---------------------------.
| Print this symbol on YYO.  |
`---------------------------*/

static void
yy_symbol_print (FILE *yyo, int yytype, YYSTYPE const * const yyvaluep, YYLTYPE const * const yylocationp, yyscan_t scanner, YYSTYPE *result)
{
  YYFPRINTF (yyo, "%s %s (",
             yytype < YYNTOKENS ? "token" : "nterm", yytname[yytype]);

  YY_LOCATION_PRINT (yyo, *yylocationp);
  YYFPRINTF (yyo, ": ");
  yy_symbol_value_print (yyo, yytype, yyvaluep, yylocationp, scanner, result);
  YYFPRINTF (yyo, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yy_state_t *yybottom, yy_state_t *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yy_state_t *yyssp, YYSTYPE *yyvsp, YYLTYPE *yylsp, int yyrule, yyscan_t scanner, YYSTYPE *result)
{
  int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %d):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       yystos[+yyssp[yyi + 1 - yynrhs]],
                       &yyvsp[(yyi + 1) - (yynrhs)]
                       , &(yylsp[(yyi + 1) - (yynrhs)])                       , scanner, result);
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, yylsp, Rule, scanner, result); \
} while (0)

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
#ifndef YYINITDEPTH
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
#   define yystrlen(S) (YY_CAST (YYPTRDIFF_T, strlen (S)))
#  else
/* Return the length of YYSTR.  */
static YYPTRDIFF_T
yystrlen (const char *yystr)
{
  YYPTRDIFF_T yylen;
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
static char *
yystpcpy (char *yydest, const char *yysrc)
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
static YYPTRDIFF_T
yytnamerr (char *yyres, const char *yystr)
{
  if (*yystr == '"')
    {
      YYPTRDIFF_T yyn = 0;
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
            else
              goto append;

          append:
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

  if (yyres)
    return yystpcpy (yyres, yystr) - yyres;
  else
    return yystrlen (yystr);
}
# endif

/* Copy into *YYMSG, which is of size *YYMSG_ALLOC, an error message
   about the unexpected token YYTOKEN for the state stack whose top is
   YYSSP.

   Return 0 if *YYMSG was successfully written.  Return 1 if *YYMSG is
   not large enough to hold the message.  In that case, also set
   *YYMSG_ALLOC to the required number of bytes.  Return 2 if the
   required number of bytes is too large to store.  */
static int
yysyntax_error (YYPTRDIFF_T *yymsg_alloc, char **yymsg,
                yy_state_t *yyssp, int yytoken)
{
  enum { YYERROR_VERBOSE_ARGS_MAXIMUM = 5 };
  /* Internationalized format string. */
  const char *yyformat = YY_NULLPTR;
  /* Arguments of yyformat: reported tokens (one for the "unexpected",
     one per "expected"). */
  char const *yyarg[YYERROR_VERBOSE_ARGS_MAXIMUM];
  /* Actual size of YYARG. */
  int yycount = 0;
  /* Cumulated lengths of YYARG.  */
  YYPTRDIFF_T yysize = 0;

  /* There are many possibilities here to consider:
     - If this state is a consistent state with a default action, then
       the only way this function was invoked is if the default action
       is an error action.  In that case, don't check for expected
       tokens because there are none.
     - The only way there can be no lookahead present (in yychar) is if
       this state is a consistent state with a default action.  Thus,
       detecting the absence of a lookahead is sufficient to determine
       that there is no unexpected or expected token to report.  In that
       case, just report a simple "syntax error".
     - Don't assume there isn't a lookahead just because this state is a
       consistent state with a default action.  There might have been a
       previous inconsistent state, consistent state with a non-default
       action, or user semantic action that manipulated yychar.
     - Of course, the expected token list depends on states to have
       correct lookahead information, and it depends on the parser not
       to perform extra reductions after fetching a lookahead from the
       scanner and before detecting a syntax error.  Thus, state merging
       (from LALR or IELR) and default reductions corrupt the expected
       token list.  However, the list is correct for canonical LR with
       one exception: it will still contain any token that will not be
       accepted due to an error action in a later state.
  */
  if (yytoken != YYEMPTY)
    {
      int yyn = yypact[+*yyssp];
      YYPTRDIFF_T yysize0 = yytnamerr (YY_NULLPTR, yytname[yytoken]);
      yysize = yysize0;
      yyarg[yycount++] = yytname[yytoken];
      if (!yypact_value_is_default (yyn))
        {
          /* Start YYX at -YYN if negative to avoid negative indexes in
             YYCHECK.  In other words, skip the first -YYN actions for
             this state because they are default actions.  */
          int yyxbegin = yyn < 0 ? -yyn : 0;
          /* Stay within bounds of both yycheck and yytname.  */
          int yychecklim = YYLAST - yyn + 1;
          int yyxend = yychecklim < YYNTOKENS ? yychecklim : YYNTOKENS;
          int yyx;

          for (yyx = yyxbegin; yyx < yyxend; ++yyx)
            if (yycheck[yyx + yyn] == yyx && yyx != YYTERROR
                && !yytable_value_is_error (yytable[yyx + yyn]))
              {
                if (yycount == YYERROR_VERBOSE_ARGS_MAXIMUM)
                  {
                    yycount = 1;
                    yysize = yysize0;
                    break;
                  }
                yyarg[yycount++] = yytname[yyx];
                {
                  YYPTRDIFF_T yysize1
                    = yysize + yytnamerr (YY_NULLPTR, yytname[yyx]);
                  if (yysize <= yysize1 && yysize1 <= YYSTACK_ALLOC_MAXIMUM)
                    yysize = yysize1;
                  else
                    return 2;
                }
              }
        }
    }

  switch (yycount)
    {
# define YYCASE_(N, S)                      \
      case N:                               \
        yyformat = S;                       \
      break
    default: /* Avoid compiler warnings. */
      YYCASE_(0, YY_("syntax error"));
      YYCASE_(1, YY_("syntax error, unexpected %s"));
      YYCASE_(2, YY_("syntax error, unexpected %s, expecting %s"));
      YYCASE_(3, YY_("syntax error, unexpected %s, expecting %s or %s"));
      YYCASE_(4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
      YYCASE_(5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
# undef YYCASE_
    }

  {
    /* Don't count the "%s"s in the final size, but reserve room for
       the terminator.  */
    YYPTRDIFF_T yysize1 = yysize + (yystrlen (yyformat) - 2 * yycount) + 1;
    if (yysize <= yysize1 && yysize1 <= YYSTACK_ALLOC_MAXIMUM)
      yysize = yysize1;
    else
      return 2;
  }

  if (*yymsg_alloc < yysize)
    {
      *yymsg_alloc = 2 * yysize;
      if (! (yysize <= *yymsg_alloc
             && *yymsg_alloc <= YYSTACK_ALLOC_MAXIMUM))
        *yymsg_alloc = YYSTACK_ALLOC_MAXIMUM;
      return 1;
    }

  /* Avoid sprintf, as that infringes on the user's name space.
     Don't have undefined behavior even if the translation
     produced a string with the wrong number of "%s"s.  */
  {
    char *yyp = *yymsg;
    int yyi = 0;
    while ((*yyp = *yyformat) != '\0')
      if (*yyp == '%' && yyformat[1] == 's' && yyi < yycount)
        {
          yyp += yytnamerr (yyp, yyarg[yyi++]);
          yyformat += 2;
        }
      else
        {
          ++yyp;
          ++yyformat;
        }
  }
  return 0;
}
#endif /* YYERROR_VERBOSE */

/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg, int yytype, YYSTYPE *yyvaluep, YYLTYPE *yylocationp, yyscan_t scanner, YYSTYPE *result)
{
  YYUSE (yyvaluep);
  YYUSE (yylocationp);
  YYUSE (scanner);
  YYUSE (result);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yytype, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yytype);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}




/*----------.
| yyparse.  |
`----------*/

int
yyparse (yyscan_t scanner, YYSTYPE *result)
{
/* The lookahead symbol.  */
int yychar;


/* The semantic value of the lookahead symbol.  */
/* Default value used for initialization, for pacifying older GCCs
   or non-GCC compilers.  */
YY_INITIAL_VALUE (static YYSTYPE yyval_default;)
YYSTYPE yylval YY_INITIAL_VALUE (= yyval_default);

/* Location data for the lookahead symbol.  */
static YYLTYPE yyloc_default
# if defined YYLTYPE_IS_TRIVIAL && YYLTYPE_IS_TRIVIAL
  = { 1, 1, 1, 1 }
# endif
;
YYLTYPE yylloc = yyloc_default;

    /* Number of syntax errors so far.  */
    int yynerrs;

    yy_state_fast_t yystate;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus;

    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.
       'yyls': related to locations.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* The state stack.  */
    yy_state_t yyssa[YYINITDEPTH];
    yy_state_t *yyss;
    yy_state_t *yyssp;

    /* The semantic value stack.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs;
    YYSTYPE *yyvsp;

    /* The location stack.  */
    YYLTYPE yylsa[YYINITDEPTH];
    YYLTYPE *yyls;
    YYLTYPE *yylsp;

    /* The locations where the error started and ended.  */
    YYLTYPE yyerror_range[3];

    YYPTRDIFF_T yystacksize;

  int yyn;
  int yyresult;
  /* Lookahead token as an internal (translated) token number.  */
  int yytoken = 0;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;
  YYLTYPE yyloc;

#if YYERROR_VERBOSE
  /* Buffer for error messages, and its allocated size.  */
  char yymsgbuf[128];
  char *yymsg = yymsgbuf;
  YYPTRDIFF_T yymsg_alloc = sizeof yymsgbuf;
#endif

#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N), yylsp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  yyssp = yyss = yyssa;
  yyvsp = yyvs = yyvsa;
  yylsp = yyls = yylsa;
  yystacksize = YYINITDEPTH;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yystate = 0;
  yyerrstatus = 0;
  yynerrs = 0;
  yychar = YYEMPTY; /* Cause a token to be read.  */
  yylsp[0] = yylloc;
  goto yysetstate;


/*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;


/*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
yysetstate:
  YYDPRINTF ((stderr, "Entering state %d\n", yystate));
  YY_ASSERT (0 <= yystate && yystate < YYNSTATES);
  YY_IGNORE_USELESS_CAST_BEGIN
  *yyssp = YY_CAST (yy_state_t, yystate);
  YY_IGNORE_USELESS_CAST_END

  if (yyss + yystacksize - 1 <= yyssp)
#if !defined yyoverflow && !defined YYSTACK_RELOCATE
    goto yyexhaustedlab;
#else
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYPTRDIFF_T yysize = yyssp - yyss + 1;

# if defined yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        yy_state_t *yyss1 = yyss;
        YYSTYPE *yyvs1 = yyvs;
        YYLTYPE *yyls1 = yyls;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * YYSIZEOF (*yyssp),
                    &yyvs1, yysize * YYSIZEOF (*yyvsp),
                    &yyls1, yysize * YYSIZEOF (*yylsp),
                    &yystacksize);
        yyss = yyss1;
        yyvs = yyvs1;
        yyls = yyls1;
      }
# else /* defined YYSTACK_RELOCATE */
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yy_state_t *yyss1 = yyss;
        union yyalloc *yyptr =
          YY_CAST (union yyalloc *,
                   YYSTACK_ALLOC (YY_CAST (YYSIZE_T, YYSTACK_BYTES (yystacksize))));
        if (! yyptr)
          goto yyexhaustedlab;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
        YYSTACK_RELOCATE (yyls_alloc, yyls);
# undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;
      yylsp = yyls + yysize - 1;

      YY_IGNORE_USELESS_CAST_BEGIN
      YYDPRINTF ((stderr, "Stack size increased to %ld\n",
                  YY_CAST (long, yystacksize)));
      YY_IGNORE_USELESS_CAST_END

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }
#endif /* !defined yyoverflow && !defined YYSTACK_RELOCATE */

  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;


/*-----------.
| yybackup.  |
`-----------*/
yybackup:
  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token: "));
      yychar = yylex (&yylval, &yylloc, scanner);
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
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);
  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END
  *++yylsp = yylloc;

  /* Discard the shifted token.  */
  yychar = YYEMPTY;
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
| yyreduce -- do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];

  /* Default location. */
  YYLLOC_DEFAULT (yyloc, (yylsp - yylen), yylen);
  yyerror_range[1] = yyloc;
  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
  case 2:
#line 342 "rholang_mercury.y"
             { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1923 "Parser.c"
    break;

  case 3:
#line 343 "rholang_mercury.y"
                        { (yyval.proc_) = make_PPar((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1929 "Parser.c"
    break;

  case 4:
#line 345 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1935 "Parser.c"
    break;

  case 5:
#line 346 "rholang_mercury.y"
                                        { (yyval.proc_) = make_PIf((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1941 "Parser.c"
    break;

  case 6:
#line 347 "rholang_mercury.y"
                                                       { (yyval.proc_) = make_PIfElse((yyvsp[-4].proc_), (yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1947 "Parser.c"
    break;

  case 7:
#line 348 "rholang_mercury.y"
                                         { (yyval.proc_) = make_PNew((yyvsp[-2].listnamedecl_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1953 "Parser.c"
    break;

  case 8:
#line 350 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1959 "Parser.c"
    break;

  case 9:
#line 351 "rholang_mercury.y"
                                                                                       { (yyval.proc_) = make_PContr((yyvsp[-8].name_), (yyvsp[-6].listname_), (yyvsp[-5].nameremainder_), (yyvsp[-1].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1965 "Parser.c"
    break;

  case 10:
#line 352 "rholang_mercury.y"
                                                          { (yyval.proc_) = make_PInput((yyvsp[-4].receipt_), (yyvsp[-1].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1971 "Parser.c"
    break;

  case 11:
#line 353 "rholang_mercury.y"
                                        { (yyval.proc_) = make_PChoice((yyvsp[-1].listbranch_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1977 "Parser.c"
    break;

  case 12:
#line 354 "rholang_mercury.y"
                                            { (yyval.proc_) = make_PMatch((yyvsp[-3].proc_), (yyvsp[-1].listcase_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1983 "Parser.c"
    break;

  case 13:
#line 355 "rholang_mercury.y"
                                { (yyval.proc_) = make_PBundle((yyvsp[-3].bundle_), (yyvsp[-1].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1989 "Parser.c"
    break;

  case 14:
#line 357 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 1995 "Parser.c"
    break;

  case 15:
#line 358 "rholang_mercury.y"
                                       { (yyval.proc_) = make_PSend((yyvsp[-4].name_), (yyvsp[-3].send_), (yyvsp[-1].listproc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2001 "Parser.c"
    break;

  case 16:
#line 360 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2007 "Parser.c"
    break;

  case 17:
#line 361 "rholang_mercury.y"
                         { (yyval.proc_) = make_POr((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2013 "Parser.c"
    break;

  case 18:
#line 363 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2019 "Parser.c"
    break;

  case 19:
#line 364 "rholang_mercury.y"
                         { (yyval.proc_) = make_PAnd((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2025 "Parser.c"
    break;

  case 20:
#line 366 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2031 "Parser.c"
    break;

  case 21:
#line 367 "rholang_mercury.y"
                         { (yyval.proc_) = make_PMatches((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2037 "Parser.c"
    break;

  case 22:
#line 368 "rholang_mercury.y"
                         { (yyval.proc_) = make_PEq((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2043 "Parser.c"
    break;

  case 23:
#line 369 "rholang_mercury.y"
                         { (yyval.proc_) = make_PNeq((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2049 "Parser.c"
    break;

  case 24:
#line 371 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2055 "Parser.c"
    break;

  case 25:
#line 372 "rholang_mercury.y"
                         { (yyval.proc_) = make_PLt((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2061 "Parser.c"
    break;

  case 26:
#line 373 "rholang_mercury.y"
                         { (yyval.proc_) = make_PLte((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2067 "Parser.c"
    break;

  case 27:
#line 374 "rholang_mercury.y"
                         { (yyval.proc_) = make_PGt((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2073 "Parser.c"
    break;

  case 28:
#line 375 "rholang_mercury.y"
                         { (yyval.proc_) = make_PGte((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2079 "Parser.c"
    break;

  case 29:
#line 377 "rholang_mercury.y"
              { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2085 "Parser.c"
    break;

  case 30:
#line 378 "rholang_mercury.y"
                         { (yyval.proc_) = make_PAdd((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2091 "Parser.c"
    break;

  case 31:
#line 379 "rholang_mercury.y"
                        { (yyval.proc_) = make_PMinus((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2097 "Parser.c"
    break;

  case 32:
#line 380 "rholang_mercury.y"
                         { (yyval.proc_) = make_PPlusPlus((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2103 "Parser.c"
    break;

  case 33:
#line 381 "rholang_mercury.y"
                         { (yyval.proc_) = make_PMinusMinus((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2109 "Parser.c"
    break;

  case 34:
#line 383 "rholang_mercury.y"
               { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2115 "Parser.c"
    break;

  case 35:
#line 384 "rholang_mercury.y"
                         { (yyval.proc_) = make_PMult((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2121 "Parser.c"
    break;

  case 36:
#line 385 "rholang_mercury.y"
                          { (yyval.proc_) = make_PDiv((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2127 "Parser.c"
    break;

  case 37:
#line 386 "rholang_mercury.y"
                          { (yyval.proc_) = make_PMod((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2133 "Parser.c"
    break;

  case 38:
#line 387 "rholang_mercury.y"
                          { (yyval.proc_) = make_PPercentPercent((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2139 "Parser.c"
    break;

  case 39:
#line 389 "rholang_mercury.y"
                { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2145 "Parser.c"
    break;

  case 40:
#line 390 "rholang_mercury.y"
                    { (yyval.proc_) = make_PNot((yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2151 "Parser.c"
    break;

  case 41:
#line 391 "rholang_mercury.y"
                   { (yyval.proc_) = make_PNeg((yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2157 "Parser.c"
    break;

  case 42:
#line 393 "rholang_mercury.y"
                { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2163 "Parser.c"
    break;

  case 43:
#line 394 "rholang_mercury.y"
                                                     { (yyval.proc_) = make_PMethod((yyvsp[-5].proc_), (yyvsp[-3]._string), (yyvsp[-1].listproc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2169 "Parser.c"
    break;

  case 44:
#line 395 "rholang_mercury.y"
                          { (yyval.proc_) = make_PExprs((yyvsp[-1].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2175 "Parser.c"
    break;

  case 45:
#line 397 "rholang_mercury.y"
                { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2181 "Parser.c"
    break;

  case 46:
#line 398 "rholang_mercury.y"
                 { (yyval.proc_) = make_PEval((yyvsp[0].name_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2187 "Parser.c"
    break;

  case 47:
#line 400 "rholang_mercury.y"
                { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2193 "Parser.c"
    break;

  case 48:
#line 401 "rholang_mercury.y"
                        { (yyval.proc_) = make_PVarRef((yyvsp[-1].varrefkind_), (yyvsp[0]._string)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2199 "Parser.c"
    break;

  case 49:
#line 402 "rholang_mercury.y"
                          { (yyval.proc_) = make_PDisjunction((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2205 "Parser.c"
    break;

  case 50:
#line 404 "rholang_mercury.y"
                { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2211 "Parser.c"
    break;

  case 51:
#line 405 "rholang_mercury.y"
                          { (yyval.proc_) = make_PConjunction((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2217 "Parser.c"
    break;

  case 52:
#line 407 "rholang_mercury.y"
                { (yyval.proc_) = (yyvsp[0].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2223 "Parser.c"
    break;

  case 53:
#line 408 "rholang_mercury.y"
                   { (yyval.proc_) = make_PNegation((yyvsp[0].proc_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2229 "Parser.c"
    break;

  case 54:
#line 410 "rholang_mercury.y"
                              { (yyval.proc_) = (yyvsp[-1].proc_); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2235 "Parser.c"
    break;

  case 55:
#line 411 "rholang_mercury.y"
           { (yyval.proc_) = make_PGround((yyvsp[0].ground_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2241 "Parser.c"
    break;

  case 56:
#line 412 "rholang_mercury.y"
               { (yyval.proc_) = make_PCollect((yyvsp[0].collection_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2247 "Parser.c"
    break;

  case 57:
#line 413 "rholang_mercury.y"
            { (yyval.proc_) = make_PVar((yyvsp[0].procvar_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2253 "Parser.c"
    break;

  case 58:
#line 414 "rholang_mercury.y"
             { (yyval.proc_) = make_PNil(); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2259 "Parser.c"
    break;

  case 59:
#line 415 "rholang_mercury.y"
               { (yyval.proc_) = make_PSimpleType((yyvsp[0].simpletype_)); (yyval.proc_)->line_number = (yyloc).first_line; (yyval.proc_)->char_number = (yyloc).first_column; result->proc_ = (yyval.proc_); }
#line 2265 "Parser.c"
    break;

  case 60:
#line 417 "rholang_mercury.y"
                       { (yyval.listproc_) = 0; result->listproc_ = (yyval.listproc_); }
#line 2271 "Parser.c"
    break;

  case 61:
#line 418 "rholang_mercury.y"
         { (yyval.listproc_) = make_ListProc((yyvsp[0].proc_), 0); result->listproc_ = (yyval.listproc_); }
#line 2277 "Parser.c"
    break;

  case 62:
#line 419 "rholang_mercury.y"
                           { (yyval.listproc_) = make_ListProc((yyvsp[-2].proc_), (yyvsp[0].listproc_)); result->listproc_ = (yyval.listproc_); }
#line 2283 "Parser.c"
    break;

  case 63:
#line 421 "rholang_mercury.y"
                   { (yyval.procvar_) = make_ProcVarWildcard(); (yyval.procvar_)->line_number = (yyloc).first_line; (yyval.procvar_)->char_number = (yyloc).first_column; result->procvar_ = (yyval.procvar_); }
#line 2289 "Parser.c"
    break;

  case 64:
#line 422 "rholang_mercury.y"
             { (yyval.procvar_) = make_ProcVarVar((yyvsp[0]._string)); (yyval.procvar_)->line_number = (yyloc).first_line; (yyval.procvar_)->char_number = (yyloc).first_column; result->procvar_ = (yyval.procvar_); }
#line 2295 "Parser.c"
    break;

  case 65:
#line 424 "rholang_mercury.y"
                { (yyval.name_) = make_NameWildcard(); (yyval.name_)->line_number = (yyloc).first_line; (yyval.name_)->char_number = (yyloc).first_column; result->name_ = (yyval.name_); }
#line 2301 "Parser.c"
    break;

  case 66:
#line 425 "rholang_mercury.y"
             { (yyval.name_) = make_NameVar((yyvsp[0]._string)); (yyval.name_)->line_number = (yyloc).first_line; (yyval.name_)->char_number = (yyloc).first_column; result->name_ = (yyval.name_); }
#line 2307 "Parser.c"
    break;

  case 67:
#line 426 "rholang_mercury.y"
                    { (yyval.name_) = make_NameQuote((yyvsp[0].proc_)); (yyval.name_)->line_number = (yyloc).first_line; (yyval.name_)->char_number = (yyloc).first_column; result->name_ = (yyval.name_); }
#line 2313 "Parser.c"
    break;

  case 68:
#line 428 "rholang_mercury.y"
                       { (yyval.listname_) = 0; result->listname_ = (yyval.listname_); }
#line 2319 "Parser.c"
    break;

  case 69:
#line 429 "rholang_mercury.y"
         { (yyval.listname_) = make_ListName((yyvsp[0].name_), 0); result->listname_ = (yyval.listname_); }
#line 2325 "Parser.c"
    break;

  case 70:
#line 430 "rholang_mercury.y"
                           { (yyval.listname_) = make_ListName((yyvsp[-2].name_), (yyvsp[0].listname_)); result->listname_ = (yyval.listname_); }
#line 2331 "Parser.c"
    break;

  case 71:
#line 432 "rholang_mercury.y"
                  { (yyval.bundle_) = make_BundleWrite(); (yyval.bundle_)->line_number = (yyloc).first_line; (yyval.bundle_)->char_number = (yyloc).first_column; result->bundle_ = (yyval.bundle_); }
#line 2337 "Parser.c"
    break;

  case 72:
#line 433 "rholang_mercury.y"
             { (yyval.bundle_) = make_BundleRead(); (yyval.bundle_)->line_number = (yyloc).first_line; (yyval.bundle_)->char_number = (yyloc).first_column; result->bundle_ = (yyval.bundle_); }
#line 2343 "Parser.c"
    break;

  case 73:
#line 434 "rholang_mercury.y"
             { (yyval.bundle_) = make_BundleEquiv(); (yyval.bundle_)->line_number = (yyloc).first_line; (yyval.bundle_)->char_number = (yyloc).first_column; result->bundle_ = (yyval.bundle_); }
#line 2349 "Parser.c"
    break;

  case 74:
#line 435 "rholang_mercury.y"
             { (yyval.bundle_) = make_BundleReadWrite(); (yyval.bundle_)->line_number = (yyloc).first_line; (yyval.bundle_)->char_number = (yyloc).first_column; result->bundle_ = (yyval.bundle_); }
#line 2355 "Parser.c"
    break;

  case 75:
#line 437 "rholang_mercury.y"
                            { (yyval.receipt_) = make_ReceiptLinear((yyvsp[0].receiptlinearimpl_)); (yyval.receipt_)->line_number = (yyloc).first_line; (yyval.receipt_)->char_number = (yyloc).first_column; result->receipt_ = (yyval.receipt_); }
#line 2361 "Parser.c"
    break;

  case 76:
#line 438 "rholang_mercury.y"
                        { (yyval.receipt_) = make_ReceiptRepeated((yyvsp[0].receiptrepeatedimpl_)); (yyval.receipt_)->line_number = (yyloc).first_line; (yyval.receipt_)->char_number = (yyloc).first_column; result->receipt_ = (yyval.receipt_); }
#line 2367 "Parser.c"
    break;

  case 77:
#line 439 "rholang_mercury.y"
                    { (yyval.receipt_) = make_ReceiptPeek((yyvsp[0].receiptpeekimpl_)); (yyval.receipt_)->line_number = (yyloc).first_line; (yyval.receipt_)->char_number = (yyloc).first_column; result->receipt_ = (yyval.receipt_); }
#line 2373 "Parser.c"
    break;

  case 78:
#line 441 "rholang_mercury.y"
                                   { (yyval.receiptlinearimpl_) = make_LinearSimple((yyvsp[0].listlinearbind_)); (yyval.receiptlinearimpl_)->line_number = (yyloc).first_line; (yyval.receiptlinearimpl_)->char_number = (yyloc).first_column; result->receiptlinearimpl_ = (yyval.receiptlinearimpl_); }
#line 2379 "Parser.c"
    break;

  case 79:
#line 443 "rholang_mercury.y"
                                                  { (yyval.linearbind_) = make_LinearBindImpl((yyvsp[-3].listname_), (yyvsp[-2].nameremainder_), (yyvsp[0].name_)); (yyval.linearbind_)->line_number = (yyloc).first_line; (yyval.linearbind_)->char_number = (yyloc).first_column; result->linearbind_ = (yyval.linearbind_); }
#line 2385 "Parser.c"
    break;

  case 80:
#line 445 "rholang_mercury.y"
                            { (yyval.listlinearbind_) = make_ListLinearBind((yyvsp[0].linearbind_), 0); result->listlinearbind_ = (yyval.listlinearbind_); }
#line 2391 "Parser.c"
    break;

  case 81:
#line 446 "rholang_mercury.y"
                                       { (yyval.listlinearbind_) = make_ListLinearBind((yyvsp[-2].linearbind_), (yyvsp[0].listlinearbind_)); result->listlinearbind_ = (yyval.listlinearbind_); }
#line 2397 "Parser.c"
    break;

  case 82:
#line 448 "rholang_mercury.y"
                                       { (yyval.receiptrepeatedimpl_) = make_RepeatedSimple((yyvsp[0].listrepeatedbind_)); (yyval.receiptrepeatedimpl_)->line_number = (yyloc).first_line; (yyval.receiptrepeatedimpl_)->char_number = (yyloc).first_column; result->receiptrepeatedimpl_ = (yyval.receiptrepeatedimpl_); }
#line 2403 "Parser.c"
    break;

  case 83:
#line 450 "rholang_mercury.y"
                                                    { (yyval.repeatedbind_) = make_RepeatedBindImpl((yyvsp[-3].listname_), (yyvsp[-2].nameremainder_), (yyvsp[0].name_)); (yyval.repeatedbind_)->line_number = (yyloc).first_line; (yyval.repeatedbind_)->char_number = (yyloc).first_column; result->repeatedbind_ = (yyval.repeatedbind_); }
#line 2409 "Parser.c"
    break;

  case 84:
#line 452 "rholang_mercury.y"
                                { (yyval.listrepeatedbind_) = make_ListRepeatedBind((yyvsp[0].repeatedbind_), 0); result->listrepeatedbind_ = (yyval.listrepeatedbind_); }
#line 2415 "Parser.c"
    break;

  case 85:
#line 453 "rholang_mercury.y"
                                           { (yyval.listrepeatedbind_) = make_ListRepeatedBind((yyvsp[-2].repeatedbind_), (yyvsp[0].listrepeatedbind_)); result->listrepeatedbind_ = (yyval.listrepeatedbind_); }
#line 2421 "Parser.c"
    break;

  case 86:
#line 455 "rholang_mercury.y"
                               { (yyval.receiptpeekimpl_) = make_PeekSimple((yyvsp[0].listpeekbind_)); (yyval.receiptpeekimpl_)->line_number = (yyloc).first_line; (yyval.receiptpeekimpl_)->char_number = (yyloc).first_column; result->receiptpeekimpl_ = (yyval.receiptpeekimpl_); }
#line 2427 "Parser.c"
    break;

  case 87:
#line 457 "rholang_mercury.y"
                                                { (yyval.peekbind_) = make_PeekBindImpl((yyvsp[-3].listname_), (yyvsp[-2].nameremainder_), (yyvsp[0].name_)); (yyval.peekbind_)->line_number = (yyloc).first_line; (yyval.peekbind_)->char_number = (yyloc).first_column; result->peekbind_ = (yyval.peekbind_); }
#line 2433 "Parser.c"
    break;

  case 88:
#line 459 "rholang_mercury.y"
                        { (yyval.listpeekbind_) = make_ListPeekBind((yyvsp[0].peekbind_), 0); result->listpeekbind_ = (yyval.listpeekbind_); }
#line 2439 "Parser.c"
    break;

  case 89:
#line 460 "rholang_mercury.y"
                                   { (yyval.listpeekbind_) = make_ListPeekBind((yyvsp[-2].peekbind_), (yyvsp[0].listpeekbind_)); result->listpeekbind_ = (yyval.listpeekbind_); }
#line 2445 "Parser.c"
    break;

  case 90:
#line 462 "rholang_mercury.y"
                { (yyval.send_) = make_SendSingle(); (yyval.send_)->line_number = (yyloc).first_line; (yyval.send_)->char_number = (yyloc).first_column; result->send_ = (yyval.send_); }
#line 2451 "Parser.c"
    break;

  case 91:
#line 463 "rholang_mercury.y"
             { (yyval.send_) = make_SendMultiple(); (yyval.send_)->line_number = (yyloc).first_line; (yyval.send_)->char_number = (yyloc).first_column; result->send_ = (yyval.send_); }
#line 2457 "Parser.c"
    break;

  case 92:
#line 465 "rholang_mercury.y"
                                          { (yyval.branch_) = make_BranchImpl((yyvsp[-2].receiptlinearimpl_), (yyvsp[0].proc_)); (yyval.branch_)->line_number = (yyloc).first_line; (yyval.branch_)->char_number = (yyloc).first_column; result->branch_ = (yyval.branch_); }
#line 2463 "Parser.c"
    break;

  case 93:
#line 467 "rholang_mercury.y"
                    { (yyval.listbranch_) = make_ListBranch((yyvsp[0].branch_), 0); result->listbranch_ = (yyval.listbranch_); }
#line 2469 "Parser.c"
    break;

  case 94:
#line 468 "rholang_mercury.y"
                      { (yyval.listbranch_) = make_ListBranch((yyvsp[-1].branch_), (yyvsp[0].listbranch_)); result->listbranch_ = (yyval.listbranch_); }
#line 2475 "Parser.c"
    break;

  case 95:
#line 470 "rholang_mercury.y"
                             { (yyval.case_) = make_CaseImpl((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.case_)->line_number = (yyloc).first_line; (yyval.case_)->char_number = (yyloc).first_column; result->case_ = (yyval.case_); }
#line 2481 "Parser.c"
    break;

  case 96:
#line 472 "rholang_mercury.y"
                { (yyval.listcase_) = make_ListCase((yyvsp[0].case_), 0); result->listcase_ = (yyval.listcase_); }
#line 2487 "Parser.c"
    break;

  case 97:
#line 473 "rholang_mercury.y"
                  { (yyval.listcase_) = make_ListCase((yyvsp[-1].case_), (yyvsp[0].listcase_)); result->listcase_ = (yyval.listcase_); }
#line 2493 "Parser.c"
    break;

  case 98:
#line 475 "rholang_mercury.y"
                    { (yyval.namedecl_) = make_NameDeclSimpl((yyvsp[0]._string)); (yyval.namedecl_)->line_number = (yyloc).first_line; (yyval.namedecl_)->char_number = (yyloc).first_column; result->namedecl_ = (yyval.namedecl_); }
#line 2499 "Parser.c"
    break;

  case 99:
#line 476 "rholang_mercury.y"
                                      { (yyval.namedecl_) = make_NameDeclUrn((yyvsp[-3]._string), (yyvsp[-1]._string)); (yyval.namedecl_)->line_number = (yyloc).first_line; (yyval.namedecl_)->char_number = (yyloc).first_column; result->namedecl_ = (yyval.namedecl_); }
#line 2505 "Parser.c"
    break;

  case 100:
#line 478 "rholang_mercury.y"
                        { (yyval.listnamedecl_) = make_ListNameDecl((yyvsp[0].namedecl_), 0); result->listnamedecl_ = (yyval.listnamedecl_); }
#line 2511 "Parser.c"
    break;

  case 101:
#line 479 "rholang_mercury.y"
                                   { (yyval.listnamedecl_) = make_ListNameDecl((yyvsp[-2].namedecl_), (yyvsp[0].listnamedecl_)); result->listnamedecl_ = (yyval.listnamedecl_); }
#line 2517 "Parser.c"
    break;

  case 102:
#line 481 "rholang_mercury.y"
                       { (yyval.boolliteral_) = make_BoolTrue(); (yyval.boolliteral_)->line_number = (yyloc).first_line; (yyval.boolliteral_)->char_number = (yyloc).first_column; result->boolliteral_ = (yyval.boolliteral_); }
#line 2523 "Parser.c"
    break;

  case 103:
#line 482 "rholang_mercury.y"
             { (yyval.boolliteral_) = make_BoolFalse(); (yyval.boolliteral_)->line_number = (yyloc).first_line; (yyval.boolliteral_)->char_number = (yyloc).first_column; result->boolliteral_ = (yyval.boolliteral_); }
#line 2529 "Parser.c"
    break;

  case 104:
#line 484 "rholang_mercury.y"
                     { (yyval.ground_) = make_GroundBool((yyvsp[0].boolliteral_)); (yyval.ground_)->line_number = (yyloc).first_line; (yyval.ground_)->char_number = (yyloc).first_column; result->ground_ = (yyval.ground_); }
#line 2535 "Parser.c"
    break;

  case 105:
#line 485 "rholang_mercury.y"
             { (yyval.ground_) = make_GroundInt((yyvsp[0]._string)); (yyval.ground_)->line_number = (yyloc).first_line; (yyval.ground_)->char_number = (yyloc).first_column; result->ground_ = (yyval.ground_); }
#line 2541 "Parser.c"
    break;

  case 106:
#line 486 "rholang_mercury.y"
             { (yyval.ground_) = make_GroundString((yyvsp[0]._string)); (yyval.ground_)->line_number = (yyloc).first_line; (yyval.ground_)->char_number = (yyloc).first_column; result->ground_ = (yyval.ground_); }
#line 2547 "Parser.c"
    break;

  case 107:
#line 487 "rholang_mercury.y"
             { (yyval.ground_) = make_GroundUri((yyvsp[0]._string)); (yyval.ground_)->line_number = (yyloc).first_line; (yyval.ground_)->char_number = (yyloc).first_column; result->ground_ = (yyval.ground_); }
#line 2553 "Parser.c"
    break;

  case 108:
#line 489 "rholang_mercury.y"
                                                      { (yyval.collection_) = make_CollectList((yyvsp[-2].listproc_), (yyvsp[-1].procremainder_)); (yyval.collection_)->line_number = (yyloc).first_line; (yyval.collection_)->char_number = (yyloc).first_column; result->collection_ = (yyval.collection_); }
#line 2559 "Parser.c"
    break;

  case 109:
#line 490 "rholang_mercury.y"
          { (yyval.collection_) = make_CollectTuple((yyvsp[0].tuple_)); (yyval.collection_)->line_number = (yyloc).first_line; (yyval.collection_)->char_number = (yyloc).first_column; result->collection_ = (yyval.collection_); }
#line 2565 "Parser.c"
    break;

  case 110:
#line 491 "rholang_mercury.y"
                                                    { (yyval.collection_) = make_CollectSet((yyvsp[-2].listproc_), (yyvsp[-1].procremainder_)); (yyval.collection_)->line_number = (yyloc).first_line; (yyval.collection_)->char_number = (yyloc).first_column; result->collection_ = (yyval.collection_); }
#line 2571 "Parser.c"
    break;

  case 111:
#line 492 "rholang_mercury.y"
                                                   { (yyval.collection_) = make_CollectMap((yyvsp[-2].listkeyvaluepair_), (yyvsp[-1].procremainder_)); (yyval.collection_)->line_number = (yyloc).first_line; (yyval.collection_)->char_number = (yyloc).first_column; result->collection_ = (yyval.collection_); }
#line 2577 "Parser.c"
    break;

  case 112:
#line 494 "rholang_mercury.y"
                                  { (yyval.keyvaluepair_) = make_KeyValuePairImpl((yyvsp[-2].proc_), (yyvsp[0].proc_)); (yyval.keyvaluepair_)->line_number = (yyloc).first_line; (yyval.keyvaluepair_)->char_number = (yyloc).first_column; result->keyvaluepair_ = (yyval.keyvaluepair_); }
#line 2583 "Parser.c"
    break;

  case 113:
#line 496 "rholang_mercury.y"
                               { (yyval.listkeyvaluepair_) = 0; result->listkeyvaluepair_ = (yyval.listkeyvaluepair_); }
#line 2589 "Parser.c"
    break;

  case 114:
#line 497 "rholang_mercury.y"
                 { (yyval.listkeyvaluepair_) = make_ListKeyValuePair((yyvsp[0].keyvaluepair_), 0); result->listkeyvaluepair_ = (yyval.listkeyvaluepair_); }
#line 2595 "Parser.c"
    break;

  case 115:
#line 498 "rholang_mercury.y"
                                           { (yyval.listkeyvaluepair_) = make_ListKeyValuePair((yyvsp[-2].keyvaluepair_), (yyvsp[0].listkeyvaluepair_)); result->listkeyvaluepair_ = (yyval.listkeyvaluepair_); }
#line 2601 "Parser.c"
    break;

  case 116:
#line 500 "rholang_mercury.y"
                              { (yyval.tuple_) = make_TupleSingle((yyvsp[-1].proc_)); (yyval.tuple_)->line_number = (yyloc).first_line; (yyval.tuple_)->char_number = (yyloc).first_column; result->tuple_ = (yyval.tuple_); }
#line 2607 "Parser.c"
    break;

  case 117:
#line 501 "rholang_mercury.y"
                                           { (yyval.tuple_) = make_TupleMultiple((yyvsp[-3].proc_), (yyvsp[-1].listproc_)); (yyval.tuple_)->line_number = (yyloc).first_line; (yyval.tuple_)->char_number = (yyloc).first_column; result->tuple_ = (yyval.tuple_); }
#line 2613 "Parser.c"
    break;

  case 118:
#line 503 "rholang_mercury.y"
                                 { (yyval.procremainder_) = make_ProcRemainderVar((yyvsp[0].procvar_)); (yyval.procremainder_)->line_number = (yyloc).first_line; (yyval.procremainder_)->char_number = (yyloc).first_column; result->procremainder_ = (yyval.procremainder_); }
#line 2619 "Parser.c"
    break;

  case 119:
#line 504 "rholang_mercury.y"
                { (yyval.procremainder_) = make_ProcRemainderEmpty(); (yyval.procremainder_)->line_number = (yyloc).first_line; (yyval.procremainder_)->char_number = (yyloc).first_column; result->procremainder_ = (yyval.procremainder_); }
#line 2625 "Parser.c"
    break;

  case 120:
#line 506 "rholang_mercury.y"
                                          { (yyval.nameremainder_) = make_NameRemainderVar((yyvsp[0].procvar_)); (yyval.nameremainder_)->line_number = (yyloc).first_line; (yyval.nameremainder_)->char_number = (yyloc).first_column; result->nameremainder_ = (yyval.nameremainder_); }
#line 2631 "Parser.c"
    break;

  case 121:
#line 507 "rholang_mercury.y"
                { (yyval.nameremainder_) = make_NameRemainderEmpty(); (yyval.nameremainder_)->line_number = (yyloc).first_line; (yyval.nameremainder_)->char_number = (yyloc).first_column; result->nameremainder_ = (yyval.nameremainder_); }
#line 2637 "Parser.c"
    break;

  case 122:
#line 509 "rholang_mercury.y"
                      { (yyval.varrefkind_) = make_VarRefKindProc(); (yyval.varrefkind_)->line_number = (yyloc).first_line; (yyval.varrefkind_)->char_number = (yyloc).first_column; result->varrefkind_ = (yyval.varrefkind_); }
#line 2643 "Parser.c"
    break;

  case 123:
#line 510 "rholang_mercury.y"
                     { (yyval.varrefkind_) = make_VarRefKindName(); (yyval.varrefkind_)->line_number = (yyloc).first_line; (yyval.varrefkind_)->char_number = (yyloc).first_column; result->varrefkind_ = (yyval.varrefkind_); }
#line 2649 "Parser.c"
    break;

  case 124:
#line 512 "rholang_mercury.y"
                      { (yyval.simpletype_) = make_SimpleTypeBool(); (yyval.simpletype_)->line_number = (yyloc).first_line; (yyval.simpletype_)->char_number = (yyloc).first_column; result->simpletype_ = (yyval.simpletype_); }
#line 2655 "Parser.c"
    break;

  case 125:
#line 513 "rholang_mercury.y"
             { (yyval.simpletype_) = make_SimpleTypeInt(); (yyval.simpletype_)->line_number = (yyloc).first_line; (yyval.simpletype_)->char_number = (yyloc).first_column; result->simpletype_ = (yyval.simpletype_); }
#line 2661 "Parser.c"
    break;

  case 126:
#line 514 "rholang_mercury.y"
             { (yyval.simpletype_) = make_SimpleTypeString(); (yyval.simpletype_)->line_number = (yyloc).first_line; (yyval.simpletype_)->char_number = (yyloc).first_column; result->simpletype_ = (yyval.simpletype_); }
#line 2667 "Parser.c"
    break;

  case 127:
#line 515 "rholang_mercury.y"
             { (yyval.simpletype_) = make_SimpleTypeUri(); (yyval.simpletype_)->line_number = (yyloc).first_line; (yyval.simpletype_)->char_number = (yyloc).first_column; result->simpletype_ = (yyval.simpletype_); }
#line 2673 "Parser.c"
    break;

  case 128:
#line 516 "rholang_mercury.y"
             { (yyval.simpletype_) = make_SimpleTypeByteArray(); (yyval.simpletype_)->line_number = (yyloc).first_line; (yyval.simpletype_)->char_number = (yyloc).first_column; result->simpletype_ = (yyval.simpletype_); }
#line 2679 "Parser.c"
    break;


#line 2683 "Parser.c"

      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", yyr1[yyn], &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);

  *++yyvsp = yyval;
  *++yylsp = yyloc;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
  {
    const int yylhs = yyr1[yyn] - YYNTOKENS;
    const int yyi = yypgoto[yylhs] + *yyssp;
    yystate = (0 <= yyi && yyi <= YYLAST && yycheck[yyi] == *yyssp
               ? yytable[yyi]
               : yydefgoto[yylhs]);
  }

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == YYEMPTY ? YYEMPTY : YYTRANSLATE (yychar);

  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
#if ! YYERROR_VERBOSE
      yyerror (&yylloc, scanner, result, YY_("syntax error"));
#else
# define YYSYNTAX_ERROR yysyntax_error (&yymsg_alloc, &yymsg, \
                                        yyssp, yytoken)
      {
        char const *yymsgp = YY_("syntax error");
        int yysyntax_error_status;
        yysyntax_error_status = YYSYNTAX_ERROR;
        if (yysyntax_error_status == 0)
          yymsgp = yymsg;
        else if (yysyntax_error_status == 1)
          {
            if (yymsg != yymsgbuf)
              YYSTACK_FREE (yymsg);
            yymsg = YY_CAST (char *, YYSTACK_ALLOC (YY_CAST (YYSIZE_T, yymsg_alloc)));
            if (!yymsg)
              {
                yymsg = yymsgbuf;
                yymsg_alloc = sizeof yymsgbuf;
                yysyntax_error_status = 2;
              }
            else
              {
                yysyntax_error_status = YYSYNTAX_ERROR;
                yymsgp = yymsg;
              }
          }
        yyerror (&yylloc, scanner, result, yymsgp);
        if (yysyntax_error_status == 2)
          goto yyexhaustedlab;
      }
# undef YYSYNTAX_ERROR
#endif
    }

  yyerror_range[1] = yylloc;

  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
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
                      yytoken, &yylval, &yylloc, scanner, result);
          yychar = YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:
  /* Pacify compilers when the user code never invokes YYERROR and the
     label yyerrorlab therefore never appears in user code.  */
  if (0)
    YYERROR;

  /* Do not reclaim the symbols of the rule whose action triggered
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
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
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

      yyerror_range[1] = *yylsp;
      yydestruct ("Error: popping",
                  yystos[yystate], yyvsp, yylsp, scanner, result);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  yyerror_range[2] = yylloc;
  /* Using YYLLOC is tempting, but would change the location of
     the lookahead.  YYLOC is available though.  */
  YYLLOC_DEFAULT (yyloc, yyerror_range, 2);
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


#if !defined yyoverflow || YYERROR_VERBOSE
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (&yylloc, scanner, result, YY_("memory exhausted"));
  yyresult = 2;
  /* Fall through.  */
#endif


/*-----------------------------------------------------.
| yyreturn -- parsing is finished, return the result.  |
`-----------------------------------------------------*/
yyreturn:
  if (yychar != YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval, &yylloc, scanner, result);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  yystos[+*yyssp], yyvsp, yylsp, scanner, result);
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
  return yyresult;
}
#line 519 "rholang_mercury.y"


/* Entrypoint: parse Proc from file. */
Proc pProc(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc1(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc1(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc2(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc2(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc3(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc3(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc4(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc4(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc5(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc5(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc6(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc6(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc7(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc7(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc8(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc8(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc9(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc9(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc10(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc10(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc11(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc11(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc12(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc12(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc13(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc13(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc14(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc14(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc15(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc15(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from file. */
Proc pProc16(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse Proc from string. */
Proc psProc16(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.proc_;
  }
}

/* Entrypoint: parse ListProc from file. */
ListProc pListProc(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listproc_;
  }
}

/* Entrypoint: parse ListProc from string. */
ListProc psListProc(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listproc_;
  }
}

/* Entrypoint: parse ProcVar from file. */
ProcVar pProcVar(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.procvar_;
  }
}

/* Entrypoint: parse ProcVar from string. */
ProcVar psProcVar(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.procvar_;
  }
}

/* Entrypoint: parse Name from file. */
Name pName(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.name_;
  }
}

/* Entrypoint: parse Name from string. */
Name psName(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.name_;
  }
}

/* Entrypoint: parse ListName from file. */
ListName pListName(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listname_;
  }
}

/* Entrypoint: parse ListName from string. */
ListName psListName(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listname_;
  }
}

/* Entrypoint: parse Bundle from file. */
Bundle pBundle(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.bundle_;
  }
}

/* Entrypoint: parse Bundle from string. */
Bundle psBundle(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.bundle_;
  }
}

/* Entrypoint: parse Receipt from file. */
Receipt pReceipt(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receipt_;
  }
}

/* Entrypoint: parse Receipt from string. */
Receipt psReceipt(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receipt_;
  }
}

/* Entrypoint: parse ReceiptLinearImpl from file. */
ReceiptLinearImpl pReceiptLinearImpl(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receiptlinearimpl_;
  }
}

/* Entrypoint: parse ReceiptLinearImpl from string. */
ReceiptLinearImpl psReceiptLinearImpl(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receiptlinearimpl_;
  }
}

/* Entrypoint: parse LinearBind from file. */
LinearBind pLinearBind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.linearbind_;
  }
}

/* Entrypoint: parse LinearBind from string. */
LinearBind psLinearBind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.linearbind_;
  }
}

/* Entrypoint: parse ListLinearBind from file. */
ListLinearBind pListLinearBind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listlinearbind_;
  }
}

/* Entrypoint: parse ListLinearBind from string. */
ListLinearBind psListLinearBind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listlinearbind_;
  }
}

/* Entrypoint: parse ReceiptRepeatedImpl from file. */
ReceiptRepeatedImpl pReceiptRepeatedImpl(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receiptrepeatedimpl_;
  }
}

/* Entrypoint: parse ReceiptRepeatedImpl from string. */
ReceiptRepeatedImpl psReceiptRepeatedImpl(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receiptrepeatedimpl_;
  }
}

/* Entrypoint: parse RepeatedBind from file. */
RepeatedBind pRepeatedBind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.repeatedbind_;
  }
}

/* Entrypoint: parse RepeatedBind from string. */
RepeatedBind psRepeatedBind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.repeatedbind_;
  }
}

/* Entrypoint: parse ListRepeatedBind from file. */
ListRepeatedBind pListRepeatedBind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listrepeatedbind_;
  }
}

/* Entrypoint: parse ListRepeatedBind from string. */
ListRepeatedBind psListRepeatedBind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listrepeatedbind_;
  }
}

/* Entrypoint: parse ReceiptPeekImpl from file. */
ReceiptPeekImpl pReceiptPeekImpl(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receiptpeekimpl_;
  }
}

/* Entrypoint: parse ReceiptPeekImpl from string. */
ReceiptPeekImpl psReceiptPeekImpl(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.receiptpeekimpl_;
  }
}

/* Entrypoint: parse PeekBind from file. */
PeekBind pPeekBind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.peekbind_;
  }
}

/* Entrypoint: parse PeekBind from string. */
PeekBind psPeekBind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.peekbind_;
  }
}

/* Entrypoint: parse ListPeekBind from file. */
ListPeekBind pListPeekBind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listpeekbind_;
  }
}

/* Entrypoint: parse ListPeekBind from string. */
ListPeekBind psListPeekBind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listpeekbind_;
  }
}

/* Entrypoint: parse Send from file. */
Send pSend(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.send_;
  }
}

/* Entrypoint: parse Send from string. */
Send psSend(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.send_;
  }
}

/* Entrypoint: parse Branch from file. */
Branch pBranch(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.branch_;
  }
}

/* Entrypoint: parse Branch from string. */
Branch psBranch(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.branch_;
  }
}

/* Entrypoint: parse ListBranch from file. */
ListBranch pListBranch(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listbranch_;
  }
}

/* Entrypoint: parse ListBranch from string. */
ListBranch psListBranch(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listbranch_;
  }
}

/* Entrypoint: parse Case from file. */
Case pCase(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.case_;
  }
}

/* Entrypoint: parse Case from string. */
Case psCase(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.case_;
  }
}

/* Entrypoint: parse ListCase from file. */
ListCase pListCase(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listcase_;
  }
}

/* Entrypoint: parse ListCase from string. */
ListCase psListCase(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listcase_;
  }
}

/* Entrypoint: parse NameDecl from file. */
NameDecl pNameDecl(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.namedecl_;
  }
}

/* Entrypoint: parse NameDecl from string. */
NameDecl psNameDecl(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.namedecl_;
  }
}

/* Entrypoint: parse ListNameDecl from file. */
ListNameDecl pListNameDecl(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listnamedecl_;
  }
}

/* Entrypoint: parse ListNameDecl from string. */
ListNameDecl psListNameDecl(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listnamedecl_;
  }
}

/* Entrypoint: parse BoolLiteral from file. */
BoolLiteral pBoolLiteral(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.boolliteral_;
  }
}

/* Entrypoint: parse BoolLiteral from string. */
BoolLiteral psBoolLiteral(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.boolliteral_;
  }
}

/* Entrypoint: parse Ground from file. */
Ground pGround(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.ground_;
  }
}

/* Entrypoint: parse Ground from string. */
Ground psGround(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.ground_;
  }
}

/* Entrypoint: parse Collection from file. */
Collection pCollection(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.collection_;
  }
}

/* Entrypoint: parse Collection from string. */
Collection psCollection(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.collection_;
  }
}

/* Entrypoint: parse KeyValuePair from file. */
KeyValuePair pKeyValuePair(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.keyvaluepair_;
  }
}

/* Entrypoint: parse KeyValuePair from string. */
KeyValuePair psKeyValuePair(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.keyvaluepair_;
  }
}

/* Entrypoint: parse ListKeyValuePair from file. */
ListKeyValuePair pListKeyValuePair(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listkeyvaluepair_;
  }
}

/* Entrypoint: parse ListKeyValuePair from string. */
ListKeyValuePair psListKeyValuePair(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.listkeyvaluepair_;
  }
}

/* Entrypoint: parse Tuple from file. */
Tuple pTuple(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.tuple_;
  }
}

/* Entrypoint: parse Tuple from string. */
Tuple psTuple(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.tuple_;
  }
}

/* Entrypoint: parse ProcRemainder from file. */
ProcRemainder pProcRemainder(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.procremainder_;
  }
}

/* Entrypoint: parse ProcRemainder from string. */
ProcRemainder psProcRemainder(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.procremainder_;
  }
}

/* Entrypoint: parse NameRemainder from file. */
NameRemainder pNameRemainder(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.nameremainder_;
  }
}

/* Entrypoint: parse NameRemainder from string. */
NameRemainder psNameRemainder(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.nameremainder_;
  }
}

/* Entrypoint: parse VarRefKind from file. */
VarRefKind pVarRefKind(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.varrefkind_;
  }
}

/* Entrypoint: parse VarRefKind from string. */
VarRefKind psVarRefKind(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.varrefkind_;
  }
}

/* Entrypoint: parse SimpleType from file. */
SimpleType pSimpleType(FILE *inp)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
inp
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  int error = yyparse(scanner, &result);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.simpletype_;
  }
}

/* Entrypoint: parse SimpleType from string. */
SimpleType psSimpleType(const char *str)
{
  YYSTYPE result;
  yyscan_t scanner = rholang_mercury__init_lexer(
0
);
  if (!scanner) {
    fprintf(stderr, "Failed to initialize lexer.\n");
    return 0;
  }
  YY_BUFFER_STATE buf = rholang_mercury__scan_string(str, scanner);
  int error = yyparse(scanner, &result);
  rholang_mercury__delete_buffer(buf, scanner);
  rholang_mercury_lex_destroy(scanner);
  if (error)
  { /* Failure */
    return 0;
  }
  else
  { /* Success */
    return result.simpletype_;
  }
}


