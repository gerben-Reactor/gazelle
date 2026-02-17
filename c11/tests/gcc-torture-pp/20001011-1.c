# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001011-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001011-1.c" 2
extern void abort(void);
extern int strcmp(const char *, const char *);

int foo(const char *a)
{
    return strcmp(a, "main");
}

int main(void)
{
    if(foo(__FUNCTION__))
        abort();
    return 0;
}
