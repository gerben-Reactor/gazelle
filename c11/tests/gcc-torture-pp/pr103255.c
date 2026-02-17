# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103255.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103255.c" 2


struct H
{
  unsigned a;
  unsigned b;
  unsigned c;
};
# 17 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103255.c"
int
main ()
{
  struct H *h = 0;
  unsigned long o;
  volatile int t = 1;

  for (o = 0x20; o <= 0x20; o += 0x1000)
    {
      struct H *u;
      u = (struct H *) (0x400000 + o);
      if (t)
 {
   h = u;
   break;
 }
    }

  if (h == 0)
    return 0;
  unsigned *tt = &h->b;
  if ((long unsigned int) tt != (0x400000 + 0x20 + __builtin_offsetof (struct H, b)))
    __builtin_abort ();
  return 0;
}
