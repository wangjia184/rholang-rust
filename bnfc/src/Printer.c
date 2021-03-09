/*** BNFC-Generated Pretty Printer and Abstract Syntax Viewer ***/

#include <ctype.h>   /* isspace */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "Printer.h"

#define INDENT_WIDTH 2

int _n_;
char *buf_;
int cur_;
int buf_size;

/* You may wish to change the renderC functions */
void renderC(Char c)
{
  if (c == '{')
  {
     bufAppendC('\n');
     indent();
     bufAppendC(c);
     _n_ = _n_ + INDENT_WIDTH;
     bufAppendC('\n');
     indent();
  }
  else if (c == '(' || c == '[')
     bufAppendC(c);
  else if (c == ')' || c == ']')
  {
     backup();
     bufAppendC(c);
     bufAppendC(' ');
  }
  else if (c == '}')
  {
     int t;
     _n_ = _n_ - INDENT_WIDTH;
     for(t=0; t<INDENT_WIDTH; t++) {
       backup();
     }
     bufAppendC(c);
     bufAppendC('\n');
     indent();
  }
  else if (c == ',')
  {
     backup();
     bufAppendC(c);
     bufAppendC(' ');
  }
  else if (c == ';')
  {
     backup();
     bufAppendC(c);
     bufAppendC('\n');
     indent();
  }
  else if (c == ' ') bufAppendC(c);
  else if (c == 0) return;
  else
  {
     bufAppendC(c);
     bufAppendC(' ');
  }
}

int allIsSpace(String s)
{
  char c;
  while ((c = *s++))
    if (! isspace(c)) return 0;
  return 1;
}

