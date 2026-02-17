# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010403-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010403-1.c" 2
void abort (void);
void exit (int);

void b (int *);
void c (int, int);
void d (int);

int e;

void a (int x, int y)
{
  int f = x ? e : 0;
  int z = y;

  b (&y);
  c (z, y);
  d (f);
}

void b (int *y)
{
  (*y)++;
}

void c (int x, int y)
{
  if (x == y)
    abort ();
}

void d (int x)
{
}

int main (void)
{
  a (0, 0);
  exit (0);
}
