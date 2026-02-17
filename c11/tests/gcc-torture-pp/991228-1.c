# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991228-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991228-1.c" 2
void abort (void);
void exit (int);

__extension__ union { double d; int i[2]; } u = { d: -0.25 };




long long endianness_test = 1;


int
signbit(double x)
{
  __extension__ union { double d; int i[2]; } u = { d: x };
  return u.i[(*(int*)&endianness_test)] < 0;
}

int main(void)
{
  if (2*sizeof(int) != sizeof(double) || u.i[(*(int*)&endianness_test)] >= 0)
    exit(0);

  if (!signbit(-0.25))
    abort();

  exit(0);
}
