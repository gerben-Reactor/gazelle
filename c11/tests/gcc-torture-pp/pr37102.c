# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr37102.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr37102.c" 2
extern void abort (void);

unsigned int a, b = 1, c;

void __attribute__ ((noinline))
foo (int x)
{
  if (x != 5)
    abort ();
}

int
main ()
{
  unsigned int d, e;
  for (d = 1; d < 5; d++)
    if (c)
      a = b;
  a = b;
  e = a << 1;
  if (e)
    e = (e << 1) ^ 1;
  foo (e);
  return 0;
}
