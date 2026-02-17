# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20111227-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20111227-1.c" 2



extern void abort (void);

void __attribute__((noinline,noclone))
bar (int a)
{
  if (a != -1)
    abort ();
}

void __attribute__((noinline,noclone))
foo (short *a, int t)
{
  short r = *a;

  if (t)
    bar ((unsigned short) r);
  else
    bar ((signed short) r);
}

short v = -1;

int main(void)
{
  foo (&v, 0);
  return 0;
}
