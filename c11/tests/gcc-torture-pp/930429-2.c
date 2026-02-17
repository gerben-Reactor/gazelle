# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930429-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930429-2.c" 2


int
f (b)
{
  return (b >> 1) > 0;
}

main ()
{
  if (!f (9))
    abort ();
  if (f (-9))
    abort ();
  exit (0);
}
