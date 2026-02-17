# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr118623.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr118623.c" 2


static int
foo (int x, int y)
{
  int a = 1 << x;
  if (y & a)
    return 0;
  return 5;
}

__attribute__((noipa)) void
bar (int x)
{
  if (((foo (x - 50, x) + x + x) & 1) == 0)
    __builtin_abort ();
}

int
main ()
{
  bar (63);
}
