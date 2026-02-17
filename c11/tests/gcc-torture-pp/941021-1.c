# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941021-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941021-1.c" 2
void abort (void);
void exit (int);

double glob_dbl;

void
f (pdbl, value)
     double *pdbl;
     double value;
{
  if (pdbl == 0)
    pdbl = &glob_dbl;

  *pdbl = value;
}

int
main (void)
{
  f ((void *) 0, 55.1);

  if (glob_dbl != 55.1)
    abort ();
  exit (0);
}
