# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960909-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960909-1.c" 2
void abort (void);
void exit (int);

int
ffs (x)
     int x;
{
  int bit, mask;

  if (x == 0)
    return 0;

  for (bit = 1, mask = 1; !(x & mask); bit++, mask <<= 1)
    ;

  return bit;
}

void
f (x)
     int x;
{
  int y;
  y = ffs (x) - 1;
  if (y < 0)
    abort ();
}

int
main (void)
{
  f (1);
  exit (0);
}
