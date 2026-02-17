# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931009-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931009-1.c" 2

main ()
{
  f ();
  exit (0);
}

static
g (out, size, lo, hi)
     int *out, size, lo, hi;
{
  int j;

  for (j = 0; j < size; j++)
    out[j] = j * (hi - lo);
}


f ()
{
  int a[2];

  g (a, 2, 0, 1);

  if (a[0] != 0 || a[1] != 1)
    abort ();
}
