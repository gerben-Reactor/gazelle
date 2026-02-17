# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr115277.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr115277.c" 2
int array[1000];
void
test (int a)
{
        if (__builtin_expect (a > 3, 1))
                return;
        for (int i = 0; i < a; i++)
                array[i]=i;
}
void
test2 (int a)
{
        if (__builtin_expect (a > 10, 1))
                return;
        for (int i = 0; i < a; i++)
                array[i]=i;
}
int
main()
{
        test(1);
        test(2);
        test(3);
        test2(10);
        if (array[9] != 9)
                __builtin_abort ();
        return 0;
}
