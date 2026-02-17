# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr19515.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr19515.c" 2


typedef union {
      char a2[8];
}aun;

void abort (void);

int main(void)
{
  aun a = {{0}};

  if (a.a2[2] != 0)
    abort ();
  return 0;
}
