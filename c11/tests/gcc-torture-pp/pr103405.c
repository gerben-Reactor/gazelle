# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103405.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr103405.c" 2
typedef long unsigned int size_t;
void *malloc(size_t);

static inline int *starter(int a)
{
    int *b = malloc(sizeof(int));
    *b = a;
    return b;
}

static inline _Bool equal(int *l, int *r)
{
    if (l == 0)
      __builtin_abort();
    if (r == 0)
      __builtin_abort();
    return *r == *l;
}

int main(void)
{
  int *i;
  int *j;
  void check(_Bool a)
  {
    _Bool t = equal(i, j);
    if (a && t) __builtin_abort ();
    _Bool t1 = equal(i, j);
    if (!a && !t1) __builtin_abort ();
  }
  i = starter(1);
  j = starter(0);
  check(1);
  i = starter(0);
  check(0);

}
