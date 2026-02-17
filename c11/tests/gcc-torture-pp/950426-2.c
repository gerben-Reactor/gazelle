# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950426-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950426-2.c" 2
void abort (void);
void exit (int);

int
main(void)
{
  long int i = -2147483647L - 1L;
  char ca = 1;

  if (i >> ca != -1073741824L)
    abort ();

  if (i >> i / -2000000000L != -1073741824L)
    abort ();

  exit (0);
}
