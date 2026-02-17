# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/shiftopt-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/shiftopt-1.c" 2






extern void abort (void);
extern void link_error (void);

void
utest (unsigned int x)
{
  if (x >> 0 != x)
    link_error ();

  if (x << 0 != x)
    link_error ();

  if (0 << x != 0)
    link_error ();

  if (0 >> x != 0)
    link_error ();

  if (-1 >> x != -1)
    link_error ();

  if (~0 >> x != ~0)
    link_error ();
}

void
stest (int x)
{
  if (x >> 0 != x)
    link_error ();

  if (x << 0 != x)
    link_error ();

  if (0 << x != 0)
    link_error ();

  if (0 >> x != 0)
    link_error ();
}

int
main ()
{
  utest(9);
  utest(0);

  stest(9);
  stest(0);

  return 0;
}


void
link_error ()
{
  abort ();
}
