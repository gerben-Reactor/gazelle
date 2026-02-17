# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20241029-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20241029-1.c" 2



__attribute__((noinline))
void foo(int *self, int *x)
{
    __builtin_puts ("foo\n");

    if (x) {
        while (1) {
            ++*self;
            if (*self == 6) break;
            if (*self == 7) __builtin_unreachable();
        }
    }
}

int main (void)
{
    int y = 0;
    foo (&y, 0);
    return 0;
}
