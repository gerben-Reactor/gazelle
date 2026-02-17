# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-10.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-10.c" 2





struct f{signed t:1;};
int g(struct f *a, int t) __attribute__((noipa));
int g(struct f *a, int t)
{
    if (t)
      a->t = -1;
    else
      a->t = 0;
    int t1 = a->t;
    if (t1) return 1;
    return t1;
}

int main(void)
{
    struct f a;
    if (!g(&a, 1)) __builtin_abort();
    return 0;
}
