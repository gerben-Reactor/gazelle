# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010106-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010106-1.c" 2



void abort (void);
void exit (int);

int f(int i) {
  switch (i)
  {
    case -2:
      return 33;
    case -1:
      return 0;
    case 0:
      return 7;
    case 1:
      return 4;
    case 2:
      return 3;
    case 3:
      return 15;
    case 4:
     return 9;
    default:
      abort ();
  }
}

int main() {
  if (f(-1))
    abort ();
  exit (0);
}
