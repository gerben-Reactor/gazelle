# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930603-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930603-1.c" 2


float fx (x)
     float x;
{
  return 1.0 + 3.0 / (2.302585093 * x);
}

main ()
{
  float fx (), inita (), initc (), a, b, c;
  a = inita ();
  c = initc ();
  f ();
  b = fx (c) + a;
  f ();
  if (a != 3.0 || b < 4.3257 || b > 4.3258 || c != 4.0)
    abort ();
  exit (0);
}

float inita () { return 3.0; }
float initc () { return 4.0; }
f () {}
