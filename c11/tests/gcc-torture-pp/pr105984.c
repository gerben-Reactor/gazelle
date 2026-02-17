# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105984.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105984.c" 2


unsigned long long g;

static inline unsigned long long
foo (unsigned char c)
{
  g -= __builtin_mul_overflow_p (4, (unsigned char) ~c, 0);
  return g;
}

int
main ()
{
  unsigned long long x = foo (1);
  if (x != 0)
    __builtin_abort ();
  return 0;
}
