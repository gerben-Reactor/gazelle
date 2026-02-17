# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr109925.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr109925.c" 2


int a, c, f;

int
main ()
{
  int g[2];
  for (c = 0; c < 2; c++)
    {
      {
 char h[20], *b = h;
 int d = 48, e = 0;
 while (d && e < 5)
   b[e++] = d /= 10;
 f = e;
      }
      g[f - 2 + c] = 0;
    }
  for (;;)
    {
      for (; a <= 4; a++)
 if (g[0])
   break;
      break;
    }
  if (a != 5)
    __builtin_abort ();
  return 0;
}
