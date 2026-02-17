# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/compare-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/compare-2.c" 2







extern void abort (void);

int
foo (int x, int y)
{

  return (x<=y) && ((unsigned int)x >= (unsigned int)y);
}

int
main ()
{
  if (! foo (-1,0))
    abort ();
  return 0;
}
