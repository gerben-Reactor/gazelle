# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930106-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930106-1.c" 2
# 10 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930106-1.c"
double g()
{
  return 1.0;
}

f()
{
  char dummy[399999];
  double f1, f2, f3;
  f1 = g();
  f2 = g();
  f3 = g();
  return f1 + f2 + f3;
}

main()
{
  if (f() != 3.0)
    abort();
  exit(0);
}
