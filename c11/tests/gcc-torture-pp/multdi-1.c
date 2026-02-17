# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/multdi-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/multdi-1.c" 2


void abort (void);




long long mpy_res;

unsigned long long mpy (long a, long b)
{
  return (unsigned long long) a * (unsigned long long) b;
}

int main(void)
{
  mpy_res = mpy(1,-1);
  if (mpy_res != -1LL)
    abort ();
  return 0;
}
