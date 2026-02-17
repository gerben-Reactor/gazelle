# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57124.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57124.c" 2



extern void abort (void);
extern void exit (int);

__attribute__ ((noinline)) void
foo(short unsigned int *p1, short unsigned int *p2)
{
  short unsigned int x1, x4;
  int x2, x3, x5, x6;
  unsigned int x7;

  x1 = *p1;
  x2 = (int) x1;
  x3 = x2 * 65536;
  x4 = *p2;
  x5 = (int) x4;
  x6 = x3 + x4;
  x7 = (unsigned int) x6;
  if (x7 <= 268435455U)
    abort ();
  exit (0);
}

int
main()
{
  short unsigned int x, y;
  x = -5;
  y = -10;
  foo (&x, &y);
}
