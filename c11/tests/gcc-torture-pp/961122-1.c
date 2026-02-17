# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961122-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961122-1.c" 2
void abort (void);
void exit (int);

long long acc;

void
addhi (short a)
{
  acc += (long long) a << 32;
}

void
subhi (short a)
{
  acc -= (long long) a << 32;
}

int
main (void)
{
  acc = 0xffff00000000ll;
  addhi (1);
  if (acc != 0x1000000000000ll)
    abort ();
  subhi (1);
  if (acc != 0xffff00000000ll)
    abort ();
  exit (0);
}
