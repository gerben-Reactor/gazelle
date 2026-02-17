# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020423-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020423-1.c" 2




extern void abort (void);
extern void exit (int);

int main (void)
{
  int my_int = 924;
  unsigned int result;

  result = ((my_int*2 + 4) - 8U) / 2;
  if (result != 922U)
    abort();

  result = ((my_int*2 - 4U) + 2) / 2;
  if (result != 923U)
    abort();

  result = (((my_int + 2) * 2) - 8U - 4) / 2;
  if (result != 920U)
    abort();
  result = (((my_int + 2) * 2) - (8U + 4)) / 2;
  if (result != 920U)
    abort();

  result = ((my_int*4 + 2U) - 4U) / 2;
  if (result != 1847U)
    abort();

  exit(0);
}
