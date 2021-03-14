#ifndef PARSER_HEADER_FILE
#define PARSER_HEADER_FILE

#include "Absyn.h"

typedef union
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
} YYSTYPE;

typedef struct YYLTYPE
{
  int first_line;
  int first_column;
  int last_line;
  int last_column;
} YYLTYPE;

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


extern YYLTYPE yylloc;
extern YYSTYPE yylval;

Proc  pProc(FILE *inp);
Proc psProc(const char *str);
Proc  pProc1(FILE *inp);
Proc psProc1(const char *str);
Proc  pProc2(FILE *inp);
Proc psProc2(const char *str);
Proc  pProc3(FILE *inp);
Proc psProc3(const char *str);
Proc  pProc4(FILE *inp);
Proc psProc4(const char *str);
Proc  pProc5(FILE *inp);
Proc psProc5(const char *str);
Proc  pProc6(FILE *inp);
Proc psProc6(const char *str);
Proc  pProc7(FILE *inp);
Proc psProc7(const char *str);
Proc  pProc8(FILE *inp);
Proc psProc8(const char *str);
Proc  pProc9(FILE *inp);
Proc psProc9(const char *str);
Proc  pProc10(FILE *inp);
Proc psProc10(const char *str);
Proc  pProc11(FILE *inp);
Proc psProc11(const char *str);
Proc  pProc12(FILE *inp);
Proc psProc12(const char *str);
Proc  pProc13(FILE *inp);
Proc psProc13(const char *str);
Proc  pProc14(FILE *inp);
Proc psProc14(const char *str);
Proc  pProc15(FILE *inp);
Proc psProc15(const char *str);
Proc  pProc16(FILE *inp);
Proc psProc16(const char *str);
ListProc  pListProc(FILE *inp);
ListProc psListProc(const char *str);
ProcVar  pProcVar(FILE *inp);
ProcVar psProcVar(const char *str);
Name  pName(FILE *inp);
Name psName(const char *str);
ListName  pListName(FILE *inp);
ListName psListName(const char *str);
Bundle  pBundle(FILE *inp);
Bundle psBundle(const char *str);
Receipt  pReceipt(FILE *inp);
Receipt psReceipt(const char *str);
ReceiptLinearImpl  pReceiptLinearImpl(FILE *inp);
ReceiptLinearImpl psReceiptLinearImpl(const char *str);
LinearBind  pLinearBind(FILE *inp);
LinearBind psLinearBind(const char *str);
ListLinearBind  pListLinearBind(FILE *inp);
ListLinearBind psListLinearBind(const char *str);
ReceiptRepeatedImpl  pReceiptRepeatedImpl(FILE *inp);
ReceiptRepeatedImpl psReceiptRepeatedImpl(const char *str);
RepeatedBind  pRepeatedBind(FILE *inp);
RepeatedBind psRepeatedBind(const char *str);
ListRepeatedBind  pListRepeatedBind(FILE *inp);
ListRepeatedBind psListRepeatedBind(const char *str);
ReceiptPeekImpl  pReceiptPeekImpl(FILE *inp);
ReceiptPeekImpl psReceiptPeekImpl(const char *str);
PeekBind  pPeekBind(FILE *inp);
PeekBind psPeekBind(const char *str);
ListPeekBind  pListPeekBind(FILE *inp);
ListPeekBind psListPeekBind(const char *str);
Send  pSend(FILE *inp);
Send psSend(const char *str);
Branch  pBranch(FILE *inp);
Branch psBranch(const char *str);
ListBranch  pListBranch(FILE *inp);
ListBranch psListBranch(const char *str);
Case  pCase(FILE *inp);
Case psCase(const char *str);
ListCase  pListCase(FILE *inp);
ListCase psListCase(const char *str);
NameDecl  pNameDecl(FILE *inp);
NameDecl psNameDecl(const char *str);
ListNameDecl  pListNameDecl(FILE *inp);
ListNameDecl psListNameDecl(const char *str);
BoolLiteral  pBoolLiteral(FILE *inp);
BoolLiteral psBoolLiteral(const char *str);
Ground  pGround(FILE *inp);
Ground psGround(const char *str);
Collection  pCollection(FILE *inp);
Collection psCollection(const char *str);
KeyValuePair  pKeyValuePair(FILE *inp);
KeyValuePair psKeyValuePair(const char *str);
ListKeyValuePair  pListKeyValuePair(FILE *inp);
ListKeyValuePair psListKeyValuePair(const char *str);
Tuple  pTuple(FILE *inp);
Tuple psTuple(const char *str);
ProcRemainder  pProcRemainder(FILE *inp);
ProcRemainder psProcRemainder(const char *str);
NameRemainder  pNameRemainder(FILE *inp);
NameRemainder psNameRemainder(const char *str);
VarRefKind  pVarRefKind(FILE *inp);
VarRefKind psVarRefKind(const char *str);
SimpleType  pSimpleType(FILE *inp);
SimpleType psSimpleType(const char *str);

#endif
