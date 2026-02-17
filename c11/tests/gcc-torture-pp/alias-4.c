# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-4.c" 2

int a = 1;
extern int b __attribute__ ((alias ("a")));
int c = 1;
extern int d __attribute__ ((alias ("c")));
int
main (int argc)
{
  int *p;
  int *q;
  if (argc)
    p = &a, q = &b;
  else
    p = &c, q = &d;
  *p = 1;
  *q = 2;
  if (*p == 1)
    __builtin_abort ();
  return 0;
}
