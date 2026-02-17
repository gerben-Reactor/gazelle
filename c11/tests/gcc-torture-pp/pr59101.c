# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr59101.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr59101.c" 2


__attribute__((noinline, noclone)) int
foo (int a)
{
  return (~a & 4102790424LL) > 0 | 6;
}

int
main ()
{
  if (foo (0) != 7)
    __builtin_abort ();
  return 0;
}
