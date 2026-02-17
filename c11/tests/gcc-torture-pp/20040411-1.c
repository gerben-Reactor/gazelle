# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040411-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040411-1.c" 2
void abort (void);

int
sub1 (int i, int j)
{
  typedef int c[i+2];
  int x[10], y[10];

  if (j == 2)
    {
      __builtin_memcpy (x, y, 10 * sizeof (int));
      return sizeof (c);
    }
  else
    return sizeof (c) * 3;
}

int
main ()
{
  if (sub1 (20, 3) != 66 * sizeof (int))
    abort ();

  return 0;
}
