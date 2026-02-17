# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961122-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961122-2.c" 2
void abort (void);
void exit (int);

int
f (int a)
{
  return ((a >= 0 && a <= 10) && ! (a >= 0));
}

int
main (void)
{
  if (f (0))
    abort ();
  exit (0);
}
