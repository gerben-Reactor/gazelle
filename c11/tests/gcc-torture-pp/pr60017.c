# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60017.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60017.c" 2


extern void abort (void);

struct S0
{
  short m0;
  short m1;
};

struct S1
{
  unsigned m0:1;
  char m1[2][2];
  struct S0 m2[2];
};

struct S1 x = { 1, {{2, 3}, {4, 5}}, {{6, 7}, {8, 9}} };

struct S1 func (void)
{
  return x;
}

int main (void)
{
  struct S1 ret = func ();

  if (ret.m2[1].m1 != 9)
    abort ();

  return 0;
}
