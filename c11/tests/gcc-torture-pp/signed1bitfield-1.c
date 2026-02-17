# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/signed1bitfield-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/signed1bitfield-1.c" 2






struct {
  signed b : 1;
} f;
int i = 55;
__attribute__((noinline))
void check(int a)
{
        if (!a)
        __builtin_abort();
}
int main() {
    int t = i != 5;
    t = t*5;
    f.b = t;
    int tt = f.b;
    check(f.b);
}
