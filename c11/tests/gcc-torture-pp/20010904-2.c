# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010904-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010904-2.c" 2








void abort (void);
void exit (int);

typedef struct x { int a; int b; } __attribute__((aligned(32))) X;
typedef struct y { X x; X y[31]; int c; } Y;

Y y[2];

int main(void)
{
  if (((char *)&y[1] - (char *)&y[0]) & 31)
    abort ();
  exit (0);
}
