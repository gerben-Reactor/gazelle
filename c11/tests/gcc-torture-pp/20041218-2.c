# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041218-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041218-2.c" 2
extern void abort (void);

int test(int n)
{
  struct s { char b[n]; } __attribute__((packed));
  n++;
  return sizeof(struct s);
}

int main()
{
  if (test(123) != 123)
    abort ();
  return 0;
}
