# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930630-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930630-1.c" 2
# 11 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930630-1.c"
main ()
{
  struct
    {
      signed int bf0:17;
      signed int bf1:7;
    } bf;

  bf.bf1 = 7;
  f (bf.bf1);
  exit (0);
}

f (x)
     int x;
{
  if (x != 7)
    abort ();
}
