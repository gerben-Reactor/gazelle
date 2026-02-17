# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20031215-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20031215-1.c" 2





typedef struct {int c, l; char ch[3];} pstr;
const pstr ao = {2, 2, "OK"};
const pstr * const a = &ao;

void test1(void)
{
    if (a->ch[a->l]) {
        ((char *)a->ch)[a->l] = 0;
    }
}

void test2(void)
{
    if (a->ch[a->l]) {
        ((char *)a->ch)[a->l] = -1;
    }
}

void test3(void)
{
    if (a->ch[a->l]) {
        ((char *)a->ch)[a->l] = 1;
    }
}

int main(void)
{
    test1();
    test2();
    test3();
    return 0;
}
