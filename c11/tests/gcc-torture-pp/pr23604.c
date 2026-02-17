# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr23604.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr23604.c" 2
extern void abort (void);

int g(int i, int j)
{
  if (i>-1)
    if (i<2)
     {
        if (i != j)
          {
            if (j != 0)
                return 0;
          }
     }
  return 1;
}

int main(void)
{
  if (!g(1, 0))
   abort ();
  return 0;
}
