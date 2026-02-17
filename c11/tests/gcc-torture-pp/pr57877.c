# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57877.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57877.c" 2


extern void abort (void);
int a, b, *c = &b, e, f = 6, g, h;
short d;

static unsigned char
foo (unsigned long long p1, int *p2)
{
  for (; g <= 0; g++)
    {
      short *i = &d;
      int *j = &e;
      h = *c;
      *i = h;
      *j = (*i == *p2) < p1;
    }
  return 0;
}

int
main ()
{
  foo (f, &a);
  if (e != 1)
    abort ();
  return 0;
}