void renderS(String s)
{
  if (*s) /* s[0] != '\0', string s not empty */
  {
    if (allIsSpace(s)) {
      backup();
      bufAppendS(s);
    } else {
      bufAppendS(s);
      bufAppendC(' ');
    }
  }
}
void indent(void)
{
  int n = _n_;
  while (--n >= 0)
    bufAppendC(' ');
}
void backup(void)
{
  if (cur_ && buf_[cur_ - 1] == ' ')
    buf_[--cur_] = 0;
}
char *printProc(Proc p)
{
  _n_ = 0;
  bufReset();
  ppProc(p, 0);
  return buf_;
}
char *printListProc(ListProc p)
{
  _n_ = 0;
  bufReset();
  ppListProc(p, 0);
  return buf_;
}
char *printProcVar(ProcVar p)
{
  _n_ = 0;
  bufReset();
  ppProcVar(p, 0);
  return buf_;
}
char *printName(Name p)
{
  _n_ = 0;
  bufReset();
  ppName(p, 0);
  return buf_;
}
char *printListName(ListName p)
{
  _n_ = 0;
  bufReset();
  ppListName(p, 0);
  return buf_;
}
char *printBundle(Bundle p)
{
  _n_ = 0;
  bufReset();
  ppBundle(p, 0);
  return buf_;
}
char *printReceipt(Receipt p)
{
  _n_ = 0;
  bufReset();
  ppReceipt(p, 0);
  return buf_;
}
char *printReceiptLinearImpl(ReceiptLinearImpl p)
{
  _n_ = 0;
  bufReset();
  ppReceiptLinearImpl(p, 0);
  return buf_;
}
char *printLinearBind(LinearBind p)
{
  _n_ = 0;
  bufReset();
  ppLinearBind(p, 0);
  return buf_;
}
char *printListLinearBind(ListLinearBind p)
{
  _n_ = 0;
  bufReset();
  ppListLinearBind(p, 0);
  return buf_;
}
char *printReceiptRepeatedImpl(ReceiptRepeatedImpl p)
{
  _n_ = 0;
  bufReset();
  ppReceiptRepeatedImpl(p, 0);
  return buf_;
}
char *printRepeatedBind(RepeatedBind p)
{
  _n_ = 0;
  bufReset();
  ppRepeatedBind(p, 0);
  return buf_;
}
char *printListRepeatedBind(ListRepeatedBind p)
{
  _n_ = 0;
  bufReset();
  ppListRepeatedBind(p, 0);
  return buf_;
}
char *printReceiptPeekImpl(ReceiptPeekImpl p)
{
  _n_ = 0;
  bufReset();
  ppReceiptPeekImpl(p, 0);
  return buf_;
}
char *printPeekBind(PeekBind p)
{
  _n_ = 0;
  bufReset();
  ppPeekBind(p, 0);
  return buf_;
}
char *printListPeekBind(ListPeekBind p)
{
  _n_ = 0;
  bufReset();
  ppListPeekBind(p, 0);
  return buf_;
}
char *printSend(Send p)
{
  _n_ = 0;
  bufReset();
  ppSend(p, 0);
  return buf_;
}
char *printBranch(Branch p)
{
  _n_ = 0;
  bufReset();
  ppBranch(p, 0);
  return buf_;
}
char *printListBranch(ListBranch p)
{
  _n_ = 0;
  bufReset();
  ppListBranch(p, 0);
  return buf_;
}
char *printCase(Case p)
{
  _n_ = 0;
  bufReset();
  ppCase(p, 0);
  return buf_;
}
char *printListCase(ListCase p)
{
  _n_ = 0;
  bufReset();
  ppListCase(p, 0);
  return buf_;
}
char *printNameDecl(NameDecl p)
{
  _n_ = 0;
  bufReset();
  ppNameDecl(p, 0);
  return buf_;
}
char *printListNameDecl(ListNameDecl p)
{
  _n_ = 0;
  bufReset();
  ppListNameDecl(p, 0);
  return buf_;
}
char *printBoolLiteral(BoolLiteral p)
{
  _n_ = 0;
  bufReset();
  ppBoolLiteral(p, 0);
  return buf_;
}
char *printGround(Ground p)
{
  _n_ = 0;
  bufReset();
  ppGround(p, 0);
  return buf_;
}
char *printCollection(Collection p)
{
  _n_ = 0;
  bufReset();
  ppCollection(p, 0);
  return buf_;
}
char *printKeyValuePair(KeyValuePair p)
{
  _n_ = 0;
  bufReset();
  ppKeyValuePair(p, 0);
  return buf_;
}
char *printListKeyValuePair(ListKeyValuePair p)
{
  _n_ = 0;
  bufReset();
  ppListKeyValuePair(p, 0);
  return buf_;
}
char *printTuple(Tuple p)
{
  _n_ = 0;
  bufReset();
  ppTuple(p, 0);
  return buf_;
}
char *printProcRemainder(ProcRemainder p)
{
  _n_ = 0;
  bufReset();
  ppProcRemainder(p, 0);
  return buf_;
}
char *printNameRemainder(NameRemainder p)
{
  _n_ = 0;
  bufReset();
  ppNameRemainder(p, 0);
  return buf_;
}
char *printVarRefKind(VarRefKind p)
{
  _n_ = 0;
  bufReset();
  ppVarRefKind(p, 0);
  return buf_;
}
char *printSimpleType(SimpleType p)
{
  _n_ = 0;
  bufReset();
  ppSimpleType(p, 0);
  return buf_;
}
char *showProc(Proc p)
{
  _n_ = 0;
  bufReset();
  shProc(p);
  return buf_;
}
char *showListProc(ListProc p)
{
  _n_ = 0;
  bufReset();
  shListProc(p);
  return buf_;
}
char *showProcVar(ProcVar p)
{
  _n_ = 0;
  bufReset();
  shProcVar(p);
  return buf_;
}
char *showName(Name p)
{
  _n_ = 0;
  bufReset();
  shName(p);
  return buf_;
}
char *showListName(ListName p)
{
  _n_ = 0;
  bufReset();
  shListName(p);
  return buf_;
}
char *showBundle(Bundle p)
{
  _n_ = 0;
  bufReset();
  shBundle(p);
  return buf_;
}
char *showReceipt(Receipt p)
{
  _n_ = 0;
  bufReset();
  shReceipt(p);
  return buf_;
}
char *showReceiptLinearImpl(ReceiptLinearImpl p)
{
  _n_ = 0;
  bufReset();
  shReceiptLinearImpl(p);
  return buf_;
}
char *showLinearBind(LinearBind p)
{
  _n_ = 0;
  bufReset();
  shLinearBind(p);
  return buf_;
}
char *showListLinearBind(ListLinearBind p)
{
  _n_ = 0;
  bufReset();
  shListLinearBind(p);
  return buf_;
}
char *showReceiptRepeatedImpl(ReceiptRepeatedImpl p)
{
  _n_ = 0;
  bufReset();
  shReceiptRepeatedImpl(p);
  return buf_;
}
char *showRepeatedBind(RepeatedBind p)
{
  _n_ = 0;
  bufReset();
  shRepeatedBind(p);
  return buf_;
}
char *showListRepeatedBind(ListRepeatedBind p)
{
  _n_ = 0;
  bufReset();
  shListRepeatedBind(p);
  return buf_;
}
char *showReceiptPeekImpl(ReceiptPeekImpl p)
{
  _n_ = 0;
  bufReset();
  shReceiptPeekImpl(p);
  return buf_;
}
char *showPeekBind(PeekBind p)
{
  _n_ = 0;
  bufReset();
  shPeekBind(p);
  return buf_;
}
char *showListPeekBind(ListPeekBind p)
{
  _n_ = 0;
  bufReset();
  shListPeekBind(p);
  return buf_;
}
char *showSend(Send p)
{
  _n_ = 0;
  bufReset();
  shSend(p);
  return buf_;
}
char *showBranch(Branch p)
{
  _n_ = 0;
  bufReset();
  shBranch(p);
  return buf_;
}
char *showListBranch(ListBranch p)
{
  _n_ = 0;
  bufReset();
  shListBranch(p);
  return buf_;
}
char *showCase(Case p)
{
  _n_ = 0;
  bufReset();
  shCase(p);
  return buf_;
}
char *showListCase(ListCase p)
{
  _n_ = 0;
  bufReset();
  shListCase(p);
  return buf_;
}
char *showNameDecl(NameDecl p)
{
  _n_ = 0;
  bufReset();
  shNameDecl(p);
  return buf_;
}
char *showListNameDecl(ListNameDecl p)
{
  _n_ = 0;
  bufReset();
  shListNameDecl(p);
  return buf_;
}
char *showBoolLiteral(BoolLiteral p)
{
  _n_ = 0;
  bufReset();
  shBoolLiteral(p);
  return buf_;
}
char *showGround(Ground p)
{
  _n_ = 0;
  bufReset();
  shGround(p);
  return buf_;
}
char *showCollection(Collection p)
{
  _n_ = 0;
  bufReset();
  shCollection(p);
  return buf_;
}
char *showKeyValuePair(KeyValuePair p)
{
  _n_ = 0;
  bufReset();
  shKeyValuePair(p);
  return buf_;
}
char *showListKeyValuePair(ListKeyValuePair p)
{
  _n_ = 0;
  bufReset();
  shListKeyValuePair(p);
  return buf_;
}
char *showTuple(Tuple p)
{
  _n_ = 0;
  bufReset();
  shTuple(p);
  return buf_;
}
char *showProcRemainder(ProcRemainder p)
{
  _n_ = 0;
  bufReset();
  shProcRemainder(p);
  return buf_;
}
char *showNameRemainder(NameRemainder p)
{
  _n_ = 0;
  bufReset();
  shNameRemainder(p);
  return buf_;
}
char *showVarRefKind(VarRefKind p)
{
  _n_ = 0;
  bufReset();
  shVarRefKind(p);
  return buf_;
}
char *showSimpleType(SimpleType p)
{
  _n_ = 0;
  bufReset();
  shSimpleType(p);
  return buf_;
}
void ppProc(Proc p, int _i_)
{
  switch(p->kind)
  {
  case is_PPar:
    if (_i_ > 0) renderC(_L_PAREN);
    ppProc(p->u.ppar_.proc_1, 0);
    renderC('|');
    ppProc(p->u.ppar_.proc_2, 1);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_PIf:
    if (_i_ > 1) renderC(_L_PAREN);
    renderS("if");
    renderC('(');
    ppProc(p->u.pif_.proc_1, 0);
    renderC(')');
    ppProc(p->u.pif_.proc_2, 2);
    if (_i_ > 1) renderC(_R_PAREN);
    break;

  case is_PIfElse:
    if (_i_ > 1) renderC(_L_PAREN);
    renderS("if");
    renderC('(');
    ppProc(p->u.pifelse_.proc_1, 0);
    renderC(')');
    ppProc(p->u.pifelse_.proc_2, 2);
    renderS("else");
    ppProc(p->u.pifelse_.proc_3, 1);
    if (_i_ > 1) renderC(_R_PAREN);
    break;

  case is_PNew:
    if (_i_ > 1) renderC(_L_PAREN);
    renderS("new");
    ppListNameDecl(p->u.pnew_.listnamedecl_, 0);
    renderS("in");
    ppProc(p->u.pnew_.proc_, 1);
    if (_i_ > 1) renderC(_R_PAREN);
    break;

  case is_PContr:
    if (_i_ > 2) renderC(_L_PAREN);
    renderS("contract");
    ppName(p->u.pcontr_.name_, 0);
    renderC('(');
    ppListName(p->u.pcontr_.listname_, 0);
    ppNameRemainder(p->u.pcontr_.nameremainder_, 0);
    renderC(')');
    renderC('=');
    renderC('{');
    ppProc(p->u.pcontr_.proc_, 0);
    renderC('}');
    if (_i_ > 2) renderC(_R_PAREN);
    break;

  case is_PInput:
    if (_i_ > 2) renderC(_L_PAREN);
    renderS("for");
    renderC('(');
    ppReceipt(p->u.pinput_.receipt_, 0);
    renderC(')');
    renderC('{');
    ppProc(p->u.pinput_.proc_, 0);
    renderC('}');
    if (_i_ > 2) renderC(_R_PAREN);
    break;

  case is_PChoice:
    if (_i_ > 2) renderC(_L_PAREN);
    renderS("select");
    renderC('{');
    ppListBranch(p->u.pchoice_.listbranch_, 0);
    renderC('}');
    if (_i_ > 2) renderC(_R_PAREN);
    break;

  case is_PMatch:
    if (_i_ > 2) renderC(_L_PAREN);
    renderS("match");
    ppProc(p->u.pmatch_.proc_, 4);
    renderC('{');
    ppListCase(p->u.pmatch_.listcase_, 0);
    renderC('}');
    if (_i_ > 2) renderC(_R_PAREN);
    break;

  case is_PBundle:
    if (_i_ > 2) renderC(_L_PAREN);
    ppBundle(p->u.pbundle_.bundle_, 0);
    renderC('{');
    ppProc(p->u.pbundle_.proc_, 0);
    renderC('}');
    if (_i_ > 2) renderC(_R_PAREN);
    break;

  case is_PSend:
    if (_i_ > 3) renderC(_L_PAREN);
    ppName(p->u.psend_.name_, 0);
    ppSend(p->u.psend_.send_, 0);
    renderC('(');
    ppListProc(p->u.psend_.listproc_, 0);
    renderC(')');
    if (_i_ > 3) renderC(_R_PAREN);
    break;

  case is_POr:
    if (_i_ > 4) renderC(_L_PAREN);
    ppProc(p->u.por_.proc_1, 4);
    renderS("or");
    ppProc(p->u.por_.proc_2, 5);
    if (_i_ > 4) renderC(_R_PAREN);
    break;

  case is_PAnd:
    if (_i_ > 5) renderC(_L_PAREN);
    ppProc(p->u.pand_.proc_1, 5);
    renderS("and");
    ppProc(p->u.pand_.proc_2, 6);
    if (_i_ > 5) renderC(_R_PAREN);
    break;

  case is_PMatches:
    if (_i_ > 6) renderC(_L_PAREN);
    ppProc(p->u.pmatches_.proc_1, 7);
    renderS("matches");
    ppProc(p->u.pmatches_.proc_2, 7);
    if (_i_ > 6) renderC(_R_PAREN);
    break;

  case is_PEq:
    if (_i_ > 6) renderC(_L_PAREN);
    ppProc(p->u.peq_.proc_1, 6);
    renderS("==");
    ppProc(p->u.peq_.proc_2, 7);
    if (_i_ > 6) renderC(_R_PAREN);
    break;

  case is_PNeq:
    if (_i_ > 6) renderC(_L_PAREN);
    ppProc(p->u.pneq_.proc_1, 6);
    renderS("!=");
    ppProc(p->u.pneq_.proc_2, 7);
    if (_i_ > 6) renderC(_R_PAREN);
    break;

  case is_PLt:
    if (_i_ > 7) renderC(_L_PAREN);
    ppProc(p->u.plt_.proc_1, 7);
    renderC('<');
    ppProc(p->u.plt_.proc_2, 8);
    if (_i_ > 7) renderC(_R_PAREN);
    break;

  case is_PLte:
    if (_i_ > 7) renderC(_L_PAREN);
    ppProc(p->u.plte_.proc_1, 7);
    renderS("<=");
    ppProc(p->u.plte_.proc_2, 8);
    if (_i_ > 7) renderC(_R_PAREN);
    break;

  case is_PGt:
    if (_i_ > 7) renderC(_L_PAREN);
    ppProc(p->u.pgt_.proc_1, 7);
    renderC('>');
    ppProc(p->u.pgt_.proc_2, 8);
    if (_i_ > 7) renderC(_R_PAREN);
    break;

  case is_PGte:
    if (_i_ > 7) renderC(_L_PAREN);
    ppProc(p->u.pgte_.proc_1, 7);
    renderS(">=");
    ppProc(p->u.pgte_.proc_2, 8);
    if (_i_ > 7) renderC(_R_PAREN);
    break;

  case is_PAdd:
    if (_i_ > 8) renderC(_L_PAREN);
    ppProc(p->u.padd_.proc_1, 8);
    renderC('+');
    ppProc(p->u.padd_.proc_2, 9);
    if (_i_ > 8) renderC(_R_PAREN);
    break;

  case is_PMinus:
    if (_i_ > 8) renderC(_L_PAREN);
    ppProc(p->u.pminus_.proc_1, 8);
    renderC('-');
    ppProc(p->u.pminus_.proc_2, 9);
    if (_i_ > 8) renderC(_R_PAREN);
    break;

  case is_PPlusPlus:
    if (_i_ > 8) renderC(_L_PAREN);
    ppProc(p->u.pplusplus_.proc_1, 8);
    renderS("++");
    ppProc(p->u.pplusplus_.proc_2, 9);
    if (_i_ > 8) renderC(_R_PAREN);
    break;

  case is_PMinusMinus:
    if (_i_ > 8) renderC(_L_PAREN);
    ppProc(p->u.pminusminus_.proc_1, 8);
    renderS("--");
    ppProc(p->u.pminusminus_.proc_2, 9);
    if (_i_ > 8) renderC(_R_PAREN);
    break;

  case is_PMult:
    if (_i_ > 9) renderC(_L_PAREN);
    ppProc(p->u.pmult_.proc_1, 9);
    renderC('*');
    ppProc(p->u.pmult_.proc_2, 10);
    if (_i_ > 9) renderC(_R_PAREN);
    break;

  case is_PDiv:
    if (_i_ > 9) renderC(_L_PAREN);
    ppProc(p->u.pdiv_.proc_1, 9);
    renderC('/');
    ppProc(p->u.pdiv_.proc_2, 10);
    if (_i_ > 9) renderC(_R_PAREN);
    break;

  case is_PMod:
    if (_i_ > 9) renderC(_L_PAREN);
    ppProc(p->u.pmod_.proc_1, 9);
    renderC('%');
    ppProc(p->u.pmod_.proc_2, 10);
    if (_i_ > 9) renderC(_R_PAREN);
    break;

  case is_PPercentPercent:
    if (_i_ > 9) renderC(_L_PAREN);
    ppProc(p->u.ppercentpercent_.proc_1, 9);
    renderS("%%");
    ppProc(p->u.ppercentpercent_.proc_2, 10);
    if (_i_ > 9) renderC(_R_PAREN);
    break;

  case is_PNot:
    if (_i_ > 10) renderC(_L_PAREN);
    renderS("not");
    ppProc(p->u.pnot_.proc_, 10);
    if (_i_ > 10) renderC(_R_PAREN);
    break;

  case is_PNeg:
    if (_i_ > 10) renderC(_L_PAREN);
    renderC('-');
    ppProc(p->u.pneg_.proc_, 10);
    if (_i_ > 10) renderC(_R_PAREN);
    break;

  case is_PMethod:
    if (_i_ > 11) renderC(_L_PAREN);
    ppProc(p->u.pmethod_.proc_, 11);
    renderC('.');
    ppIdent(p->u.pmethod_.var_, 0);
    renderC('(');
    ppListProc(p->u.pmethod_.listproc_, 0);
    renderC(')');
    if (_i_ > 11) renderC(_R_PAREN);
    break;

  case is_PExprs:
    if (_i_ > 11) renderC(_L_PAREN);
    renderC('(');
    ppProc(p->u.pexprs_.proc_, 4);
    renderC(')');
    if (_i_ > 11) renderC(_R_PAREN);
    break;

  case is_PEval:
    if (_i_ > 12) renderC(_L_PAREN);
    renderC('*');
    ppName(p->u.peval_.name_, 0);
    if (_i_ > 12) renderC(_R_PAREN);
    break;

  case is_PVarRef:
    if (_i_ > 13) renderC(_L_PAREN);
    ppVarRefKind(p->u.pvarref_.varrefkind_, 0);
    ppIdent(p->u.pvarref_.var_, 0);
    if (_i_ > 13) renderC(_R_PAREN);
    break;

  case is_PDisjunction:
    if (_i_ > 13) renderC(_L_PAREN);
    ppProc(p->u.pdisjunction_.proc_1, 13);
    renderS("\\/");
    ppProc(p->u.pdisjunction_.proc_2, 14);
    if (_i_ > 13) renderC(_R_PAREN);
    break;

  case is_PConjunction:
    if (_i_ > 14) renderC(_L_PAREN);
    ppProc(p->u.pconjunction_.proc_1, 14);
    renderS("/\\");
    ppProc(p->u.pconjunction_.proc_2, 15);
    if (_i_ > 14) renderC(_R_PAREN);
    break;

  case is_PNegation:
    if (_i_ > 15) renderC(_L_PAREN);
    renderC('~');
    ppProc(p->u.pnegation_.proc_, 15);
    if (_i_ > 15) renderC(_R_PAREN);
    break;

  case is_PGround:
    if (_i_ > 16) renderC(_L_PAREN);
    ppGround(p->u.pground_.ground_, 0);
    if (_i_ > 16) renderC(_R_PAREN);
    break;

  case is_PCollect:
    if (_i_ > 16) renderC(_L_PAREN);
    ppCollection(p->u.pcollect_.collection_, 0);
    if (_i_ > 16) renderC(_R_PAREN);
    break;

  case is_PVar:
    if (_i_ > 16) renderC(_L_PAREN);
    ppProcVar(p->u.pvar_.procvar_, 0);
    if (_i_ > 16) renderC(_R_PAREN);
    break;

  case is_PNil:
    if (_i_ > 16) renderC(_L_PAREN);
    renderS("Nil");
    if (_i_ > 16) renderC(_R_PAREN);
    break;

  case is_PSimpleType:
    if (_i_ > 16) renderC(_L_PAREN);
    ppSimpleType(p->u.psimpletype_.simpletype_, 0);
    if (_i_ > 16) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Proc!\n");
    exit(1);
  }
}

