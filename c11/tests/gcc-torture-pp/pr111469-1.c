# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111469-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111469-1.c" 2


long f;
char *g;
__attribute__((noinline))
char o() {
  char l;
  while (f)
    ;
  l = *g;
  return l;
}






__attribute__((noinline))
unsigned short gg(unsigned short a, unsigned short b)
{
  short d;
  if (a > b)
  {
    d= b;
  }
  else
  {
    o();
    d = a;
  }
  return d;
}

int main(void)
{
  gg(3, 2);
}
