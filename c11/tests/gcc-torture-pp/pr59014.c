# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr59014.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr59014.c" 2


int a = 2, b, c, d;

int
foo ()
{
  for (;; c++)
    if ((b > 0) | (a & 1))
      ;
    else
      {
 d = a;
 return 0;
      }
}

int
main ()
{
  foo ();
  if (d != 2)
    __builtin_abort ();
  return 0;
}
