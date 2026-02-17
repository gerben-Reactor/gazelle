# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr118915.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr118915.c" 2


int a;

int
foo (int c, int d, int e, int f)
{
  if (!d || !e)
    return -22;
  if (c > 16)
    return -22;
  if (!f)
    return -22;
  return 2;
}

int
main ()
{
  if (foo (a + 21, a + 6, a + 34, a + 26) != -22)
    __builtin_abort ();
}
