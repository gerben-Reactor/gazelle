# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-2c.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-2c.c" 2


extern void abort (void);
extern void exit (int);

int a[2];

__inline__ void f (int b, int o)
{
  unsigned int i;
  int *p;
  for (p = &a[b], i = b; --i < ~0; )
    *--p = i * 3 + o;
}

void
g(int b)
{
  f (b, (int)a);
}

int
main ()
{
  a[0] = a[1] = 0;
  g (2);
  if (a[0] != (int)a || a[1] != (int)a + 3)
    abort ();
  exit (0);
}
