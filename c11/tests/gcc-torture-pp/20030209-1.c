# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030209-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030209-1.c" 2


void abort (void);
void exit (int);

double x[100][100];
int main ()
{
  int i;

  i = 99;
  x[i][0] = 42;
  if (x[99][0] != 42)
    abort ();
  exit (0);
}
