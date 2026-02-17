# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60072.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60072.c" 2


int c = 1;

__attribute__ ((optimize (1)))
static int *foo (int *p)
{
  return p;
}

int
main ()
{
  *foo (&c) = 2;
  return c - 2;
}
