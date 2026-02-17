# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-2.c" 2


int a[10]={};
extern int b[10] __attribute__ ((alias("a")));
int off;
int
main(void)
{
  b[off]=1;
  a[off]=2;
  if (b[off]!=2)
   __builtin_abort ();
  return 0;
}
