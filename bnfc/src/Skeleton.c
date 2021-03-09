/*** BNFC-Generated Visitor Traversal Skeleton. ***/
/* This traverses the abstract syntax tree.
   To use, copy Skeleton.h and Skeleton.c to
   new files. */

#include <stdlib.h>
#include <stdio.h>

#include "Skeleton.h"

void visitProc(Proc p)
{
  switch(p->kind)
  {
  case is_PPar:
    /* Code for PPar Goes Here */
    visitProc(p->u.ppar_.proc_1);
    visitProc(p->u.ppar_.proc_2);
    break;
  case is_PIf:
    /* Code for PIf Goes Here */
    visitProc(p->u.pif_.proc_1);
    visitProc(p->u.pif_.proc_2);
    break;
  case is_PIfElse:
    /* Code for PIfElse Goes Here */
    visitProc(p->u.pifelse_.proc_1);
    visitProc(p->u.pifelse_.proc_2);
    visitProc(p->u.pifelse_.proc_3);
    break;
  case is_PNew:
    /* Code for PNew Goes Here */
    visitListNameDecl(p->u.pnew_.listnamedecl_);
    visitProc(p->u.pnew_.proc_);
    break;
  case is_PContr:
    /* Code for PContr Goes Here */
    visitName(p->u.pcontr_.name_);
    visitListName(p->u.pcontr_.listname_);
    visitNameRemainder(p->u.pcontr_.nameremainder_);
    visitProc(p->u.pcontr_.proc_);
    break;
  case is_PInput:
    /* Code for PInput Goes Here */
    visitReceipt(p->u.pinput_.receipt_);
    visitProc(p->u.pinput_.proc_);
    break;
  case is_PChoice:
    /* Code for PChoice Goes Here */
    visitListBranch(p->u.pchoice_.listbranch_);
    break;
  case is_PMatch:
    /* Code for PMatch Goes Here */
    visitProc(p->u.pmatch_.proc_);
    visitListCase(p->u.pmatch_.listcase_);
    break;
  case is_PBundle:
    /* Code for PBundle Goes Here */
    visitBundle(p->u.pbundle_.bundle_);
    visitProc(p->u.pbundle_.proc_);
    break;
  case is_PSend:
    /* Code for PSend Goes Here */
    visitName(p->u.psend_.name_);
    visitSend(p->u.psend_.send_);
    visitListProc(p->u.psend_.listproc_);
    break;
  case is_POr:
    /* Code for POr Goes Here */
    visitProc(p->u.por_.proc_1);
    visitProc(p->u.por_.proc_2);
    break;
  case is_PAnd:
    /* Code for PAnd Goes Here */
    visitProc(p->u.pand_.proc_1);
    visitProc(p->u.pand_.proc_2);
    break;
  case is_PMatches:
    /* Code for PMatches Goes Here */
    visitProc(p->u.pmatches_.proc_1);
    visitProc(p->u.pmatches_.proc_2);
    break;
  case is_PEq:
    /* Code for PEq Goes Here */
    visitProc(p->u.peq_.proc_1);
    visitProc(p->u.peq_.proc_2);
    break;
  case is_PNeq:
    /* Code for PNeq Goes Here */
    visitProc(p->u.pneq_.proc_1);
    visitProc(p->u.pneq_.proc_2);
    break;
  case is_PLt:
    /* Code for PLt Goes Here */
    visitProc(p->u.plt_.proc_1);
    visitProc(p->u.plt_.proc_2);
    break;
  case is_PLte:
    /* Code for PLte Goes Here */
    visitProc(p->u.plte_.proc_1);
    visitProc(p->u.plte_.proc_2);
    break;
  case is_PGt:
    /* Code for PGt Goes Here */
    visitProc(p->u.pgt_.proc_1);
    visitProc(p->u.pgt_.proc_2);
    break;
  case is_PGte:
    /* Code for PGte Goes Here */
    visitProc(p->u.pgte_.proc_1);
    visitProc(p->u.pgte_.proc_2);
    break;
  case is_PAdd:
    /* Code for PAdd Goes Here */
    visitProc(p->u.padd_.proc_1);
    visitProc(p->u.padd_.proc_2);
    break;
  case is_PMinus:
    /* Code for PMinus Goes Here */
    visitProc(p->u.pminus_.proc_1);
    visitProc(p->u.pminus_.proc_2);
    break;
  case is_PPlusPlus:
    /* Code for PPlusPlus Goes Here */
    visitProc(p->u.pplusplus_.proc_1);
    visitProc(p->u.pplusplus_.proc_2);
    break;
  case is_PMinusMinus:
    /* Code for PMinusMinus Goes Here */
    visitProc(p->u.pminusminus_.proc_1);
    visitProc(p->u.pminusminus_.proc_2);
    break;
  case is_PMult:
    /* Code for PMult Goes Here */
    visitProc(p->u.pmult_.proc_1);
    visitProc(p->u.pmult_.proc_2);
    break;
  case is_PDiv:
    /* Code for PDiv Goes Here */
    visitProc(p->u.pdiv_.proc_1);
    visitProc(p->u.pdiv_.proc_2);
    break;
  case is_PMod:
    /* Code for PMod Goes Here */
    visitProc(p->u.pmod_.proc_1);
    visitProc(p->u.pmod_.proc_2);
    break;
  case is_PPercentPercent:
    /* Code for PPercentPercent Goes Here */
    visitProc(p->u.ppercentpercent_.proc_1);
    visitProc(p->u.ppercentpercent_.proc_2);
    break;
  case is_PNot:
    /* Code for PNot Goes Here */
    visitProc(p->u.pnot_.proc_);
    break;
  case is_PNeg:
    /* Code for PNeg Goes Here */
    visitProc(p->u.pneg_.proc_);
    break;
  case is_PMethod:
    /* Code for PMethod Goes Here */
    visitProc(p->u.pmethod_.proc_);
    visitVar(p->u.pmethod_.var_);
    visitListProc(p->u.pmethod_.listproc_);
    break;
  case is_PExprs:
    /* Code for PExprs Goes Here */
    visitProc(p->u.pexprs_.proc_);
    break;
  case is_PEval:
    /* Code for PEval Goes Here */
    visitName(p->u.peval_.name_);
    break;
  case is_PVarRef:
    /* Code for PVarRef Goes Here */
    visitVarRefKind(p->u.pvarref_.varrefkind_);
    visitVar(p->u.pvarref_.var_);
    break;
  case is_PDisjunction:
    /* Code for PDisjunction Goes Here */
    visitProc(p->u.pdisjunction_.proc_1);
    visitProc(p->u.pdisjunction_.proc_2);
    break;
  case is_PConjunction:
    /* Code for PConjunction Goes Here */
    visitProc(p->u.pconjunction_.proc_1);
    visitProc(p->u.pconjunction_.proc_2);
    break;
  case is_PNegation:
    /* Code for PNegation Goes Here */
    visitProc(p->u.pnegation_.proc_);
    break;
  case is_PGround:
    /* Code for PGround Goes Here */
    visitGround(p->u.pground_.ground_);
    break;
  case is_PCollect:
    /* Code for PCollect Goes Here */
    visitCollection(p->u.pcollect_.collection_);
    break;
  case is_PVar:
    /* Code for PVar Goes Here */
    visitProcVar(p->u.pvar_.procvar_);
    break;
  case is_PNil:
    /* Code for PNil Goes Here */
    break;
  case is_PSimpleType:
    /* Code for PSimpleType Goes Here */
    visitSimpleType(p->u.psimpletype_.simpletype_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Proc!\n");
    exit(1);
  }
}

void visitListProc(ListProc listproc)
{
  while(listproc  != 0)
  {
    /* Code For ListProc Goes Here */
    visitProc(listproc->proc_);
    listproc = listproc->listproc_;
  }
}

void visitProcVar(ProcVar p)
{
  switch(p->kind)
  {
  case is_ProcVarWildcard:
    /* Code for ProcVarWildcard Goes Here */
    break;
  case is_ProcVarVar:
    /* Code for ProcVarVar Goes Here */
    visitVar(p->u.procvarvar_.var_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ProcVar!\n");
    exit(1);
  }
}

void visitName(Name p)
{
  switch(p->kind)
  {
  case is_NameWildcard:
    /* Code for NameWildcard Goes Here */
    break;
  case is_NameVar:
    /* Code for NameVar Goes Here */
    visitVar(p->u.namevar_.var_);
    break;
  case is_NameQuote:
    /* Code for NameQuote Goes Here */
    visitProc(p->u.namequote_.proc_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Name!\n");
    exit(1);
  }
}

void visitListName(ListName listname)
{
  while(listname  != 0)
  {
    /* Code For ListName Goes Here */
    visitName(listname->name_);
    listname = listname->listname_;
  }
}

void visitBundle(Bundle p)
{
  switch(p->kind)
  {
  case is_BundleWrite:
    /* Code for BundleWrite Goes Here */
    break;
  case is_BundleRead:
    /* Code for BundleRead Goes Here */
    break;
  case is_BundleEquiv:
    /* Code for BundleEquiv Goes Here */
    break;
  case is_BundleReadWrite:
    /* Code for BundleReadWrite Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Bundle!\n");
    exit(1);
  }
}

void visitReceipt(Receipt p)
{
  switch(p->kind)
  {
  case is_ReceiptLinear:
    /* Code for ReceiptLinear Goes Here */
    visitReceiptLinearImpl(p->u.receiptlinear_.receiptlinearimpl_);
    break;
  case is_ReceiptRepeated:
    /* Code for ReceiptRepeated Goes Here */
    visitReceiptRepeatedImpl(p->u.receiptrepeated_.receiptrepeatedimpl_);
    break;
  case is_ReceiptPeek:
    /* Code for ReceiptPeek Goes Here */
    visitReceiptPeekImpl(p->u.receiptpeek_.receiptpeekimpl_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Receipt!\n");
    exit(1);
  }
}

void visitReceiptLinearImpl(ReceiptLinearImpl p)
{
  switch(p->kind)
  {
  case is_LinearSimple:
    /* Code for LinearSimple Goes Here */
    visitListLinearBind(p->u.linearsimple_.listlinearbind_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ReceiptLinearImpl!\n");
    exit(1);
  }
}

void visitLinearBind(LinearBind p)
{
  switch(p->kind)
  {
  case is_LinearBindImpl:
    /* Code for LinearBindImpl Goes Here */
    visitListName(p->u.linearbindimpl_.listname_);
    visitNameRemainder(p->u.linearbindimpl_.nameremainder_);
    visitName(p->u.linearbindimpl_.name_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing LinearBind!\n");
    exit(1);
  }
}

void visitListLinearBind(ListLinearBind listlinearbind)
{
  while(listlinearbind  != 0)
  {
    /* Code For ListLinearBind Goes Here */
    visitLinearBind(listlinearbind->linearbind_);
    listlinearbind = listlinearbind->listlinearbind_;
  }
}

void visitReceiptRepeatedImpl(ReceiptRepeatedImpl p)
{
  switch(p->kind)
  {
  case is_RepeatedSimple:
    /* Code for RepeatedSimple Goes Here */
    visitListRepeatedBind(p->u.repeatedsimple_.listrepeatedbind_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ReceiptRepeatedImpl!\n");
    exit(1);
  }
}

void visitRepeatedBind(RepeatedBind p)
{
  switch(p->kind)
  {
  case is_RepeatedBindImpl:
    /* Code for RepeatedBindImpl Goes Here */
    visitListName(p->u.repeatedbindimpl_.listname_);
    visitNameRemainder(p->u.repeatedbindimpl_.nameremainder_);
    visitName(p->u.repeatedbindimpl_.name_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing RepeatedBind!\n");
    exit(1);
  }
}

void visitListRepeatedBind(ListRepeatedBind listrepeatedbind)
{
  while(listrepeatedbind  != 0)
  {
    /* Code For ListRepeatedBind Goes Here */
    visitRepeatedBind(listrepeatedbind->repeatedbind_);
    listrepeatedbind = listrepeatedbind->listrepeatedbind_;
  }
}

void visitReceiptPeekImpl(ReceiptPeekImpl p)
{
  switch(p->kind)
  {
  case is_PeekSimple:
    /* Code for PeekSimple Goes Here */
    visitListPeekBind(p->u.peeksimple_.listpeekbind_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ReceiptPeekImpl!\n");
    exit(1);
  }
}

void visitPeekBind(PeekBind p)
{
  switch(p->kind)
  {
  case is_PeekBindImpl:
    /* Code for PeekBindImpl Goes Here */
    visitListName(p->u.peekbindimpl_.listname_);
    visitNameRemainder(p->u.peekbindimpl_.nameremainder_);
    visitName(p->u.peekbindimpl_.name_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing PeekBind!\n");
    exit(1);
  }
}

void visitListPeekBind(ListPeekBind listpeekbind)
{
  while(listpeekbind  != 0)
  {
    /* Code For ListPeekBind Goes Here */
    visitPeekBind(listpeekbind->peekbind_);
    listpeekbind = listpeekbind->listpeekbind_;
  }
}

void visitSend(Send p)
{
  switch(p->kind)
  {
  case is_SendSingle:
    /* Code for SendSingle Goes Here */
    break;
  case is_SendMultiple:
    /* Code for SendMultiple Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Send!\n");
    exit(1);
  }
}

void visitBranch(Branch p)
{
  switch(p->kind)
  {
  case is_BranchImpl:
    /* Code for BranchImpl Goes Here */
    visitReceiptLinearImpl(p->u.branchimpl_.receiptlinearimpl_);
    visitProc(p->u.branchimpl_.proc_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Branch!\n");
    exit(1);
  }
}

void visitListBranch(ListBranch listbranch)
{
  while(listbranch  != 0)
  {
    /* Code For ListBranch Goes Here */
    visitBranch(listbranch->branch_);
    listbranch = listbranch->listbranch_;
  }
}

void visitCase(Case p)
{
  switch(p->kind)
  {
  case is_CaseImpl:
    /* Code for CaseImpl Goes Here */
    visitProc(p->u.caseimpl_.proc_1);
    visitProc(p->u.caseimpl_.proc_2);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Case!\n");
    exit(1);
  }
}

void visitListCase(ListCase listcase)
{
  while(listcase  != 0)
  {
    /* Code For ListCase Goes Here */
    visitCase(listcase->case_);
    listcase = listcase->listcase_;
  }
}

void visitNameDecl(NameDecl p)
{
  switch(p->kind)
  {
  case is_NameDeclSimpl:
    /* Code for NameDeclSimpl Goes Here */
    visitVar(p->u.namedeclsimpl_.var_);
    break;
  case is_NameDeclUrn:
    /* Code for NameDeclUrn Goes Here */
    visitVar(p->u.namedeclurn_.var_);
    visitUriLiteral(p->u.namedeclurn_.uriliteral_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing NameDecl!\n");
    exit(1);
  }
}

void visitListNameDecl(ListNameDecl listnamedecl)
{
  while(listnamedecl  != 0)
  {
    /* Code For ListNameDecl Goes Here */
    visitNameDecl(listnamedecl->namedecl_);
    listnamedecl = listnamedecl->listnamedecl_;
  }
}

void visitBoolLiteral(BoolLiteral p)
{
  switch(p->kind)
  {
  case is_BoolTrue:
    /* Code for BoolTrue Goes Here */
    break;
  case is_BoolFalse:
    /* Code for BoolFalse Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing BoolLiteral!\n");
    exit(1);
  }
}

void visitGround(Ground p)
{
  switch(p->kind)
  {
  case is_GroundBool:
    /* Code for GroundBool Goes Here */
    visitBoolLiteral(p->u.groundbool_.boolliteral_);
    break;
  case is_GroundInt:
    /* Code for GroundInt Goes Here */
    visitLongLiteral(p->u.groundint_.longliteral_);
    break;
  case is_GroundString:
    /* Code for GroundString Goes Here */
    visitStringLiteral(p->u.groundstring_.stringliteral_);
    break;
  case is_GroundUri:
    /* Code for GroundUri Goes Here */
    visitUriLiteral(p->u.grounduri_.uriliteral_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Ground!\n");
    exit(1);
  }
}

void visitCollection(Collection p)
{
  switch(p->kind)
  {
  case is_CollectList:
    /* Code for CollectList Goes Here */
    visitListProc(p->u.collectlist_.listproc_);
    visitProcRemainder(p->u.collectlist_.procremainder_);
    break;
  case is_CollectTuple:
    /* Code for CollectTuple Goes Here */
    visitTuple(p->u.collecttuple_.tuple_);
    break;
  case is_CollectSet:
    /* Code for CollectSet Goes Here */
    visitListProc(p->u.collectset_.listproc_);
    visitProcRemainder(p->u.collectset_.procremainder_);
    break;
  case is_CollectMap:
    /* Code for CollectMap Goes Here */
    visitListKeyValuePair(p->u.collectmap_.listkeyvaluepair_);
    visitProcRemainder(p->u.collectmap_.procremainder_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Collection!\n");
    exit(1);
  }
}

void visitKeyValuePair(KeyValuePair p)
{
  switch(p->kind)
  {
  case is_KeyValuePairImpl:
    /* Code for KeyValuePairImpl Goes Here */
    visitProc(p->u.keyvaluepairimpl_.proc_1);
    visitProc(p->u.keyvaluepairimpl_.proc_2);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing KeyValuePair!\n");
    exit(1);
  }
}

void visitListKeyValuePair(ListKeyValuePair listkeyvaluepair)
{
  while(listkeyvaluepair  != 0)
  {
    /* Code For ListKeyValuePair Goes Here */
    visitKeyValuePair(listkeyvaluepair->keyvaluepair_);
    listkeyvaluepair = listkeyvaluepair->listkeyvaluepair_;
  }
}

void visitTuple(Tuple p)
{
  switch(p->kind)
  {
  case is_TupleSingle:
    /* Code for TupleSingle Goes Here */
    visitProc(p->u.tuplesingle_.proc_);
    break;
  case is_TupleMultiple:
    /* Code for TupleMultiple Goes Here */
    visitProc(p->u.tuplemultiple_.proc_);
    visitListProc(p->u.tuplemultiple_.listproc_);
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing Tuple!\n");
    exit(1);
  }
}

void visitProcRemainder(ProcRemainder p)
{
  switch(p->kind)
  {
  case is_ProcRemainderVar:
    /* Code for ProcRemainderVar Goes Here */
    visitProcVar(p->u.procremaindervar_.procvar_);
    break;
  case is_ProcRemainderEmpty:
    /* Code for ProcRemainderEmpty Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing ProcRemainder!\n");
    exit(1);
  }
}

void visitNameRemainder(NameRemainder p)
{
  switch(p->kind)
  {
  case is_NameRemainderVar:
    /* Code for NameRemainderVar Goes Here */
    visitProcVar(p->u.nameremaindervar_.procvar_);
    break;
  case is_NameRemainderEmpty:
    /* Code for NameRemainderEmpty Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing NameRemainder!\n");
    exit(1);
  }
}

void visitVarRefKind(VarRefKind p)
{
  switch(p->kind)
  {
  case is_VarRefKindProc:
    /* Code for VarRefKindProc Goes Here */
    break;
  case is_VarRefKindName:
    /* Code for VarRefKindName Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing VarRefKind!\n");
    exit(1);
  }
}

void visitSimpleType(SimpleType p)
{
  switch(p->kind)
  {
  case is_SimpleTypeBool:
    /* Code for SimpleTypeBool Goes Here */
    break;
  case is_SimpleTypeInt:
    /* Code for SimpleTypeInt Goes Here */
    break;
  case is_SimpleTypeString:
    /* Code for SimpleTypeString Goes Here */
    break;
  case is_SimpleTypeUri:
    /* Code for SimpleTypeUri Goes Here */
    break;
  case is_SimpleTypeByteArray:
    /* Code for SimpleTypeByteArray Goes Here */
    break;

  default:
    fprintf(stderr, "Error: bad kind field when printing SimpleType!\n");
    exit(1);
  }
}

void visitLongLiteral(LongLiteral p)
{
  /* Code for LongLiteral Goes Here */
}
void visitStringLiteral(StringLiteral p)
{
  /* Code for StringLiteral Goes Here */
}
void visitUriLiteral(UriLiteral p)
{
  /* Code for UriLiteral Goes Here */
}
void visitVar(Var p)
{
  /* Code for Var Goes Here */
}
void visitIdent(Ident i)
{
  /* Code for Ident Goes Here */
}
void visitInteger(Integer i)
{
  /* Code for Integer Goes Here */
}
void visitDouble(Double d)
{
  /* Code for Double Goes Here */
}
void visitChar(Char c)
{
  /* Code for Char Goes Here */
}
void visitString(String s)
{
  /* Code for String Goes Here */
}

