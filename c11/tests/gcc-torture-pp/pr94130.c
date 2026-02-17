# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94130.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94130.c" 2


int
main ()
{
  int a[8];
  char *b = __builtin_memset (a, 0, sizeof (a));
  a[0] = 1;
  a[1] = 2;
  a[2] = 3;
  if (b != (char *) a)
    __builtin_abort ();
  else
    asm volatile ("" : : "g" (a) : "memory");
  return 0;
}
