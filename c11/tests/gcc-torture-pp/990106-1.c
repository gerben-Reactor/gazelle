# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990106-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990106-1.c" 2


foo(bufp)
char *bufp;
{
    int x = 80;
    return (*bufp++ = x ? 'a' : 'b');
}

main()
{
  char x;

  if (foo (&x) != 'a')
    abort ();

  exit (0);
}
