# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr83269.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr83269.c" 2


int
main ()
{

  volatile unsigned char a = 1;
  long long b = 0x80000000L;
  int c = -((int)(-b) - (-0x7fffffff * a));
  if (c != 1)
    __builtin_abort ();

  return 0;
}
