# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93434.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93434.c" 2
typedef struct creal_T {
  double re;
  double im;
} creal_T;


int main() {
  int k;
  int i;
  int j;
  creal_T t2[16];
  double inval;

  inval = 1.0;
  for (j = 0; j < 16; ++j) {
    t2[j].re = 0;
    t2[j].im = 0;
  }

  for (j = 0; j < 16/4; j++) {
    i = j * 4;
    t2[i].re = inval;
    t2[i].im = inval;
    k = i + 3;
    t2[k].re = inval;
    t2[k].im = inval;
    t2[i] = t2[k];
    t2[k].re = inval;
  }

  for (i = 0; i < 2; ++i)
    if (t2[i].re != !i || t2[i].im != !i)
      __builtin_abort ();

  return 0;
}
