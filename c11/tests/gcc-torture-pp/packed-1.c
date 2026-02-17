# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/packed-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/packed-1.c" 2
void abort (void);
void exit (int);

short x1 = 17;

struct
{
  short i __attribute__ ((packed));
} t;

void
f ()
{
  t.i = x1;
  if (t.i != 17)
    abort ();
}

int
main (void)
{
  f ();
  exit (0);
}
