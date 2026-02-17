# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960513-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960513-1.c" 2
void abort (void);
void exit (int);

long double
f (d, i)
     long double d;
     int i;
{
  long double e;

  d = -d;
  e = d;
  if (i == 1)
    d *= 2;
  d -= e * d;
  d -= e * d;
  d -= e * d;
  d -= e * d;
  d -= e * d;
  return d;
}

int
main (void)
{
  if (! (int) (f (2.0L, 1)))
    abort ();
  exit (0);
}
