# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941110-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941110-1.c" 2


f (const int x)
{
  int y = 0;
  y = x ? y : -y;
  {
    const int *p = &x;
  }
  return y;
}

main ()
{
  if (f (0))
    abort ();
  exit (0);
}
