# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-5.c" 2


extern void abort (void);

struct s
{
  unsigned long long a:2;
  unsigned long long b:40;
  unsigned long long c:22;
};

__attribute__ ((noinline)) void
g (unsigned long long a, unsigned long long b)
{
  asm ("");
  if (a != b)
    abort ();
}

__attribute__ ((noinline)) void
f (struct s s, unsigned long long b)
{
  asm ("");
  g (((unsigned long long) (s.b-8)) + 8, b);
}

int
main ()
{
  struct s s = {1, 10, 3};
  struct s t = {1, 2, 3};
  f (s, 10);
  f (t, 0x10000000002);
  return 0;
}
