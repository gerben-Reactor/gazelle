# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090711-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090711-1.c" 2




extern void abort (void);

long long __attribute__ ((noinline))
div (long long val)
{
  return val / 32768;
}

int main (void)
{
  long long d1 = -990000000;
  long long d2 = div(d1);
  if (d2 != -30212)
    abort ();
  return 0;
}
