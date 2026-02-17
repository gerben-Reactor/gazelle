# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930126-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930126-1.c" 2
void abort (void);
void exit (int);

struct s {
  unsigned long long a:8, b:32;
};

struct s
f(struct s x)
{
  x.b = 0xcdef1234;
  return x;
}

int
main(void)
{
  static struct s i;
  i.a = 12;
  i = f(i);
  if (i.a != 12 || i.b != 0xcdef1234)
    abort();
  exit(0);
}
