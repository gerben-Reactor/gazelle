# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr82388.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr82388.c" 2


struct A { int b; int c; int d; } e;

struct A
foo (void)
{
  struct A h[30] = {{0,0,0}};
  return h[29];
}

int
main ()
{
  e = foo ();
  return e.b;
}
