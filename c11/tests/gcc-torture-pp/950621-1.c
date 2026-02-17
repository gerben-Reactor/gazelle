# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950621-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950621-1.c" 2


struct s
{
  int a;
  int b;
  struct s *dummy;
};

f (struct s *sp)
{
  return sp && sp->a == -1 && sp->b == -1;
}

main ()
{
  struct s x;
  x.a = x.b = -1;
  if (f (&x) == 0)
    abort ();
  exit (0);
}
