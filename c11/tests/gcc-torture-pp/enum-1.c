# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/enum-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/enum-1.c" 2
void abort (void);
void exit (int);

typedef enum
{
  END = -1,
  EMPTY = (1 << 8 ) ,
  BACKREF,
  BEGLINE,
  ENDLINE,
  BEGWORD,
  ENDWORD,
  LIMWORD,
  NOTLIMWORD,
  QMARK,
  STAR,
  PLUS,
  REPMN,
  CAT,
  OR,
  ORTOP,
  LPAREN,
  RPAREN,
  CSET
} token;

static token tok;

static int
atom ()
{
  if ((tok >= 0 && tok < (1 << 8 ) ) || tok >= CSET || tok == BACKREF
      || tok == BEGLINE || tok == ENDLINE || tok == BEGWORD
      || tok == ENDWORD || tok == LIMWORD || tok == NOTLIMWORD)
    return 1;
  else
    return 0;
}

int
main (void)
{
  tok = 0;
  if (atom () != 1)
    abort ();
  exit (0);
}
