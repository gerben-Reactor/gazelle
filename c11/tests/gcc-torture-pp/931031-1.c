# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931031-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931031-1.c" 2
# 12 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931031-1.c"
struct foo
{
  unsigned y:1;
  unsigned x:32;
};

int
f (x)
     struct foo x;
{
  int t = x.x;
  if (t < 0)
    return 1;
  return t+1;
}

main ()
{
  struct foo x;
  x.x = -1;
  if (f (x) == 0)
    abort ();
  exit (0);
}
