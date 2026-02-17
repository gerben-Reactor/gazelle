# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr70586.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr70586.c" 2


int a, e, f;
short b, c, d;

int
foo (int x, int y)
{
  return (y == 0 || (x && y == 1)) ? x : x % y;
}

static short
bar (void)
{
  int i = foo (c, f);
  f = foo (d, 2);
  int g = foo (b, c);
  int h = foo (g > 0, c);
  c = (3 >= h ^ 7) <= foo (i, c);
  if (foo (e, 1))
    return a;
  return 0;
}

int
main ()
{
  bar ();
  return 0;
}