void ppListProc(ListProc listproc, int i)
{
  while(listproc != 0)
  {
    if (listproc->listproc_ == 0)
    {
      ppProc(listproc->proc_, i);
      listproc = 0;
    }
    else
    {
      ppProc(listproc->proc_, i);
      renderC(',');
      listproc = listproc->listproc_;
    }
  }
}

void ppProcVar(ProcVar p, int _i_)
{
  switch(p->kind)
  {
  case is_ProcVarWildcard:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('_');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_ProcVarVar:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.procvarvar_.var_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ProcVar!\n");
    exit(1);
  }
}

void ppName(Name p, int _i_)
{
  switch(p->kind)
  {
  case is_NameWildcard:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('_');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_NameVar:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.namevar_.var_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_NameQuote:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('@');
    ppProc(p->u.namequote_.proc_, 12);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Name!\n");
    exit(1);
  }
}

void ppListName(ListName listname, int i)
{
  while(listname != 0)
  {
    if (listname->listname_ == 0)
    {
      ppName(listname->name_, i);
      listname = 0;
    }
    else
    {
      ppName(listname->name_, i);
      renderC(',');
      listname = listname->listname_;
    }
  }
}

void ppBundle(Bundle p, int _i_)
{
  switch(p->kind)
  {
  case is_BundleWrite:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("bundle+");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_BundleRead:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("bundle-");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_BundleEquiv:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("bundle0");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_BundleReadWrite:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("bundle");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Bundle!\n");
    exit(1);
  }
}

