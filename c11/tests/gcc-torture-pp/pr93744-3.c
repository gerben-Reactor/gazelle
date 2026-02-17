# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93744-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93744-3.c" 2


int w;

int
foo (int x, int y, int z)
{
  int r = z + ((w++ - z) & -(x < y));
  return r;
}

int
main ()
{
  w = 4;
  if (foo (5, 7, 12) != 4 || w != 5)
    __builtin_abort ();
  if (foo (7, 5, 12) != 12 || w != 6)
    __builtin_abort ();
  return 0;
}
