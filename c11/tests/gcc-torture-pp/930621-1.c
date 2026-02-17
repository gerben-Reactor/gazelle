# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930621-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930621-1.c" 2
# 11 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930621-1.c"
f ()
{
  struct {
    int x : 18;
    int y : 14;
  } foo;

  foo.x = 10;
  foo.y = 20;

  return foo.y;
}

main ()
{
  if (f () != 20)
    abort ();
  exit (0);
}
