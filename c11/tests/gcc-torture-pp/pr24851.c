# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr24851.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr24851.c" 2






void abort(void);
int main()
{
  int a[10], *p, *q;
  q = &a[1];
  p = &q[-1];
  if (p >= &a[9])
    abort ();
  return 0;
}
