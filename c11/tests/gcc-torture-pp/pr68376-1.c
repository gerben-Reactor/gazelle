# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr68376-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr68376-1.c" 2


int a, b, c = 1;
signed char d;

int
main ()
{
  for (; a < 1; a++)
    for (; b < 1; b++)
      {
 signed char e = ~d;
 if (d < 1)
   e = d;
 d = e;
 if (!c)
   __builtin_abort ();
      }

  if (d != 0)
    __builtin_abort ();

  return 0;
}
