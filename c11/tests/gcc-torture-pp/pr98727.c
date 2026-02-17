# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98727.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98727.c" 2


__attribute__((noipa)) long int
foo (long int x, long int y)
{
  long int z = (unsigned long) x * y;
  if (x != z / y)
    return -1;
  return z;
}

int
main ()
{
  if (foo (4, 24) != 96
      || foo (124, 126) != 124L * 126
      || foo (9223372036854775807L / 16, 17) != -1)
    __builtin_abort ();
  return 0;
}
