# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/strct-pack-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/strct-pack-2.c" 2
void abort (void);
void exit (int);

typedef struct
{
  short a __attribute__ ((aligned (2),packed));
  short *ap[2] __attribute__ ((aligned (2),packed));
} A;

int
main (void)
{
  short i, j = 1;
  A a, *ap = &a;
  ap->ap[j] = &i;
  exit (0);
}
