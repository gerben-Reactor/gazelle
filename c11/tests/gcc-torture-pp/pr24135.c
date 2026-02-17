# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr24135.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr24135.c" 2


extern void abort (void);

int x(int a, int b)
{
  __label__ xlab;
  __label__ xlab2;

  void y(int b)
    {
       switch (b)
        {
          case 1: goto xlab;
          case 2: goto xlab;
        }
    }

  a = a + 2;
  y (b);

 xlab:
  return a;

 xlab2:
  a++;
  return a;

}

int main ()
{
  int i, j;

  for (j = 1; j <= 2; ++j)
    for (i = 1; i <= 2; ++i)
      {
 int a = x (j, i);
 if (a != 2 + j)
   abort ();
      }

  return 0;
}
