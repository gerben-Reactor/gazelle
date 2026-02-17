# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58943.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58943.c" 2


unsigned int x[1] = { 2 };

unsigned int
foo (void)
{
  x[0] |= 128;
  return 1;
}

int
main ()
{
  x[0] |= foo ();
  if (x[0] != 131)
    __builtin_abort ();
  return 0;
}
