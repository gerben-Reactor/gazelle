# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930608-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930608-1.c" 2


void abort (void);
void exit (int);

double f (double a) {}
double (* const a[]) (double) = {&f};

int
main (void)
{
  double (*p) ();
  p = &f;
  if (p != a[0])
    abort ();
  exit (0);
}
