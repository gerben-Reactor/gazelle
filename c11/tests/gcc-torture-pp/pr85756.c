# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85756.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85756.c" 2



int a, c, *e, f, h = 10;
short b;
unsigned int p;

__attribute__((noipa)) void
bar (int a)
{
  asm volatile ("" : : "r" (a) : "memory");
}

void
foo ()
{
  unsigned j = 1, m = 430523;
  int k, n = 1, *l = &h;
lab:
  p = m;
  m = -((~65535U | j) - n);
  f = b << ~(n - 8);
  n = (m || b) ^ f;
  j = p;
  if (p < m)
    *l = k < 3;
  if (!n)
    l = &k;
  if (c)
    {
      bar (a);
      goto lab;
    }
  if (!*l)
    *e = 1;
}

int
main ()
{
  foo ();
  return 0;
}
