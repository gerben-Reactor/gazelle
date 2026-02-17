# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr106032.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr106032.c" 2


__attribute__((noipa)) int
foo (int x, int *y)
{
  int a = 0;
  if (x < 0)
    a = *y;
  return a;
}

int
main ()
{
  int a = 42;
  if (foo (0, 0) != 0 || foo (1, 0) != 0)
    __builtin_abort ();
  if (foo (-1, &a) != 42 || foo (-42, &a) != 42)
    __builtin_abort ();
  return 0;
}
