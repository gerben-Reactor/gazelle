# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64255.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64255.c" 2


__attribute__((noinline, noclone)) void
bar (long i, unsigned long j)
{
  if (i != 1 || j != 1)
    __builtin_abort ();
}

__attribute__((noinline, noclone)) void
foo (long i)
{
  unsigned long j;

  if (!i)
    return;
  j = i >= 0 ? (unsigned long) i : - (unsigned long) i;
  if ((i >= 0 ? (unsigned long) i : - (unsigned long) i) != j)
    __builtin_abort ();
  bar (i, j);
}

int
main ()
{
  foo (1);
  return 0;
}
