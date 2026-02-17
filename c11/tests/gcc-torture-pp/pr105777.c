# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105777.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105777.c" 2


# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/../../gcc.dg/tree-ssa/pr105777.c" 1
# 22 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/../../gcc.dg/tree-ssa/pr105777.c"
__attribute__((noipa)) int
foo (int x)
{
  return __builtin_mul_overflow_p (x, 35, 0);
}

__attribute__((noipa)) int
bar (long int x)
{
  return __builtin_mul_overflow_p (x, 35L, 0L);
}

__attribute__((noipa)) int
baz (int x)
{
  return __builtin_mul_overflow_p (42, x, 0);
}

__attribute__((noipa)) int
qux (long int x)
{
  return __builtin_mul_overflow_p (42, x, 0L);
}

__attribute__((noipa)) int
corge (int x)
{
  return __builtin_mul_overflow_p (x, -39, 0);
}

__attribute__((noipa)) int
garply (long int x)
{
  return __builtin_mul_overflow_p (x, -39L, 0L);
}

__attribute__((noipa)) int
grault (int x)
{
  return __builtin_mul_overflow_p (-46, x, 0);
}

__attribute__((noipa)) int
waldo (long int x)
{
  return __builtin_mul_overflow_p (-46, x, 0L);
}
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr105777.c" 2

int
main ()
{
  if (foo (0) != 0
      || foo (2147483647 / 35) != 0
      || foo (2147483647 / 35 + 1) != 1
      || foo (2147483647) != 1
      || foo ((-2147483647 - 1) / 35) != 0
      || foo ((-2147483647 - 1) / 35 - 1) != 1
      || foo (-2147483647 - 1) != 1)
    __builtin_abort ();
  if (bar (0) != 0
      || bar (9223372036854775807L / 35) != 0
      || bar (9223372036854775807L / 35 + 1) != 1
      || bar (9223372036854775807L) != 1
      || bar ((-9223372036854775807L - 1) / 35) != 0
      || bar ((-9223372036854775807L - 1) / 35 - 1) != 1
      || bar (-9223372036854775807L - 1) != 1)
    __builtin_abort ();
  if (baz (0) != 0
      || baz (2147483647 / 42) != 0
      || baz (2147483647 / 42 + 1) != 1
      || baz (2147483647) != 1
      || baz ((-2147483647 - 1) / 42) != 0
      || baz ((-2147483647 - 1) / 42 - 1) != 1
      || baz (-2147483647 - 1) != 1)
    __builtin_abort ();
  if (qux (0) != 0
      || qux (9223372036854775807L / 42) != 0
      || qux (9223372036854775807L / 42 + 1) != 1
      || qux (9223372036854775807L) != 1
      || qux ((-9223372036854775807L - 1) / 42) != 0
      || qux ((-9223372036854775807L - 1) / 42 - 1) != 1
      || qux (-9223372036854775807L - 1) != 1)
    __builtin_abort ();
  if (corge (0) != 0
      || corge (2147483647 / -39) != 0
      || corge (2147483647 / -39 - 1) != 1
      || corge (2147483647) != 1
      || corge ((-2147483647 - 1) / -39) != 0
      || corge ((-2147483647 - 1) / -39 + 1) != 1
      || corge (-2147483647 - 1) != 1)
    __builtin_abort ();
  if (garply (0) != 0
      || garply (9223372036854775807L / -39) != 0
      || garply (9223372036854775807L / -39 - 1) != 1
      || garply (9223372036854775807L) != 1
      || garply ((-9223372036854775807L - 1) / -39) != 0
      || garply ((-9223372036854775807L - 1) / -39 + 1) != 1
      || garply (-9223372036854775807L - 1) != 1)
    __builtin_abort ();
  if (grault (0) != 0
      || grault (2147483647 / -46) != 0
      || grault (2147483647 / -46 - 1) != 1
      || grault (2147483647) != 1
      || grault ((-2147483647 - 1) / -46) != 0
      || grault ((-2147483647 - 1) / -46 + 1) != 1
      || grault (-2147483647 - 1) != 1)
    __builtin_abort ();
  if (waldo (0) != 0
      || waldo (9223372036854775807L / -46) != 0
      || waldo (9223372036854775807L / -46 - 1) != 1
      || waldo (9223372036854775807L) != 1
      || waldo ((-9223372036854775807L - 1) / -46) != 0
      || waldo ((-9223372036854775807L - 1) / -46 + 1) != 1
      || waldo (-9223372036854775807L - 1) != 1)
    __builtin_abort ();
  return 0;
}
