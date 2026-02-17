# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950322-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950322-1.c" 2


f (unsigned char *a)
{
  int i, j;
  int x, y;

  j = a[1];
  i = a[0] - j;
  if (i < 0)
    {
      x = 1;
      y = -i;
    }
  else
    {
      x = 0;
      y = i;
    }
  return x + y;
}


main ()
{
  unsigned char a[2];
  a[0] = 8;
  a[1] = 9;
  if (f (a) != 2)
    abort ();
  exit (0);
}
