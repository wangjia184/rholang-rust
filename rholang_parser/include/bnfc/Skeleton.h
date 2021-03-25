#ifndef SKELETON_HEADER
#define SKELETON_HEADER
/* You might want to change the above name. */

#include "Absyn.h"


void visitProc(Proc p);
void visitListProc(ListProc p);
void visitProcVar(ProcVar p);
void visitName(Name p);
void visitListName(ListName p);
void visitBundle(Bundle p);
void visitReceipt(Receipt p);
void visitReceiptLinearImpl(ReceiptLinearImpl p);
void visitLinearBind(LinearBind p);
void visitListLinearBind(ListLinearBind p);
void visitReceiptRepeatedImpl(ReceiptRepeatedImpl p);
void visitRepeatedBind(RepeatedBind p);
void visitListRepeatedBind(ListRepeatedBind p);
void visitReceiptPeekImpl(ReceiptPeekImpl p);
void visitPeekBind(PeekBind p);
void visitListPeekBind(ListPeekBind p);
void visitSend(Send p);
void visitBranch(Branch p);
void visitListBranch(ListBranch p);
void visitCase(Case p);
void visitListCase(ListCase p);
void visitNameDecl(NameDecl p);
void visitListNameDecl(ListNameDecl p);
void visitBoolLiteral(BoolLiteral p);
void visitGround(Ground p);
void visitCollection(Collection p);
void visitKeyValuePair(KeyValuePair p);
void visitListKeyValuePair(ListKeyValuePair p);
void visitTuple(Tuple p);
void visitProcRemainder(ProcRemainder p);
void visitNameRemainder(NameRemainder p);
void visitVarRefKind(VarRefKind p);
void visitSimpleType(SimpleType p);

void visitLongLiteral(LongLiteral p);void visitStringLiteral(StringLiteral p);void visitUriLiteral(UriLiteral p);void visitVar(Var p);
void visitIdent(Ident i);
void visitInteger(Integer i);
void visitDouble(Double d);
void visitChar(Char c);
void visitString(String s);

#endif

