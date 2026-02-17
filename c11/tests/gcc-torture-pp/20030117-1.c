# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030117-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030117-1.c" 2
void abort (void);
void exit (int);

int foo (int, int, int);
int bar (int, int, int);

int main (void)
{
  if (foo (5, 10, 21) != 12)
    abort ();

  if (bar (9, 12, 15) != 150)
    abort ();

  exit (0);
}

int foo (int x, int y, int z)
{
  return (x + y + z) / 3;
}

int bar (int x, int y, int z)
{
  return foo (x * x, y * y, z * z);
}
