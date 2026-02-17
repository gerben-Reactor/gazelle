# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941202-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941202-1.c" 2

g (x, y)
{
  if (x != 3)
    abort ();
}

static inline
f (int i)
{
  int *tmp;

  tmp = (int *) alloca (sizeof (i));
  *tmp = i;
  g (*tmp, 0);
}

main ()
{
  f (3);
  exit (0);
};
