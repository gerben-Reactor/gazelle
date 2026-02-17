# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111863-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111863-1.c" 2


__attribute__((noipa))
int f(int a)
{
        a &= 2;
        return a != 1;
}
int main(void)
{
        int t = f(1);
        if (!t)
        __builtin_abort();
        __builtin_printf("%d\n",t);
        return 0;
}
