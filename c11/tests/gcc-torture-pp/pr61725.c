# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr61725.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr61725.c" 2


int
main ()
{
  int x;
  for (x = -128; x <= 128; x++)
    {
      int a = __builtin_ffs (x);
      if (x == 0 && a != 0)
        __builtin_abort ();
    }
  return 0;
}
