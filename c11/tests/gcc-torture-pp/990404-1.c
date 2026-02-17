# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990404-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990404-1.c" 2
void abort (void);
void exit (int);

int x[10] = { 0,1,2,3,4,5,6,7,8,9};

int
main()
{
  int niterations = 0, i;

  for (;;) {
    int i, mi, max;
    max = 0;
    for (i = 0; i < 10 ; i++) {
      if (x[i] > max) {
 max = x[i];
 mi = i;
      }
    }
    if (max == 0)
      break;
    x[mi] = 0;
    niterations++;
    if (niterations > 10)
      abort ();
  }

  exit (0);
}
