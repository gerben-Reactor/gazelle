# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/940122-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/940122-1.c" 2


char *a = 0;
char *b = 0;

g (x)
     int x;
{
  if ((!!a) != (!!b))
    abort ();
}

f (x)
     int x;
{
  g (x * x);
}

main ()
{
  f (100);
  exit (0);
}
