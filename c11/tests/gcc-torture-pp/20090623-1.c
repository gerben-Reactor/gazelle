# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090623-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090623-1.c" 2
int * __restrict__ x;

int foo (int y)
{
  *x = y;
  return *x;
}

extern void abort (void);

int main()
{
  int i = 0;
  x = &i;
  if (foo(1) != 1)
    abort ();
  return 0;
}
