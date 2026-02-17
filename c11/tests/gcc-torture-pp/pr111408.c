# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111408.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111408.c" 2


int a, b, c, d;
short e;

int
foo ()
{
  c = a % (sizeof (int) * 8);
  if (b & 1 << c)
    return -1;
  return 0;
}

int
main ()
{
  for (; e != 1; e++)
    {
      int g = foo ();
      if (g + d - 9 + d)
 continue;
      for (;;)
 __builtin_abort ();
    }
}
