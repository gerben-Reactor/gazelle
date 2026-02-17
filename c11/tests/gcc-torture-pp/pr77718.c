# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr77718.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr77718.c" 2


char a[64] __attribute__((aligned (8)));

__attribute__((noinline, noclone)) int
foo (void)
{
  return __builtin_memcmp ("bbbbbb", a, 6);
}

__attribute__((noinline, noclone)) int
bar (void)
{
  return __builtin_memcmp (a, "bbbbbb", 6);
}

int
main ()
{
  __builtin_memset (a, 'a', sizeof (a));
  if (((foo () < 0) ^ ('a' > 'b'))
      || ((bar () < 0) ^ ('a' < 'b')))
    __builtin_abort ();
  return 0;
}
