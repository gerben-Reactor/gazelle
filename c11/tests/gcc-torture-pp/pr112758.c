# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr112758.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr112758.c" 2


int a = -2147483647 - 1;

int
main ()
{
  if (-2147483647 - 1U == 0x80000000ULL)
    {
      unsigned long long b = 0xffff00ffffffffffULL;
      if ((b & a) != 0xffff00ff80000000ULL)
 __builtin_abort ();
    }
  return 0;
}
