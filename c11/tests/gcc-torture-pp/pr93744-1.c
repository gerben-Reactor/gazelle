# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93744-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93744-1.c" 2


typedef int I;

int
main ()
{
  int a = 0;
  I b = 0;
  (a > 0) * (b |= 2);
  if (b != 2)
    __builtin_abort ();
  return 0;
}
