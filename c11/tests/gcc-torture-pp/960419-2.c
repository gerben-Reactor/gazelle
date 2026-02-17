# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960419-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960419-2.c" 2
void abort (void);
void exit (int);



int
main(void)
{
  int a[8] = {1};
  int i;

  for (i = 1; i < 8; i++)
    if (a[i] != 0)
      abort();

  exit (0);
}
