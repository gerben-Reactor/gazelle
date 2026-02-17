# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041019-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041019-1.c" 2


test_store_ccp (int i)
{
  int *p, a, b, c;

  if (i < 5)
    p = &a;
  else if (i > 8)
    p = &b;
  else
    p = &c;

  *p = 10;
  b = 3;


  return *p + 2;
}


test_store_copy_prop (int i)
{
  int *p, a, b, c;

  if (i < 5)
    p = &a;
  else if (i > 8)
    p = &b;
  else
    p = &c;

  *p = i;
  b = i + 1;


  return *p;
}


main()
{
  int x;

  x = test_store_ccp (10);
  if (x == 12)
    abort ();

  x = test_store_copy_prop (9);
  if (x == 9)
    abort ();

  return 0;
}
