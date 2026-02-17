# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97404.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97404.c" 2



char a, b;
long c;
short d, e;
long *f = &c;
int g;
char h(signed char i) { return 0; }
static short j(short i, int k) { return i < 0 ? 0 : i >> k; }
void l(void);
void m(void)
{
  e = j(d | 9766, 11);
    *f = e;
}
void l(void)
{
  a = 5 | g;
    b = h(a);
}
int main()
{
  m();
  if (c != 4)
    __builtin_abort();
  return 0;
}
