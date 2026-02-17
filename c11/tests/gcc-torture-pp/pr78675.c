# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78675.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78675.c" 2


long int a;

__attribute__((noinline, noclone)) long int
foo (long int x)
{
  long int b;
  while (a < 1)
    {
      b = a && x;
      ++a;
    }
  return b;
}

int
main ()
{
  if (foo (0) != 0)
    __builtin_abort ();
  a = 0;
  if (foo (1) != 0)
    __builtin_abort ();
  a = 0;
  if (foo (25) != 0)
    __builtin_abort ();
  a = -64;
  if (foo (0) != 0)
    __builtin_abort ();
  a = -64;
  if (foo (1) != 0)
    __builtin_abort ();
  a = -64;
  if (foo (25) != 0)
    __builtin_abort ();
  return 0;
}
