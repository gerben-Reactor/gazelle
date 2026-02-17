# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr104196.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr104196.c" 2


int a = 6;

int
main ()
{
  while (1)
    {
      int b = a < 0 && 0 < -2147483647 - a ? 0 : a;
      if (b != 4096 - 2147483647)
 {
   if (a < 6)
     __builtin_abort ();
   break;
 }
    }
  return 0;
}
