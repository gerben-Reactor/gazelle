# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030404-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030404-1.c" 2




void abort (void);

int
main(void)
{
  int i, j, k;

  k = 0;
  while (k < 10)
    {
      k++;
      if (j > i)
 j = 5;
      else
 j =3;
    }

  if (k != 10)
    abort ();

  return 0;
}