void ppReceipt(Receipt p, int _i_)
{
  switch(p->kind)
  {
  case is_ReceiptLinear:
    if (_i_ > 0) renderC(_L_PAREN);
    ppReceiptLinearImpl(p->u.receiptlinear_.receiptlinearimpl_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_ReceiptRepeated:
    if (_i_ > 0) renderC(_L_PAREN);
    ppReceiptRepeatedImpl(p->u.receiptrepeated_.receiptrepeatedimpl_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_ReceiptPeek:
    if (_i_ > 0) renderC(_L_PAREN);
    ppReceiptPeekImpl(p->u.receiptpeek_.receiptpeekimpl_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Receipt!\n");
    exit(1);
  }
}

void ppReceiptLinearImpl(ReceiptLinearImpl p, int _i_)
{
  switch(p->kind)
  {
  case is_LinearSimple:
    if (_i_ > 0) renderC(_L_PAREN);
    ppListLinearBind(p->u.linearsimple_.listlinearbind_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ReceiptLinearImpl!\n");
    exit(1);
  }
}

void ppLinearBind(LinearBind p, int _i_)
{
  switch(p->kind)
  {
  case is_LinearBindImpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppListName(p->u.linearbindimpl_.listname_, 0);
    ppNameRemainder(p->u.linearbindimpl_.nameremainder_, 0);
    renderS("<-");
    ppName(p->u.linearbindimpl_.name_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing LinearBind!\n");
    exit(1);
  }
}

void ppListLinearBind(ListLinearBind listlinearbind, int i)
{
  while(listlinearbind != 0)
  {
    if (listlinearbind->listlinearbind_ == 0)
    {
      ppLinearBind(listlinearbind->linearbind_, i);
      listlinearbind = 0;
    }
    else
    {
      ppLinearBind(listlinearbind->linearbind_, i);
      renderC(';');
      listlinearbind = listlinearbind->listlinearbind_;
    }
  }
}

void ppReceiptRepeatedImpl(ReceiptRepeatedImpl p, int _i_)
{
  switch(p->kind)
  {
  case is_RepeatedSimple:
    if (_i_ > 0) renderC(_L_PAREN);
    ppListRepeatedBind(p->u.repeatedsimple_.listrepeatedbind_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ReceiptRepeatedImpl!\n");
    exit(1);
  }
}

void ppRepeatedBind(RepeatedBind p, int _i_)
{
  switch(p->kind)
  {
  case is_RepeatedBindImpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppListName(p->u.repeatedbindimpl_.listname_, 0);
    ppNameRemainder(p->u.repeatedbindimpl_.nameremainder_, 0);
    renderS("<=");
    ppName(p->u.repeatedbindimpl_.name_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing RepeatedBind!\n");
    exit(1);
  }
}

void ppListRepeatedBind(ListRepeatedBind listrepeatedbind, int i)
{
  while(listrepeatedbind != 0)
  {
    if (listrepeatedbind->listrepeatedbind_ == 0)
    {
      ppRepeatedBind(listrepeatedbind->repeatedbind_, i);
      listrepeatedbind = 0;
    }
    else
    {
      ppRepeatedBind(listrepeatedbind->repeatedbind_, i);
      renderC(';');
      listrepeatedbind = listrepeatedbind->listrepeatedbind_;
    }
  }
}

void ppReceiptPeekImpl(ReceiptPeekImpl p, int _i_)
{
  switch(p->kind)
  {
  case is_PeekSimple:
    if (_i_ > 0) renderC(_L_PAREN);
    ppListPeekBind(p->u.peeksimple_.listpeekbind_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ReceiptPeekImpl!\n");
    exit(1);
  }
}

void ppPeekBind(PeekBind p, int _i_)
{
  switch(p->kind)
  {
  case is_PeekBindImpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppListName(p->u.peekbindimpl_.listname_, 0);
    ppNameRemainder(p->u.peekbindimpl_.nameremainder_, 0);
    renderS("<<-");
    ppName(p->u.peekbindimpl_.name_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing PeekBind!\n");
    exit(1);
  }
}

void ppListPeekBind(ListPeekBind listpeekbind, int i)
{
  while(listpeekbind != 0)
  {
    if (listpeekbind->listpeekbind_ == 0)
    {
      ppPeekBind(listpeekbind->peekbind_, i);
      listpeekbind = 0;
    }
    else
    {
      ppPeekBind(listpeekbind->peekbind_, i);
      renderC(';');
      listpeekbind = listpeekbind->listpeekbind_;
    }
  }
}

void ppSend(Send p, int _i_)
{
  switch(p->kind)
  {
  case is_SendSingle:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('!');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_SendMultiple:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("!!");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Send!\n");
    exit(1);
  }
}

void ppBranch(Branch p, int _i_)
{
  switch(p->kind)
  {
  case is_BranchImpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppReceiptLinearImpl(p->u.branchimpl_.receiptlinearimpl_, 0);
    renderS("=>");
    ppProc(p->u.branchimpl_.proc_, 3);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Branch!\n");
    exit(1);
  }
}

void ppListBranch(ListBranch listbranch, int i)
{
  while(listbranch != 0)
  {
    if (listbranch->listbranch_ == 0)
    {
      ppBranch(listbranch->branch_, i);
      listbranch = 0;
    }
    else
    {
      ppBranch(listbranch->branch_, i);
      renderS("");
      listbranch = listbranch->listbranch_;
    }
  }
}

void ppCase(Case p, int _i_)
{
  switch(p->kind)
  {
  case is_CaseImpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppProc(p->u.caseimpl_.proc_1, 13);
    renderS("=>");
    ppProc(p->u.caseimpl_.proc_2, 3);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Case!\n");
    exit(1);
  }
}

void ppListCase(ListCase listcase, int i)
{
  while(listcase != 0)
  {
    if (listcase->listcase_ == 0)
    {
      ppCase(listcase->case_, i);
      listcase = 0;
    }
    else
    {
      ppCase(listcase->case_, i);
      renderS("");
      listcase = listcase->listcase_;
    }
  }
}

void ppNameDecl(NameDecl p, int _i_)
{
  switch(p->kind)
  {
  case is_NameDeclSimpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.namedeclsimpl_.var_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_NameDeclUrn:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.namedeclurn_.var_, 0);
    renderC('(');
    ppIdent(p->u.namedeclurn_.uriliteral_, 0);
    renderC(')');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing NameDecl!\n");
    exit(1);
  }
}

void ppListNameDecl(ListNameDecl listnamedecl, int i)
{
  while(listnamedecl != 0)
  {
    if (listnamedecl->listnamedecl_ == 0)
    {
      ppNameDecl(listnamedecl->namedecl_, i);
      listnamedecl = 0;
    }
    else
    {
      ppNameDecl(listnamedecl->namedecl_, i);
      renderC(',');
      listnamedecl = listnamedecl->listnamedecl_;
    }
  }
}

void ppBoolLiteral(BoolLiteral p, int _i_)
{
  switch(p->kind)
  {
  case is_BoolTrue:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("true");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_BoolFalse:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("false");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing BoolLiteral!\n");
    exit(1);
  }
}

void ppGround(Ground p, int _i_)
{
  switch(p->kind)
  {
  case is_GroundBool:
    if (_i_ > 0) renderC(_L_PAREN);
    ppBoolLiteral(p->u.groundbool_.boolliteral_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_GroundInt:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.groundint_.longliteral_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_GroundString:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.groundstring_.stringliteral_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_GroundUri:
    if (_i_ > 0) renderC(_L_PAREN);
    ppIdent(p->u.grounduri_.uriliteral_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Ground!\n");
    exit(1);
  }
}

void ppCollection(Collection p, int _i_)
{
  switch(p->kind)
  {
  case is_CollectList:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('[');
    ppListProc(p->u.collectlist_.listproc_, 0);
    ppProcRemainder(p->u.collectlist_.procremainder_, 0);
    renderC(']');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_CollectTuple:
    if (_i_ > 0) renderC(_L_PAREN);
    ppTuple(p->u.collecttuple_.tuple_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_CollectSet:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("Set");
    renderC('(');
    ppListProc(p->u.collectset_.listproc_, 0);
    ppProcRemainder(p->u.collectset_.procremainder_, 0);
    renderC(')');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_CollectMap:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('{');
    ppListKeyValuePair(p->u.collectmap_.listkeyvaluepair_, 0);
    ppProcRemainder(p->u.collectmap_.procremainder_, 0);
    renderC('}');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Collection!\n");
    exit(1);
  }
}

void ppKeyValuePair(KeyValuePair p, int _i_)
{
  switch(p->kind)
  {
  case is_KeyValuePairImpl:
    if (_i_ > 0) renderC(_L_PAREN);
    ppProc(p->u.keyvaluepairimpl_.proc_1, 0);
    renderC(':');
    ppProc(p->u.keyvaluepairimpl_.proc_2, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing KeyValuePair!\n");
    exit(1);
  }
}

void ppListKeyValuePair(ListKeyValuePair listkeyvaluepair, int i)
{
  while(listkeyvaluepair != 0)
  {
    if (listkeyvaluepair->listkeyvaluepair_ == 0)
    {
      ppKeyValuePair(listkeyvaluepair->keyvaluepair_, i);
      listkeyvaluepair = 0;
    }
    else
    {
      ppKeyValuePair(listkeyvaluepair->keyvaluepair_, i);
      renderC(',');
      listkeyvaluepair = listkeyvaluepair->listkeyvaluepair_;
    }
  }
}

void ppTuple(Tuple p, int _i_)
{
  switch(p->kind)
  {
  case is_TupleSingle:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('(');
    ppProc(p->u.tuplesingle_.proc_, 0);
    renderS(",)");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_TupleMultiple:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('(');
    ppProc(p->u.tuplemultiple_.proc_, 0);
    renderC(',');
    ppListProc(p->u.tuplemultiple_.listproc_, 0);
    renderC(')');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Tuple!\n");
    exit(1);
  }
}

void ppProcRemainder(ProcRemainder p, int _i_)
{
  switch(p->kind)
  {
  case is_ProcRemainderVar:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("...");
    ppProcVar(p->u.procremaindervar_.procvar_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_ProcRemainderEmpty:
    if (_i_ > 0) renderC(_L_PAREN);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ProcRemainder!\n");
    exit(1);
  }
}

void ppNameRemainder(NameRemainder p, int _i_)
{
  switch(p->kind)
  {
  case is_NameRemainderVar:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("...");
    renderC('@');
    ppProcVar(p->u.nameremaindervar_.procvar_, 0);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_NameRemainderEmpty:
    if (_i_ > 0) renderC(_L_PAREN);
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing NameRemainder!\n");
    exit(1);
  }
}

void ppVarRefKind(VarRefKind p, int _i_)
{
  switch(p->kind)
  {
  case is_VarRefKindProc:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('=');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_VarRefKindName:
    if (_i_ > 0) renderC(_L_PAREN);
    renderC('=');
    renderC('*');
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing VarRefKind!\n");
    exit(1);
  }
}

void ppSimpleType(SimpleType p, int _i_)
{
  switch(p->kind)
  {
  case is_SimpleTypeBool:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("Bool");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_SimpleTypeInt:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("Int");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_SimpleTypeString:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("String");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_SimpleTypeUri:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("Uri");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  case is_SimpleTypeByteArray:
    if (_i_ > 0) renderC(_L_PAREN);
    renderS("ByteArray");
    if (_i_ > 0) renderC(_R_PAREN);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing SimpleType!\n");
    exit(1);
  }
}

void ppInteger(Integer n, int i)
{
  char tmp[20];
  sprintf(tmp, "%d", n);
  renderS(tmp);
}
void ppDouble(Double d, int i)
{
  char tmp[24];
  sprintf(tmp, "%.15g", d);
  renderS(tmp);
}
void ppChar(Char c, int i)
{
  char tmp[4];
  sprintf(tmp, "'%c'", c);
  renderS(tmp);
}
void ppString(String s, int i)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
  bufAppendC(' ');
}
void ppIdent(String s, int i)
{
  renderS(s);
}

void ppLongLiteral(String s, int i)
{
  renderS(s);
}


void ppStringLiteral(String s, int i)
{
  renderS(s);
}


void ppUriLiteral(String s, int i)
{
  renderS(s);
}


void ppVar(String s, int i)
{
  renderS(s);
}


void shProc(Proc p)
{
  switch(p->kind)
  {
  case is_PPar:
    bufAppendC('(');

    bufAppendS("PPar");

    bufAppendC(' ');

    shProc(p->u.ppar_.proc_1);
  bufAppendC(' ');
    shProc(p->u.ppar_.proc_2);

    bufAppendC(')');

    break;
  case is_PIf:
    bufAppendC('(');

    bufAppendS("PIf");

    bufAppendC(' ');

    shProc(p->u.pif_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pif_.proc_2);

    bufAppendC(')');

    break;
  case is_PIfElse:
    bufAppendC('(');

    bufAppendS("PIfElse");

    bufAppendC(' ');

    shProc(p->u.pifelse_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pifelse_.proc_2);
  bufAppendC(' ');
    shProc(p->u.pifelse_.proc_3);

    bufAppendC(')');

    break;
  case is_PNew:
    bufAppendC('(');

    bufAppendS("PNew");

    bufAppendC(' ');

    shListNameDecl(p->u.pnew_.listnamedecl_);
  bufAppendC(' ');
    shProc(p->u.pnew_.proc_);

    bufAppendC(')');

    break;
  case is_PContr:
    bufAppendC('(');

    bufAppendS("PContr");

    bufAppendC(' ');

    shName(p->u.pcontr_.name_);
  bufAppendC(' ');
    shListName(p->u.pcontr_.listname_);
  bufAppendC(' ');
    shNameRemainder(p->u.pcontr_.nameremainder_);
  bufAppendC(' ');
    shProc(p->u.pcontr_.proc_);

    bufAppendC(')');

    break;
  case is_PInput:
    bufAppendC('(');

    bufAppendS("PInput");

    bufAppendC(' ');

    shReceipt(p->u.pinput_.receipt_);
  bufAppendC(' ');
    shProc(p->u.pinput_.proc_);

    bufAppendC(')');

    break;
  case is_PChoice:
    bufAppendC('(');

    bufAppendS("PChoice");

    bufAppendC(' ');

    shListBranch(p->u.pchoice_.listbranch_);

    bufAppendC(')');

    break;
  case is_PMatch:
    bufAppendC('(');

    bufAppendS("PMatch");

    bufAppendC(' ');

    shProc(p->u.pmatch_.proc_);
  bufAppendC(' ');
    shListCase(p->u.pmatch_.listcase_);

    bufAppendC(')');

    break;
  case is_PBundle:
    bufAppendC('(');

    bufAppendS("PBundle");

    bufAppendC(' ');

    shBundle(p->u.pbundle_.bundle_);
  bufAppendC(' ');
    shProc(p->u.pbundle_.proc_);

    bufAppendC(')');

    break;
  case is_PSend:
    bufAppendC('(');

    bufAppendS("PSend");

    bufAppendC(' ');

    shName(p->u.psend_.name_);
  bufAppendC(' ');
    shSend(p->u.psend_.send_);
  bufAppendC(' ');
    shListProc(p->u.psend_.listproc_);

    bufAppendC(')');

    break;
  case is_POr:
    bufAppendC('(');

    bufAppendS("POr");

    bufAppendC(' ');

    shProc(p->u.por_.proc_1);
  bufAppendC(' ');
    shProc(p->u.por_.proc_2);

    bufAppendC(')');

    break;
  case is_PAnd:
    bufAppendC('(');

    bufAppendS("PAnd");

    bufAppendC(' ');

    shProc(p->u.pand_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pand_.proc_2);

    bufAppendC(')');

    break;
  case is_PMatches:
    bufAppendC('(');

    bufAppendS("PMatches");

    bufAppendC(' ');

    shProc(p->u.pmatches_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pmatches_.proc_2);

    bufAppendC(')');

    break;
  case is_PEq:
    bufAppendC('(');

    bufAppendS("PEq");

    bufAppendC(' ');

    shProc(p->u.peq_.proc_1);
  bufAppendC(' ');
    shProc(p->u.peq_.proc_2);

    bufAppendC(')');

    break;
  case is_PNeq:
    bufAppendC('(');

    bufAppendS("PNeq");

    bufAppendC(' ');

    shProc(p->u.pneq_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pneq_.proc_2);

    bufAppendC(')');

    break;
  case is_PLt:
    bufAppendC('(');

    bufAppendS("PLt");

    bufAppendC(' ');

    shProc(p->u.plt_.proc_1);
  bufAppendC(' ');
    shProc(p->u.plt_.proc_2);

    bufAppendC(')');

    break;
  case is_PLte:
    bufAppendC('(');

    bufAppendS("PLte");

    bufAppendC(' ');

    shProc(p->u.plte_.proc_1);
  bufAppendC(' ');
    shProc(p->u.plte_.proc_2);

    bufAppendC(')');

    break;
  case is_PGt:
    bufAppendC('(');

    bufAppendS("PGt");

    bufAppendC(' ');

    shProc(p->u.pgt_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pgt_.proc_2);

    bufAppendC(')');

    break;
  case is_PGte:
    bufAppendC('(');

    bufAppendS("PGte");

    bufAppendC(' ');

    shProc(p->u.pgte_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pgte_.proc_2);

    bufAppendC(')');

    break;
  case is_PAdd:
    bufAppendC('(');

    bufAppendS("PAdd");

    bufAppendC(' ');

    shProc(p->u.padd_.proc_1);
  bufAppendC(' ');
    shProc(p->u.padd_.proc_2);

    bufAppendC(')');

    break;
  case is_PMinus:
    bufAppendC('(');

    bufAppendS("PMinus");

    bufAppendC(' ');

    shProc(p->u.pminus_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pminus_.proc_2);

    bufAppendC(')');

    break;
  case is_PPlusPlus:
    bufAppendC('(');

    bufAppendS("PPlusPlus");

    bufAppendC(' ');

    shProc(p->u.pplusplus_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pplusplus_.proc_2);

    bufAppendC(')');

    break;
  case is_PMinusMinus:
    bufAppendC('(');

    bufAppendS("PMinusMinus");

    bufAppendC(' ');

    shProc(p->u.pminusminus_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pminusminus_.proc_2);

    bufAppendC(')');

    break;
  case is_PMult:
    bufAppendC('(');

    bufAppendS("PMult");

    bufAppendC(' ');

    shProc(p->u.pmult_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pmult_.proc_2);

    bufAppendC(')');

    break;
  case is_PDiv:
    bufAppendC('(');

    bufAppendS("PDiv");

    bufAppendC(' ');

    shProc(p->u.pdiv_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pdiv_.proc_2);

    bufAppendC(')');

    break;
  case is_PMod:
    bufAppendC('(');

    bufAppendS("PMod");

    bufAppendC(' ');

    shProc(p->u.pmod_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pmod_.proc_2);

    bufAppendC(')');

    break;
  case is_PPercentPercent:
    bufAppendC('(');

    bufAppendS("PPercentPercent");

    bufAppendC(' ');

    shProc(p->u.ppercentpercent_.proc_1);
  bufAppendC(' ');
    shProc(p->u.ppercentpercent_.proc_2);

    bufAppendC(')');

    break;
  case is_PNot:
    bufAppendC('(');

    bufAppendS("PNot");

    bufAppendC(' ');

    shProc(p->u.pnot_.proc_);

    bufAppendC(')');

    break;
  case is_PNeg:
    bufAppendC('(');

    bufAppendS("PNeg");

    bufAppendC(' ');

    shProc(p->u.pneg_.proc_);

    bufAppendC(')');

    break;
  case is_PMethod:
    bufAppendC('(');

    bufAppendS("PMethod");

    bufAppendC(' ');

    shProc(p->u.pmethod_.proc_);
  bufAppendC(' ');
    shIdent(p->u.pmethod_.var_);
  bufAppendC(' ');
    shListProc(p->u.pmethod_.listproc_);

    bufAppendC(')');

    break;
  case is_PExprs:
    bufAppendC('(');

    bufAppendS("PExprs");

    bufAppendC(' ');

    shProc(p->u.pexprs_.proc_);

    bufAppendC(')');

    break;
  case is_PEval:
    bufAppendC('(');

    bufAppendS("PEval");

    bufAppendC(' ');

    shName(p->u.peval_.name_);

    bufAppendC(')');

    break;
  case is_PVarRef:
    bufAppendC('(');

    bufAppendS("PVarRef");

    bufAppendC(' ');

    shVarRefKind(p->u.pvarref_.varrefkind_);
  bufAppendC(' ');
    shIdent(p->u.pvarref_.var_);

    bufAppendC(')');

    break;
  case is_PDisjunction:
    bufAppendC('(');

    bufAppendS("PDisjunction");

    bufAppendC(' ');

    shProc(p->u.pdisjunction_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pdisjunction_.proc_2);

    bufAppendC(')');

    break;
  case is_PConjunction:
    bufAppendC('(');

    bufAppendS("PConjunction");

    bufAppendC(' ');

    shProc(p->u.pconjunction_.proc_1);
  bufAppendC(' ');
    shProc(p->u.pconjunction_.proc_2);

    bufAppendC(')');

    break;
  case is_PNegation:
    bufAppendC('(');

    bufAppendS("PNegation");

    bufAppendC(' ');

    shProc(p->u.pnegation_.proc_);

    bufAppendC(')');

    break;
  case is_PGround:
    bufAppendC('(');

    bufAppendS("PGround");

    bufAppendC(' ');

    shGround(p->u.pground_.ground_);

    bufAppendC(')');

    break;
  case is_PCollect:
    bufAppendC('(');

    bufAppendS("PCollect");

    bufAppendC(' ');

    shCollection(p->u.pcollect_.collection_);

    bufAppendC(')');

    break;
  case is_PVar:
    bufAppendC('(');

    bufAppendS("PVar");

    bufAppendC(' ');

    shProcVar(p->u.pvar_.procvar_);

    bufAppendC(')');

    break;
  case is_PNil:

    bufAppendS("PNil");




    break;
  case is_PSimpleType:
    bufAppendC('(');

    bufAppendS("PSimpleType");

    bufAppendC(' ');

    shSimpleType(p->u.psimpletype_.simpletype_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Proc!\n");
    exit(1);
  }
}

void shListProc(ListProc listproc)
{
  bufAppendC('[');
  while(listproc != 0)
  {
    if (listproc->listproc_)
    {
      shProc(listproc->proc_);
      bufAppendS(", ");
      listproc = listproc->listproc_;
    }
    else
    {
      shProc(listproc->proc_);
      listproc = 0;
    }
  }
  bufAppendC(']');
}

void shProcVar(ProcVar p)
{
  switch(p->kind)
  {
  case is_ProcVarWildcard:

    bufAppendS("ProcVarWildcard");




    break;
  case is_ProcVarVar:
    bufAppendC('(');

    bufAppendS("ProcVarVar");

    bufAppendC(' ');

    shIdent(p->u.procvarvar_.var_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing ProcVar!\n");
    exit(1);
  }
}

void shName(Name p)
{
  switch(p->kind)
  {
  case is_NameWildcard:

    bufAppendS("NameWildcard");




    break;
  case is_NameVar:
    bufAppendC('(');

    bufAppendS("NameVar");

    bufAppendC(' ');

    shIdent(p->u.namevar_.var_);

    bufAppendC(')');

    break;
  case is_NameQuote:
    bufAppendC('(');

    bufAppendS("NameQuote");

    bufAppendC(' ');

    shProc(p->u.namequote_.proc_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Name!\n");
    exit(1);
  }
}

void shListName(ListName listname)
{
  bufAppendC('[');
  while(listname != 0)
  {
    if (listname->listname_)
    {
      shName(listname->name_);
      bufAppendS(", ");
      listname = listname->listname_;
    }
    else
    {
      shName(listname->name_);
      listname = 0;
    }
  }
  bufAppendC(']');
}

void shBundle(Bundle p)
{
  switch(p->kind)
  {
  case is_BundleWrite:

    bufAppendS("BundleWrite");




    break;
  case is_BundleRead:

    bufAppendS("BundleRead");




    break;
  case is_BundleEquiv:

    bufAppendS("BundleEquiv");




    break;
  case is_BundleReadWrite:

    bufAppendS("BundleReadWrite");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Bundle!\n");
    exit(1);
  }
}

void shReceipt(Receipt p)
{
  switch(p->kind)
  {
  case is_ReceiptLinear:
    bufAppendC('(');

    bufAppendS("ReceiptLinear");

    bufAppendC(' ');

    shReceiptLinearImpl(p->u.receiptlinear_.receiptlinearimpl_);

    bufAppendC(')');

    break;
  case is_ReceiptRepeated:
    bufAppendC('(');

    bufAppendS("ReceiptRepeated");

    bufAppendC(' ');

    shReceiptRepeatedImpl(p->u.receiptrepeated_.receiptrepeatedimpl_);

    bufAppendC(')');

    break;
  case is_ReceiptPeek:
    bufAppendC('(');

    bufAppendS("ReceiptPeek");

    bufAppendC(' ');

    shReceiptPeekImpl(p->u.receiptpeek_.receiptpeekimpl_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Receipt!\n");
    exit(1);
  }
}

void shReceiptLinearImpl(ReceiptLinearImpl p)
{
  switch(p->kind)
  {
  case is_LinearSimple:
    bufAppendC('(');

    bufAppendS("LinearSimple");

    bufAppendC(' ');

    shListLinearBind(p->u.linearsimple_.listlinearbind_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing ReceiptLinearImpl!\n");
    exit(1);
  }
}

void shLinearBind(LinearBind p)
{
  switch(p->kind)
  {
  case is_LinearBindImpl:
    bufAppendC('(');

    bufAppendS("LinearBindImpl");

    bufAppendC(' ');

    shListName(p->u.linearbindimpl_.listname_);
  bufAppendC(' ');
    shNameRemainder(p->u.linearbindimpl_.nameremainder_);
  bufAppendC(' ');
    shName(p->u.linearbindimpl_.name_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing LinearBind!\n");
    exit(1);
  }
}

void shListLinearBind(ListLinearBind listlinearbind)
{
  bufAppendC('[');
  while(listlinearbind != 0)
  {
    if (listlinearbind->listlinearbind_)
    {
      shLinearBind(listlinearbind->linearbind_);
      bufAppendS(", ");
      listlinearbind = listlinearbind->listlinearbind_;
    }
    else
    {
      shLinearBind(listlinearbind->linearbind_);
      listlinearbind = 0;
    }
  }
  bufAppendC(']');
}

void shReceiptRepeatedImpl(ReceiptRepeatedImpl p)
{
  switch(p->kind)
  {
  case is_RepeatedSimple:
    bufAppendC('(');

    bufAppendS("RepeatedSimple");

    bufAppendC(' ');

    shListRepeatedBind(p->u.repeatedsimple_.listrepeatedbind_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing ReceiptRepeatedImpl!\n");
    exit(1);
  }
}

void shRepeatedBind(RepeatedBind p)
{
  switch(p->kind)
  {
  case is_RepeatedBindImpl:
    bufAppendC('(');

    bufAppendS("RepeatedBindImpl");

    bufAppendC(' ');

    shListName(p->u.repeatedbindimpl_.listname_);
  bufAppendC(' ');
    shNameRemainder(p->u.repeatedbindimpl_.nameremainder_);
  bufAppendC(' ');
    shName(p->u.repeatedbindimpl_.name_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing RepeatedBind!\n");
    exit(1);
  }
}

void shListRepeatedBind(ListRepeatedBind listrepeatedbind)
{
  bufAppendC('[');
  while(listrepeatedbind != 0)
  {
    if (listrepeatedbind->listrepeatedbind_)
    {
      shRepeatedBind(listrepeatedbind->repeatedbind_);
      bufAppendS(", ");
      listrepeatedbind = listrepeatedbind->listrepeatedbind_;
    }
    else
    {
      shRepeatedBind(listrepeatedbind->repeatedbind_);
      listrepeatedbind = 0;
    }
  }
  bufAppendC(']');
}

void shReceiptPeekImpl(ReceiptPeekImpl p)
{
  switch(p->kind)
  {
  case is_PeekSimple:
    bufAppendC('(');

    bufAppendS("PeekSimple");

    bufAppendC(' ');

    shListPeekBind(p->u.peeksimple_.listpeekbind_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing ReceiptPeekImpl!\n");
    exit(1);
  }
}

void shPeekBind(PeekBind p)
{
  switch(p->kind)
  {
  case is_PeekBindImpl:
    bufAppendC('(');

    bufAppendS("PeekBindImpl");

    bufAppendC(' ');

    shListName(p->u.peekbindimpl_.listname_);
  bufAppendC(' ');
    shNameRemainder(p->u.peekbindimpl_.nameremainder_);
  bufAppendC(' ');
    shName(p->u.peekbindimpl_.name_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing PeekBind!\n");
    exit(1);
  }
}

void shListPeekBind(ListPeekBind listpeekbind)
{
  bufAppendC('[');
  while(listpeekbind != 0)
  {
    if (listpeekbind->listpeekbind_)
    {
      shPeekBind(listpeekbind->peekbind_);
      bufAppendS(", ");
      listpeekbind = listpeekbind->listpeekbind_;
    }
    else
    {
      shPeekBind(listpeekbind->peekbind_);
      listpeekbind = 0;
    }
  }
  bufAppendC(']');
}

void shSend(Send p)
{
  switch(p->kind)
  {
  case is_SendSingle:

    bufAppendS("SendSingle");




    break;
  case is_SendMultiple:

    bufAppendS("SendMultiple");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Send!\n");
    exit(1);
  }
}

void shBranch(Branch p)
{
  switch(p->kind)
  {
  case is_BranchImpl:
    bufAppendC('(');

    bufAppendS("BranchImpl");

    bufAppendC(' ');

    shReceiptLinearImpl(p->u.branchimpl_.receiptlinearimpl_);
  bufAppendC(' ');
    shProc(p->u.branchimpl_.proc_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Branch!\n");
    exit(1);
  }
}

void shListBranch(ListBranch listbranch)
{
  bufAppendC('[');
  while(listbranch != 0)
  {
    if (listbranch->listbranch_)
    {
      shBranch(listbranch->branch_);
      bufAppendS(", ");
      listbranch = listbranch->listbranch_;
    }
    else
    {
      shBranch(listbranch->branch_);
      listbranch = 0;
    }
  }
  bufAppendC(']');
}

void shCase(Case p)
{
  switch(p->kind)
  {
  case is_CaseImpl:
    bufAppendC('(');

    bufAppendS("CaseImpl");

    bufAppendC(' ');

    shProc(p->u.caseimpl_.proc_1);
  bufAppendC(' ');
    shProc(p->u.caseimpl_.proc_2);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Case!\n");
    exit(1);
  }
}

void shListCase(ListCase listcase)
{
  bufAppendC('[');
  while(listcase != 0)
  {
    if (listcase->listcase_)
    {
      shCase(listcase->case_);
      bufAppendS(", ");
      listcase = listcase->listcase_;
    }
    else
    {
      shCase(listcase->case_);
      listcase = 0;
    }
  }
  bufAppendC(']');
}

void shNameDecl(NameDecl p)
{
  switch(p->kind)
  {
  case is_NameDeclSimpl:
    bufAppendC('(');

    bufAppendS("NameDeclSimpl");

    bufAppendC(' ');

    shIdent(p->u.namedeclsimpl_.var_);

    bufAppendC(')');

    break;
  case is_NameDeclUrn:
    bufAppendC('(');

    bufAppendS("NameDeclUrn");

    bufAppendC(' ');

    shIdent(p->u.namedeclurn_.var_);
  bufAppendC(' ');
    shIdent(p->u.namedeclurn_.uriliteral_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing NameDecl!\n");
    exit(1);
  }
}

void shListNameDecl(ListNameDecl listnamedecl)
{
  bufAppendC('[');
  while(listnamedecl != 0)
  {
    if (listnamedecl->listnamedecl_)
    {
      shNameDecl(listnamedecl->namedecl_);
      bufAppendS(", ");
      listnamedecl = listnamedecl->listnamedecl_;
    }
    else
    {
      shNameDecl(listnamedecl->namedecl_);
      listnamedecl = 0;
    }
  }
  bufAppendC(']');
}

void shBoolLiteral(BoolLiteral p)
{
  switch(p->kind)
  {
  case is_BoolTrue:

    bufAppendS("BoolTrue");




    break;
  case is_BoolFalse:

    bufAppendS("BoolFalse");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing BoolLiteral!\n");
    exit(1);
  }
}

void shGround(Ground p)
{
  switch(p->kind)
  {
  case is_GroundBool:
    bufAppendC('(');

    bufAppendS("GroundBool");

    bufAppendC(' ');

    shBoolLiteral(p->u.groundbool_.boolliteral_);

    bufAppendC(')');

    break;
  case is_GroundInt:
    bufAppendC('(');

    bufAppendS("GroundInt");

    bufAppendC(' ');

    shIdent(p->u.groundint_.longliteral_);

    bufAppendC(')');

    break;
  case is_GroundString:
    bufAppendC('(');

    bufAppendS("GroundString");

    bufAppendC(' ');

    shIdent(p->u.groundstring_.stringliteral_);

    bufAppendC(')');

    break;
  case is_GroundUri:
    bufAppendC('(');

    bufAppendS("GroundUri");

    bufAppendC(' ');

    shIdent(p->u.grounduri_.uriliteral_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Ground!\n");
    exit(1);
  }
}

void shCollection(Collection p)
{
  switch(p->kind)
  {
  case is_CollectList:
    bufAppendC('(');

    bufAppendS("CollectList");

    bufAppendC(' ');

    shListProc(p->u.collectlist_.listproc_);
  bufAppendC(' ');
    shProcRemainder(p->u.collectlist_.procremainder_);

    bufAppendC(')');

    break;
  case is_CollectTuple:
    bufAppendC('(');

    bufAppendS("CollectTuple");

    bufAppendC(' ');

    shTuple(p->u.collecttuple_.tuple_);

    bufAppendC(')');

    break;
  case is_CollectSet:
    bufAppendC('(');

    bufAppendS("CollectSet");

    bufAppendC(' ');

    shListProc(p->u.collectset_.listproc_);
  bufAppendC(' ');
    shProcRemainder(p->u.collectset_.procremainder_);

    bufAppendC(')');

    break;
  case is_CollectMap:
    bufAppendC('(');

    bufAppendS("CollectMap");

    bufAppendC(' ');

    shListKeyValuePair(p->u.collectmap_.listkeyvaluepair_);
  bufAppendC(' ');
    shProcRemainder(p->u.collectmap_.procremainder_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Collection!\n");
    exit(1);
  }
}

void shKeyValuePair(KeyValuePair p)
{
  switch(p->kind)
  {
  case is_KeyValuePairImpl:
    bufAppendC('(');

    bufAppendS("KeyValuePairImpl");

    bufAppendC(' ');

    shProc(p->u.keyvaluepairimpl_.proc_1);
  bufAppendC(' ');
    shProc(p->u.keyvaluepairimpl_.proc_2);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing KeyValuePair!\n");
    exit(1);
  }
}

void shListKeyValuePair(ListKeyValuePair listkeyvaluepair)
{
  bufAppendC('[');
  while(listkeyvaluepair != 0)
  {
    if (listkeyvaluepair->listkeyvaluepair_)
    {
      shKeyValuePair(listkeyvaluepair->keyvaluepair_);
      bufAppendS(", ");
      listkeyvaluepair = listkeyvaluepair->listkeyvaluepair_;
    }
    else
    {
      shKeyValuePair(listkeyvaluepair->keyvaluepair_);
      listkeyvaluepair = 0;
    }
  }
  bufAppendC(']');
}

void shTuple(Tuple p)
{
  switch(p->kind)
  {
  case is_TupleSingle:
    bufAppendC('(');

    bufAppendS("TupleSingle");

    bufAppendC(' ');

    shProc(p->u.tuplesingle_.proc_);

    bufAppendC(')');

    break;
  case is_TupleMultiple:
    bufAppendC('(');

    bufAppendS("TupleMultiple");

    bufAppendC(' ');

    shProc(p->u.tuplemultiple_.proc_);
  bufAppendC(' ');
    shListProc(p->u.tuplemultiple_.listproc_);

    bufAppendC(')');

    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing Tuple!\n");
    exit(1);
  }
}

void shProcRemainder(ProcRemainder p)
{
  switch(p->kind)
  {
  case is_ProcRemainderVar:
    bufAppendC('(');

    bufAppendS("ProcRemainderVar");

    bufAppendC(' ');

    shProcVar(p->u.procremaindervar_.procvar_);

    bufAppendC(')');

    break;
  case is_ProcRemainderEmpty:

    bufAppendS("ProcRemainderEmpty");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing ProcRemainder!\n");
    exit(1);
  }
}

void shNameRemainder(NameRemainder p)
{
  switch(p->kind)
  {
  case is_NameRemainderVar:
    bufAppendC('(');

    bufAppendS("NameRemainderVar");

    bufAppendC(' ');

    shProcVar(p->u.nameremaindervar_.procvar_);

    bufAppendC(')');

    break;
  case is_NameRemainderEmpty:

    bufAppendS("NameRemainderEmpty");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing NameRemainder!\n");
    exit(1);
  }
}

void shVarRefKind(VarRefKind p)
{
  switch(p->kind)
  {
  case is_VarRefKindProc:

    bufAppendS("VarRefKindProc");




    break;
  case is_VarRefKindName:

    bufAppendS("VarRefKindName");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing VarRefKind!\n");
    exit(1);
  }
}

void shSimpleType(SimpleType p)
{
  switch(p->kind)
  {
  case is_SimpleTypeBool:

    bufAppendS("SimpleTypeBool");




    break;
  case is_SimpleTypeInt:

    bufAppendS("SimpleTypeInt");




    break;
  case is_SimpleTypeString:

    bufAppendS("SimpleTypeString");




    break;
  case is_SimpleTypeUri:

    bufAppendS("SimpleTypeUri");




    break;
  case is_SimpleTypeByteArray:

    bufAppendS("SimpleTypeByteArray");




    break;

  default:
    fprintf(stderr, "Error: bad kind field when showing SimpleType!\n");
    exit(1);
  }
}

void shInteger(Integer i)
{
  char tmp[20];
  sprintf(tmp, "%d", i);
  bufAppendS(tmp);
}
void shDouble(Double d)
{
  char tmp[24];
  sprintf(tmp, "%.15g", d);
  bufAppendS(tmp);
}
void shChar(Char c)
{
  bufAppendC('\'');
  bufAppendC(c);
  bufAppendC('\'');
}
void shString(String s)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
}
void shIdent(String s)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
}

void shLongLiteral(String s)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
}


void shStringLiteral(String s)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
}


void shUriLiteral(String s)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
}


void shVar(String s)
{
  bufAppendC('\"');
  bufAppendS(s);
  bufAppendC('\"');
}


void bufAppendS(const char *s)
{
  int len = strlen(s);
  int n;
  while (cur_ + len >= buf_size)
  {
    buf_size *= 2; /* Double the buffer size */
    resizeBuffer();
  }
  for(n = 0; n < len; n++)
  {
    buf_[cur_ + n] = s[n];
  }
  cur_ += len;
  buf_[cur_] = 0;
}
void bufAppendC(const char c)
{
  if (cur_ + 1 >= buf_size)
  {
    buf_size *= 2; /* Double the buffer size */
    resizeBuffer();
  }
  buf_[cur_] = c;
  cur_++;
  buf_[cur_] = 0;
}
void bufReset(void)
{
  cur_ = 0;
  buf_size = BUFFER_INITIAL;
  resizeBuffer();
  memset(buf_, 0, buf_size);
}
void resizeBuffer(void)
{
  char *temp = (char *) malloc(buf_size);
  if (!temp)
  {
    fprintf(stderr, "Error: Out of memory while attempting to grow buffer!\n");
    exit(1);
  }
  if (buf_)
  {
    strncpy(temp, buf_, buf_size); /* peteg: strlcpy is safer, but not POSIX/ISO C. */
    free(buf_);
  }
  buf_ = temp;
}
char *buf_;
int cur_, buf_size;

