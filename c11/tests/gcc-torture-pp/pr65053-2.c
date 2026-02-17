# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr65053-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr65053-2.c" 2


int i;
unsigned int x;

int
main ()
{
  asm volatile ("" : "+g" (x));
  unsigned int n = x;
  unsigned int u = 32;
  if (n >= 32)
    __builtin_abort ();
  if (n != 0)
    u = n + 32;

  while (u != 32)
    {
      asm ("" : : "g" (u));
      u = 32;
      i = 1;
    }

  if (i)
    __builtin_abort ();
  return 0;
}
