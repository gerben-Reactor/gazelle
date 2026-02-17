# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020720-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020720-1.c" 2






extern void abort (void);
extern double fabs (double);
extern void link_error (void);

void
foo (double x)
{
  double p, q;

  p = fabs (x);
  q = 0.0;
  if (p < q)
    link_error ();
}

int
main()
{
  foo (1.0);
  return 0;
}


void
link_error ()
{
  abort ();
}
