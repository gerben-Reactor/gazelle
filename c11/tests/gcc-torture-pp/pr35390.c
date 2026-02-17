# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr35390.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr35390.c" 2
extern void abort (void);

unsigned int foo (int n)
{
  return ~((unsigned int)~n);
}

int main()
{
  if (foo(0) != 0)
    abort ();
  return 0;
}
