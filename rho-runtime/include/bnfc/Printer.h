#ifndef PRINTER_HEADER
#define PRINTER_HEADER

#include "Absyn.h"

/* Certain applications may improve performance by changing the buffer size */
#define BUFFER_INITIAL 2048
/* You may wish to change _L_PAREN or _R_PAREN */
#define _L_PAREN '('
#define _R_PAREN ')'

/* The following are simple heuristics for rendering terminals */
/* You may wish to change them */
void renderCC(Char c);
void renderCS(String s);
void indent(void);
void backup(void);


char *printProc(Proc p);
char *printListProc(ListProc p);
char *printProcVar(ProcVar p);
char *printName(Name p);
char *printListName(ListName p);
char *printBundle(Bundle p);
char *printReceipt(Receipt p);
char *printReceiptLinearImpl(ReceiptLinearImpl p);
char *printLinearBind(LinearBind p);
char *printListLinearBind(ListLinearBind p);
char *printReceiptRepeatedImpl(ReceiptRepeatedImpl p);
char *printRepeatedBind(RepeatedBind p);
char *printListRepeatedBind(ListRepeatedBind p);
char *printReceiptPeekImpl(ReceiptPeekImpl p);
char *printPeekBind(PeekBind p);
char *printListPeekBind(ListPeekBind p);
char *printSend(Send p);
char *printBranch(Branch p);
char *printListBranch(ListBranch p);
char *printCase(Case p);
char *printListCase(ListCase p);
char *printNameDecl(NameDecl p);
char *printListNameDecl(ListNameDecl p);
char *printBoolLiteral(BoolLiteral p);
char *printGround(Ground p);
char *printCollection(Collection p);
char *printKeyValuePair(KeyValuePair p);
char *printListKeyValuePair(ListKeyValuePair p);
char *printTuple(Tuple p);
char *printProcRemainder(ProcRemainder p);
char *printNameRemainder(NameRemainder p);
char *printVarRefKind(VarRefKind p);
char *printSimpleType(SimpleType p);

void ppProc(Proc p, int i);
void ppListProc(ListProc p, int i);
void ppProcVar(ProcVar p, int i);
void ppName(Name p, int i);
void ppListName(ListName p, int i);
void ppBundle(Bundle p, int i);
void ppReceipt(Receipt p, int i);
void ppReceiptLinearImpl(ReceiptLinearImpl p, int i);
void ppLinearBind(LinearBind p, int i);
void ppListLinearBind(ListLinearBind p, int i);
void ppReceiptRepeatedImpl(ReceiptRepeatedImpl p, int i);
void ppRepeatedBind(RepeatedBind p, int i);
void ppListRepeatedBind(ListRepeatedBind p, int i);
void ppReceiptPeekImpl(ReceiptPeekImpl p, int i);
void ppPeekBind(PeekBind p, int i);
void ppListPeekBind(ListPeekBind p, int i);
void ppSend(Send p, int i);
void ppBranch(Branch p, int i);
void ppListBranch(ListBranch p, int i);
void ppCase(Case p, int i);
void ppListCase(ListCase p, int i);
void ppNameDecl(NameDecl p, int i);
void ppListNameDecl(ListNameDecl p, int i);
void ppBoolLiteral(BoolLiteral p, int i);
void ppGround(Ground p, int i);
void ppCollection(Collection p, int i);
void ppKeyValuePair(KeyValuePair p, int i);
void ppListKeyValuePair(ListKeyValuePair p, int i);
void ppTuple(Tuple p, int i);
void ppProcRemainder(ProcRemainder p, int i);
void ppNameRemainder(NameRemainder p, int i);
void ppVarRefKind(VarRefKind p, int i);
void ppSimpleType(SimpleType p, int i);

char *showProc(Proc p);
char *showListProc(ListProc p);
char *showProcVar(ProcVar p);
char *showName(Name p);
char *showListName(ListName p);
char *showBundle(Bundle p);
char *showReceipt(Receipt p);
char *showReceiptLinearImpl(ReceiptLinearImpl p);
char *showLinearBind(LinearBind p);
char *showListLinearBind(ListLinearBind p);
char *showReceiptRepeatedImpl(ReceiptRepeatedImpl p);
char *showRepeatedBind(RepeatedBind p);
char *showListRepeatedBind(ListRepeatedBind p);
char *showReceiptPeekImpl(ReceiptPeekImpl p);
char *showPeekBind(PeekBind p);
char *showListPeekBind(ListPeekBind p);
char *showSend(Send p);
char *showBranch(Branch p);
char *showListBranch(ListBranch p);
char *showCase(Case p);
char *showListCase(ListCase p);
char *showNameDecl(NameDecl p);
char *showListNameDecl(ListNameDecl p);
char *showBoolLiteral(BoolLiteral p);
char *showGround(Ground p);
char *showCollection(Collection p);
char *showKeyValuePair(KeyValuePair p);
char *showListKeyValuePair(ListKeyValuePair p);
char *showTuple(Tuple p);
char *showProcRemainder(ProcRemainder p);
char *showNameRemainder(NameRemainder p);
char *showVarRefKind(VarRefKind p);
char *showSimpleType(SimpleType p);

void shProc(Proc p);
void shListProc(ListProc p);
void shProcVar(ProcVar p);
void shName(Name p);
void shListName(ListName p);
void shBundle(Bundle p);
void shReceipt(Receipt p);
void shReceiptLinearImpl(ReceiptLinearImpl p);
void shLinearBind(LinearBind p);
void shListLinearBind(ListLinearBind p);
void shReceiptRepeatedImpl(ReceiptRepeatedImpl p);
void shRepeatedBind(RepeatedBind p);
void shListRepeatedBind(ListRepeatedBind p);
void shReceiptPeekImpl(ReceiptPeekImpl p);
void shPeekBind(PeekBind p);
void shListPeekBind(ListPeekBind p);
void shSend(Send p);
void shBranch(Branch p);
void shListBranch(ListBranch p);
void shCase(Case p);
void shListCase(ListCase p);
void shNameDecl(NameDecl p);
void shListNameDecl(ListNameDecl p);
void shBoolLiteral(BoolLiteral p);
void shGround(Ground p);
void shCollection(Collection p);
void shKeyValuePair(KeyValuePair p);
void shListKeyValuePair(ListKeyValuePair p);
void shTuple(Tuple p);
void shProcRemainder(ProcRemainder p);
void shNameRemainder(NameRemainder p);
void shVarRefKind(VarRefKind p);
void shSimpleType(SimpleType p);

void ppLongLiteral(String s, int i);
void ppStringLiteral(String s, int i);
void ppUriLiteral(String s, int i);
void ppVar(String s, int i);
void shLongLiteral(String s);
void shStringLiteral(String s);
void shUriLiteral(String s);
void shVar(String s);
void ppInteger(Integer n, int i);
void ppDouble(Double d, int i);
void ppChar(Char c, int i);
void ppString(String s, int i);
void ppIdent(String s, int i);
void shInteger(Integer n);
void shDouble(Double d);
void shChar(Char c);
void shString(String s);
void shIdent(String s);
void bufAppendS(const char *s);
void bufAppendC(const char c);
void bufReset(void);
void resizeBuffer(void);

#endif

