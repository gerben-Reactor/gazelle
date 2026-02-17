# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97888-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97888-1.c" 2


int a = 1, c = 4, d, e;

int
main ()
{
  int f = -173;
  int b;
  for (b = 0; b < 10; b++)
    {
      int g = f % (~0 && a), h = 0, i = 0;
      if (g)
 __builtin_unreachable ();
      if (c)
 h = f;
      if (h > -173)
 e = d / i;
      f = h;
    }
  if (f != -173)
    __builtin_abort ();
  return 0;
}
