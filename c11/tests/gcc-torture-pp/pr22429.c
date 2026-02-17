# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr22429.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr22429.c" 2
extern void abort (void);



int f(int n)
{
  if (-(1 << (sizeof(int) * 8 - 2)) <= n && n <= (1 << (sizeof(int) * 8 - 2))-1)
    return 1;
  return 0;
}

int main ()
{
  if (f ((1 << (sizeof(int) * 8 - 2))))
    abort ();
  return 0;
}
