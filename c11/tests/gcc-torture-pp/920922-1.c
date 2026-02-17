# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920922-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920922-1.c" 2
void abort (void);
void exit (int);

unsigned long*
f(p)unsigned long*p;
{
  unsigned long a = (*p++) >> 24;
  return p + a;
}

int
main (void)
{
  unsigned long x = 0x80000000UL;
  if (f(&x) != &x + 0x81)
    abort();
  exit(0);
}
