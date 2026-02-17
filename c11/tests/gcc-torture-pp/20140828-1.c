# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20140828-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20140828-1.c" 2
short *f(short *a, int b, int *d) __attribute__((noinline,noclone));

short *f(short *a, int b, int *d)
{
  short c = *a;
  a++;
  c = b << c;
  *d = c;
  return a;
}

int main(void)
{
  int d;
  short a[2];
  a[0] = 0;
  if (f(a, 1, &d) != &a[1])
    __builtin_abort ();
  if (d != 1)
    __builtin_abort ();
  return 0;
}
