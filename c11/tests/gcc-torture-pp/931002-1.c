# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931002-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931002-1.c" 2


void exit (int);

void
f (void (*func) ())
{
  func ();
}

int
main (void)
{
  void t0 ()
    {
    }

  void t1 ()
    {
      f (t0);
    }

  void t2 ()
    {
      t1 ();
    }

  t1 ();
  t1 ();
  t2 ();

  exit (0);
}
