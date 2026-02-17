# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980602-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980602-1.c" 2
void abort (void);
void exit (int);

int
main(void)
{
  int i;
  for (i = 1; i < 100; i++)
    ;
  if (i == 100)
    exit (0);
  abort ();
}
