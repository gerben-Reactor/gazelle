# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr109778.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr109778.c" 2


int a, b, c, d, *e = &c;

static inline unsigned
foo (unsigned char x)
{
  x = 1 | x << 1;
  x = x >> 4 | x << 4;
  return x;
}

static inline void
bar (unsigned x)
{
  *e = 8 > foo (x + 86) - 86;
}

int
main ()
{
  d = a && b;
  bar (d + 4);
  if (c != 1)
    __builtin_abort ();
}
