# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950706-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950706-1.c" 2
void abort (void);
void exit (int);

int
f (int n)
{
  return (n > 0) - (n < 0);
}

int
main (void)
{
  if (f (-1) != -1)
    abort ();
  if (f (1) != 1)
    abort ();
  if (f (0) != 0)
    abort ();
  exit (0);
}
