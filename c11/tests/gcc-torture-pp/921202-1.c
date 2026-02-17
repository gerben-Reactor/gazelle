# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921202-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921202-1.c" 2
# 10 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921202-1.c"
main ()
{
  long dx[2055 +1];
  long dy[2055 +1];
  long s1[2055];
  int cyx, cyy;
  int i;
  long size;

  for (;;)
    {
      size = 2055;
      mpn_random2 (s1, size);

      for (i = 0; i < 1; i++)
 ;

      dy[size] = 0x12345678;

      for (i = 0; i < 1; i++)
 cyy = mpn_mul_1 (dy, s1, size);

      if (cyx != cyy || mpn_cmp (dx, dy, size + 1) != 0 || dx[size] != 0x12345678)
 {
   foo ("", 8, cyy); mpn_print (dy, size);
 }
      exxit();
    }
}

foo (){}
mpn_mul_1(){}
mpn_print (){}
mpn_random2(){}
mpn_cmp(){}
exxit(){exit(0);}
