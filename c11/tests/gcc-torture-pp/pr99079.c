# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr99079.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr99079.c" 2


__attribute__((noipa)) unsigned long long
foo (int x)
{
  unsigned long long s = 1 << x;
  return 4897637220ULL % s;
}

int
main ()
{
  if (4 * 8 != 32)
    return 0;
  if (foo (31) != 4897637220ULL)
    __builtin_abort ();
  return 0;
}
