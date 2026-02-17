# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr15262.c" 2



void abort (void);

struct A
{
  int t;
  int i;
};

void
bar (float *p)
{
  *p = 5.2;
}

int
foo(struct A *locp, int i, int str)
{
  float f, g, *p;
  int T355;
  int *T356;



  p = i ? &g : &f;
  bar (p);
  if (*p > 0.0)
    str = 1;

  T355 = locp->i;
  T356 = &locp->i;
  *T356 = str;
  T355 = locp->i;

  return T355;
}

int
main (void)
{
  struct A loc;
  int str;

  loc.i = 2;
  str = foo (&loc, 10, 3);
  if (str!=1)
    abort ();
  return 0;
}
