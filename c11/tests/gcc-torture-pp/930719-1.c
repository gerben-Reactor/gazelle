# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930719-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930719-1.c" 2


int
f (foo, bar, com)
{
  unsigned align;
  if (foo)
    return 0;
  while (1)
    {
      switch (bar)
 {
 case 1:
   if (com != 0)
      return align;
   *(char *) 0 = 0;
 }
    }
}

main ()
{
  f (0, 1, 1);
  exit (0);
}
