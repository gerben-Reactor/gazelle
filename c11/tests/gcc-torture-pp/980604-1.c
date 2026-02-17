# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980604-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980604-1.c" 2
void abort (void);
void exit (int);

int a = 1;
int b = -1;

int c = 1;
int d = 0;

int
main (void)
{
  double e;
  double f;
  double g;

  f = c;
  g = d;
  e = (a < b) ? f : g;
  if (e)
    abort ();
  exit(0);
}
