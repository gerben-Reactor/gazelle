# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20170419-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20170419-1.c" 2






int x;

int main (void)
{
  volatile int a = 0;
  volatile int b = -0x7fffffff;
  int j;

  for(j = 0; j < 18; j += 1) {
    x = ( (a == 0) != (b - (int)((-0x7fffffff -1)) ) );
  }

  if (x != 0)
    __builtin_abort ();

  return 0;
}
