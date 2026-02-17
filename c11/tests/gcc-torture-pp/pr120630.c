# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr120630.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr120630.c" 2


__attribute__((noipa)) int
foo (const char *x, ...)
{
  return *x;
}

int a, b, c;
unsigned d = 1;

int
main ()
{
  if (a)
    foo ("0");
  int e = -1;
  if (a < 1)
    {
      e = c;
      if (c)
 while (1)
   ;
    }
  b = (~e + 0UL) / -1;
  if (d > b)
    __builtin_abort ();
  return 0;
}
