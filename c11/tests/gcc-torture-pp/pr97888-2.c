# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97888-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97888-2.c" 2


__attribute__((noipa)) void
foo (int i)
{
  if ((i % 7) >= 0)
    {
      if (i >= 0)
        __builtin_abort ();
    }
}

int
main ()
{
  foo (-7);
  foo (-21);
  return 0;
}
