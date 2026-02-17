# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57568.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57568.c" 2


extern void abort (void);
int a[6][9] = { }, b = 1, *c = &a[3][5];

int
main ()
{
  if (b && (*c = *c + *c))
    abort ();
  return 0;
}
