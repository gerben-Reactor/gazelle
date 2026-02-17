# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78720.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78720.c" 2


__attribute__((noinline, noclone)) long int
foo (signed char x)
{
  return x < 0 ? 0x80000L : 0L;
}

__attribute__((noinline, noclone)) long int
bar (signed char x)
{
  return x < 0 ? 0x80L : 0L;
}

__attribute__((noinline, noclone)) long int
baz (signed char x)
{
  return x < 0 ? 0x20L : 0L;
}

int
main ()
{
  if (foo (-1) != 0x80000L || bar (-1) != 0x80L || baz (-1) != 0x20L
      || foo (0) != 0L || bar (0) != 0L || baz (0) != 0L
      || foo (31) != 0L || bar (31) != 0L || baz (31) != 0L)
    __builtin_abort ();
  return 0;
}
