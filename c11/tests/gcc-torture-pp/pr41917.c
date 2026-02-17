# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr41917.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr41917.c" 2


extern void abort (void);
unsigned int a = 1;

int
main (void)
{
  unsigned int b, c, d;

  if (sizeof (int) != 4 || (int) 0xc7d24b5e > 0)
    return 0;

  c = 0xc7d24b5e;
  d = a | -2;
  b = (d == 0) ? c : (c % d);
  if (b != c)
    abort ();

  return 0;
}
