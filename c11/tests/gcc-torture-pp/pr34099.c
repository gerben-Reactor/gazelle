# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34099.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34099.c" 2
int foo (int b, int c)
{
  int x;
  if (b)
    return x & c;
  else
    return 1;
}
extern void abort (void);
int main()
{
  if (foo(1, 0) != 0)
    abort ();
  return 0;
}
