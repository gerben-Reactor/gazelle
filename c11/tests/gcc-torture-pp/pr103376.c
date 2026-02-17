# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103376.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103376.c" 2


long long a = 0x123456789abcdef0LL, f;
int b, c, *d;

__attribute__((noipa)) void
foo (int x)
{
  asm volatile ("" : : "r" (x));
}

int
main ()
{
  long long e;
  e = a;
  if (b)
    {
      foo (c);
      d = (int *) 0;
      while (*d)
 ;
    }
  f = a ^ e;
  asm volatile ("" : "+m" (f));
  if (f != 0)
    __builtin_abort ();
  return 0;
}
