# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr19005.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr19005.c" 2

extern void abort (void);

int v, s;

void
bar (int a, int b)
{
  unsigned char x = v;

  if (!s)
    {
      if (a != x || b != (unsigned char) (x + 1))
        abort ();
    }
  else if (a != (unsigned char) (x + 1) || b != x)
    abort ();
  s ^= 1;
}

int
foo (int x)
{
  unsigned char a = x, b = x + 1;

  bar (a, b);
  a ^= b; b ^= a; a ^= b;
  bar (a, b);
  return 0;
}

int
main (void)
{
  for (v = -10; v < 266; v++)
    foo (v);
  return 0;
}
