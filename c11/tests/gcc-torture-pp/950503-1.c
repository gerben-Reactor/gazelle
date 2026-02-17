# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950503-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950503-1.c" 2
void abort (void);
void exit (int);

int
main (void)
{
  int tmp;
  unsigned long long utmp1, utmp2;

  tmp = 16;

  utmp1 = (~((unsigned long long) 0)) >> tmp;
  utmp2 = (~((unsigned long long) 0)) >> 16;

  if (utmp1 != utmp2)
    abort ();
  exit (0);
}
