# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921218-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921218-2.c" 2


f()
{
  long l2;
  unsigned short us;
  unsigned long ul;
  short s2;

  ul = us = l2 = s2 = -1;
  return ul;
}

main()
{
  if (f()!=(unsigned short)-1)
    abort();
  exit(0);
}
