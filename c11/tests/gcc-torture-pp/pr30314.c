# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr30314.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr30314.c" 2


# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/../../gcc.dg/tree-ssa/pr30314.c" 1
# 10 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/../../gcc.dg/tree-ssa/pr30314.c"
__attribute__((noipa)) int
foo (unsigned int x)
{
  return __builtin_mul_overflow_p (x, 35U, 0U);
}

__attribute__((noipa)) int
bar (unsigned long int x)
{
  return __builtin_mul_overflow_p (x, 35UL, 0UL);
}

__attribute__((noipa)) int
baz (unsigned int x)
{
  return __builtin_mul_overflow_p (42, x, 0U);
}

__attribute__((noipa)) int
qux (unsigned long int x)
{
  return __builtin_mul_overflow_p (42, x, 0UL);
}
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr30314.c" 2

int
main ()
{
  if (foo (0) != 0
      || foo (~0U / 35) != 0
      || foo (~0U / 35 + 1) != 1
      || foo (~0U) != 1)
    __builtin_abort ();
  if (bar (0) != 0
      || bar (~0UL / 35) != 0
      || bar (~0UL / 35 + 1) != 1
      || bar (~0UL) != 1)
    __builtin_abort ();
  if (baz (0) != 0
      || baz (~0U / 42) != 0
      || baz (~0U / 42 + 1) != 1
      || baz (~0U) != 1)
    __builtin_abort ();
  if (qux (0) != 0
      || qux (~0UL / 42) != 0
      || qux (~0UL / 42 + 1) != 1
      || qux (~0UL) != 1)
    __builtin_abort ();
  return 0;
}
