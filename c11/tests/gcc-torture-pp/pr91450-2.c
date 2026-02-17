# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr91450-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr91450-2.c" 2


__attribute__((noipa)) void
foo (int a, int b)
{
  unsigned long long r;
  if (__builtin_mul_overflow (a, b, &r))
    __builtin_abort ();
  if (r != 0)
    __builtin_abort ();
}

__attribute__((noipa)) void
bar (int a, int b)
{
  unsigned long long r;
  if (a >= 0)
    return;
  if (__builtin_mul_overflow (a, b, &r))
    __builtin_abort ();
  if (r != 0)
    __builtin_abort ();
}

__attribute__((noipa)) void
baz (int a, int b)
{
  unsigned long long r;
  if (b >= 0)
    return;
  if (__builtin_mul_overflow (a, b, &r))
    __builtin_abort ();
  if (r != 0)
    __builtin_abort ();
}

__attribute__((noipa)) void
qux (int a, int b)
{
  unsigned long long r;
  if (a >= 0)
    return;
  if (b < 0)
    return;
  if (__builtin_mul_overflow (a, b, &r))
    __builtin_abort ();
  if (r != 0)
    __builtin_abort ();
}

__attribute__((noipa)) void
quux (int a, int b)
{
  unsigned long long r;
  if (a < 0)
    return;
  if (b >= 0)
    return;
  if (__builtin_mul_overflow (a, b, &r))
    __builtin_abort ();
  if (r != 0)
    __builtin_abort ();
}

int
main ()
{
  foo (-4, 0);
  foo (0, -4);
  foo (0, 0);
  bar (-4, 0);
  baz (0, -4);
  qux (-4, 0);
  quux (0, -4);
  return 0;
}
