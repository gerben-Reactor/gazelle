# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941014-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941014-1.c" 2
void abort (void);
void exit (int);

int f (int a, int b) { }

int
main (void)
{
  unsigned long addr1;
  unsigned long addr2;

  addr1 = (unsigned long) &f;
  addr1 += 5;
  addr2 = 5 + (unsigned long) &f;

  if (addr1 != addr2)
    abort ();
  exit (0);
}
