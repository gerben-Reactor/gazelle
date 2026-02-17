# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr123864.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr123864.c" 2


[[gnu::noipa]] static int
foo (long long x)
{
  return __builtin_mul_overflow_p (x, ~0U, x);
}

int
main ()
{
  if (foo (0))
    __builtin_abort ();

  if (foo (2147483647 + 1LL))
    __builtin_abort ();
  if (!foo (2147483647 + 2LL))
    __builtin_abort ();
  if (foo (-2147483647 - 1LL))
    __builtin_abort ();
  if (!foo (-2147483647 - 2LL))
    __builtin_abort ();

}
