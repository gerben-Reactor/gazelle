# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98474.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98474.c" 2



typedef __uint128_t T;






__attribute__ ((noipa)) void
foo (T *x)
{
  *x += ((T) 1) << ((16 * 8 / 2) + 1);
}

int
main ()
{
  T a = ((T) 1) << ((16 * 8 / 2) + 1);
  T b = a;
  T n;
  foo (&b);
  n = b;
  while (n >= a)
    n -= a;
  if ((int) (n >> (16 * 8 / 2)) != 0)
    __builtin_abort ();
  return 0;
}
