# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98304-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98304-2.c" 2


# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/../../gcc.dg/pr98304-1.c" 1





__attribute__((noipa)) int foo(int n) {
    return n - (((n > 63) ? n : 63) & -64);
}


__attribute__((noipa)) unsigned int bar(unsigned int n) {
    return n - (((n > 63) ? n : 63) & -64);
}


__attribute__((noipa)) int goo(int n) {
    return n - (((n > 31) ? n : 31) & -32);
}


__attribute__((noipa)) int baz(int n) {
    return n - (((64 > n) ? 63 : n) & -64);
}


__attribute__((noipa)) int fred(int n) {
    return n - (((63 < n) ? n : 63) & -64);
}


__attribute__((noipa)) int qux(int n) {
    return n - (((n > 62) ? n : 62) & -63);
}


__attribute__((noipa)) unsigned int quux(unsigned int n) {
    return n - (((n > 62) ? n : 62) & -63);
}


__attribute__((noipa)) int waldo(int n, int x) {
    return n - (((n > 63) ? n : 63) & x);
}


__attribute__((noipa)) int corge(int n) {
    return n - (((n > 1) ? n : 1) & -64);
}


__attribute__((noipa)) unsigned int thud(unsigned int n)
{
    return n - (((n > 1) ? n : 1) & -64);
}
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98304-2.c" 2


int main() {


    if (foo(-42) != -42
        || foo(0) != 0
        || foo(63) != 63
        || foo(64) != 0
        || foo(65) != 1
        || foo(99) != 35) {
            __builtin_abort();
        }


    if (bar(42) != 42
        || bar(0) != 0
        || bar(63) != 63
        || bar(64) != 0
        || bar(65) != 1
        || bar(99) != 35) {
            __builtin_abort();
        }


    if (corge(13) != 13
        || thud(13) != 13
        || qux(13) != 13
        || quux(13) != 13) {
            __builtin_abort();
        }

    return 0;
}
