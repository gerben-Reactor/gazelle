# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/970923-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/970923-1.c" 2
void abort (void);
void exit (int);

int
ts(a)
     int a;
{
  if (a < 1000 && a > 2000)
    return 1;
  else
    return 0;
}

int
tu(a)
     unsigned int a;
{
  if (a < 1000 && a > 2000)
    return 1;
  else
    return 0;
}

int
main(void)
{
  if (ts (0) || tu (0))
    abort ();
  exit (0);
}
