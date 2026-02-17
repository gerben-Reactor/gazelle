# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262-1.c" 2
# 10 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262-1.c"
void abort (void);

struct A
{
  int t;
  int i;
};

int foo () { return 3; }

int
main (void)
{
  struct A loc, *locp;
  float f, g, *p;
  int T355, *T356;



  f = 3;
  g = 2;
  p = foo () ? &g : &f;
  if (*p > 0.0)
    g = 1;


  locp = __builtin_malloc (sizeof (*locp));
  locp->i = 10;
  T355 = locp->i;


  T356 = &locp->i;
  *T356 = 1;



  T355 = locp->i;
  if (T355 != 1)
    abort ();

  return 0;
}
