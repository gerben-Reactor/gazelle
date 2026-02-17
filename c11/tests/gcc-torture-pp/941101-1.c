# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941101-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941101-1.c" 2

f ()
{
  int var = 7;

  if ((var/7) == 1)
    return var/7;
  return 0;
}

main ()
{
  if (f () != 1)
    abort ();
  exit (0);
}
