# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97325.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97325.c" 2


unsigned long long
foo (unsigned long long c)
{
  return c ? __builtin_ffs (-(unsigned short) c) : 0;
}

int
main ()
{
  if (foo (2) != 2)
    __builtin_abort ();
  return 0;
}
