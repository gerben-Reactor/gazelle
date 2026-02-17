# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr120677.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr120677.c" 2



unsigned a;
int b, e;

int
foo (int d)
{
  switch (d)
    {
    case 0:
    case 2:
      return 0;
    default:
      return 1;
    }
}

int
main ()
{
  for (b = 8; b; b--)
    if (a & 1)
      a = a >> 1 ^ 20000000;
    else
      a >>= 1;
  e = foo (0);
  if (e || a)
    __builtin_abort ();
}
