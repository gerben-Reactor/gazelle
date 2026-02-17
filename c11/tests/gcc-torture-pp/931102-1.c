# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931102-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931102-1.c" 2

typedef union
{
  struct
    {
      char h, l;
    } b;
} T;

f (x)
     int x;
{
  int num = 0;
  T reg;

  reg.b.l = x;
  while ((reg.b.l & 1) == 0)
    {
      num++;
      reg.b.l >>= 1;
    }
  return num;
}

main ()
{
  if (f (2) != 1)
    abort ();
  exit (0);
}
