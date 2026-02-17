# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930702-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930702-1.c" 2

fp (double a, int b)
{
  if (a != 33 || b != 11)
    abort ();
}

main ()
{
  int (*f) (double, int) = fp;

  fp (33, 11);
  f (33, 11);
  exit (0);
}
