# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr45262.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr45262.c" 2




extern void abort (void);

int
foo (unsigned int x)
{
  return ((int) x < 0) || ((int) (-x) < 0);
}

int
bar (unsigned int x)
{
  return x >> 31 || (-x) >> 31;
}

int
main (void)
{
  if (foo (1) != 1)
    abort ();
  if (foo (0) != 0)
    abort ();
  if (foo (-1) != 1)
    abort ();
  if (bar (1) != 1)
    abort ();
  if (bar (0) != 0)
    abort ();
  if (bar (-1) != 1)
    abort ();
  return 0;
}
