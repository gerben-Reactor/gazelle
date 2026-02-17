# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr86844.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr86844.c" 2


__attribute__((noipa)) void
foo (int *p)
{
  *p = 0;
  *((char *)p + 3) = 1;
  *((char *)p + 1) = 2;
  *((char *)p + 2) = *((char *)p + 6);
}

int
main ()
{
  int a[2] = { -1, 0 };
  if (sizeof (int) != 4)
    return 0;
  ((char *)a)[6] = 3;
  foo (a);
  if (((char *)a)[0] != 0 || ((char *)a)[1] != 2
      || ((char *)a)[2] != 3 || ((char *)a)[3] != 1)
    __builtin_abort ();
  return 0;
}
