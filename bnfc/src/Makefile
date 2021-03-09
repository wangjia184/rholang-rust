CC = gcc -g
CCFLAGS = --ansi -W -Wall -Wno-unused-parameter -Wno-unused-function -Wno-unneeded-internal-declaration ${CC_OPTS}

FLEX = flex
FLEX_OPTS = -Prholang_mercury_

BISON = bison
BISON_OPTS = -t -prholang_mercury_

OBJS = Absyn.o Buffer.o Lexer.o Parser.o Printer.o

.PHONY : clean distclean

all : Testrholang_mercury

clean :
	rm -f *.o Testrholang_mercury rholang_mercury.aux rholang_mercury.log rholang_mercury.pdf rholang_mercury.dvi rholang_mercury.ps rholang_mercury

distclean : clean
	rm -f Absyn.h Absyn.c Buffer.h Buffer.c rholang_mercury.l Lexer.c rholang_mercury.y Parser.h Parser.c Printer.c Printer.h Skeleton.c Skeleton.h Test.c Makefile rholang_mercury.tex

Testrholang_mercury : ${OBJS} Test.o
	@echo "Linking Testrholang_mercury..."
	${CC} ${OBJS} Test.o -o Testrholang_mercury

Absyn.o : Absyn.c Absyn.h
	${CC} ${CCFLAGS} -c Absyn.c

Buffer.o : Buffer.c Buffer.h
	${CC} ${CCFLAGS} -c Buffer.c

Lexer.c : rholang_mercury.l
	${FLEX} ${FLEX_OPTS} -oLexer.c rholang_mercury.l

Parser.c : rholang_mercury.y
	${BISON} ${BISON_OPTS} rholang_mercury.y -o Parser.c

Lexer.o : Lexer.c Parser.h
	${CC} ${CCFLAGS} -c Lexer.c

Parser.o : Parser.c Absyn.h
	${CC} ${CCFLAGS} -c Parser.c

Printer.o : Printer.c Printer.h Absyn.h
	${CC} ${CCFLAGS} -c Printer.c

Test.o : Test.c Parser.h Printer.h Absyn.h
	${CC} ${CCFLAGS} -c Test.c
