# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020819-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020819-1.c" 2


foo ()
{
  return 0;
}

main()
{
  int i, j, k, ccp_bad = 0;

  for (i = 0; i < 10; i++)
    {
      for (j = 0; j < 10; j++)
 if (foo ())
   ccp_bad = 1;

      k = ccp_bad != 0;
      if (k)
 abort ();
    }

  exit (0);
}
