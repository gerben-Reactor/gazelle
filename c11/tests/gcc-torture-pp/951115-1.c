# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/951115-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/951115-1.c" 2
void abort (void);
void exit (int);

int var = 0;

void
g ()
{
  var = 1;
}

void
f ()
{
  int f2 = 0;

  if (f2 == 0)
    ;

  g ();
}

int
main (void)
{
  f ();
  if (var != 1)
    abort ();
  exit (0);
}
