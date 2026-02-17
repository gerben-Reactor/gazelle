# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931110-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931110-1.c" 2
void exit (int);

typedef struct
{
  short f:3, g:3, h:10;
} small;

struct
{
  int i;
  small s[10];
} x;

int
main (void)
{
  int i;
  for (i = 0; i < 10; i++)
    x.s[i].f = 0;
  exit (0);
}
