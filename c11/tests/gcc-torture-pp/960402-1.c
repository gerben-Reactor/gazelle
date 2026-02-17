# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960402-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960402-1.c" 2


f (signed long long int x)
{
  return x > 0xFFFFFFFFLL || x < -0x80000000LL;
}

main ()
{
  if (f (0) != 0)
    abort ();
  exit (0);
}
