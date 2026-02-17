# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111422.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111422.c" 2


int a, b;
int *c = &b;
unsigned d;
signed char e;
int f = 1;

int
foo (int k, signed char *l)
{
  if (k < 6)
    return a;
  l[0] = l[1] = l[k - 1] = 8;
  return 0;
}

int
bar (int k)
{
  signed char g[11];
  int h = foo (k, g);
  return h;
}

int
main ()
{
  for (; b < 8; b = b + 1)
    ;
  int j;
  int *n[8];
  for (j = 0; 18446744073709551608ULL + bar (*c) + *c + j < 2; j++)
    n[j] = &f;
  for (; e <= 4; e++)
    d = *n[0] == f;
  if (d != 1)
    __builtin_abort ();
}
