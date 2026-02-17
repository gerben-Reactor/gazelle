# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20071018-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20071018-1.c" 2
extern void abort(void);

struct foo {
  int rank;
  char *name;
};

struct mem {
  struct foo *x[4];
};

void __attribute__((noinline)) bar(struct foo **f)
{
  *f = __builtin_malloc(sizeof(struct foo));
}
struct foo * __attribute__((noinline, noclone)) foo(int rank)
{
  void *x = __builtin_malloc(sizeof(struct mem));
  struct mem *as = x;
  struct foo **upper = &as->x[rank * 8 - 5];
  *upper = 0;
  bar(upper);
  return *upper;
}

int main()
{
  if (foo(1) == 0)
    abort ();
  return 0;
}
