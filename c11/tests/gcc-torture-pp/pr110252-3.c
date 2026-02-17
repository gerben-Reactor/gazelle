# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110252-3.c" 2

unsigned int a = 1387579096U;
void sinkandcheck(unsigned b) __attribute__((noipa));
void sinkandcheck(unsigned b)
{
        if (a != b)
        __builtin_abort();
}
int main() {
    a = 1 < (~a) ? 1 : (~a);
    sinkandcheck(1);
    return 0;
}
