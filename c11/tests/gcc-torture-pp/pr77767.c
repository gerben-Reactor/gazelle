# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr77767.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr77767.c" 2


void
foo (int a, int b[a++], int c, int d[c++])
{
  if (a != 2 || c != 2)
    __builtin_abort ();
}

int
main ()
{
  int e[10];
  foo (1, e, 1, e);
  return 0;
}
