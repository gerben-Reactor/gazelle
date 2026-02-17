# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961004-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961004-1.c" 2
void abort (void);
void exit (int);

int k = 0;

int
main(void)
{
  int i;
  int j;

  for (i = 0; i < 2; i++)
    {
      if (k)
 {
   if (j != 2)
     abort ();
 }
      else
 {
   j = 2;
   k++;
 }
    }
  exit (0);
}
