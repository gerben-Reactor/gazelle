# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921017-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921017-1.c" 2



f(n)
{
  int a[n];
  int g(i)
    {
      return a[i];
    }
  a[1]=4711;
  return g(1);
}
main()
{
  if(f(2)!=4711)abort();

  exit(0);
}
