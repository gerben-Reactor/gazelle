# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58662.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58662.c" 2
extern void abort (void);

int a, c, d;
volatile int b;

static int
foo (int p1, short p2)
{
  return p1 / p2;
}

int
main ()
{
  char e;
  d = foo (a == 0, (0, 35536));
  e = d % 14;
  b = e && c;
  if (b != 0)
    abort ();
  return 0;
}
