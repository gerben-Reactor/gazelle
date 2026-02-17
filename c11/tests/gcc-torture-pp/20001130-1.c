# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001130-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001130-1.c" 2
void abort(void);
static inline int bar(void) { return 1; }
static int mem[3];

static int foo(int x)
{
  if (x != 0)
    return x;

  mem[x++] = foo(bar());

  if (x != 1)
    abort();

  return 0;
}

int main()
{
  foo(0);
  return 0;
}
