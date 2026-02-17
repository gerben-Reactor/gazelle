# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960218-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960218-1.c" 2


int gl;

g (x)
{
  gl = x;
  return 0;
}

f (x)
{
  int a = ~x;
  while (a)
    a = g (a);
}

main ()
{
  f (3);
  if (gl != -4)
    abort ();
  exit (0);
}
