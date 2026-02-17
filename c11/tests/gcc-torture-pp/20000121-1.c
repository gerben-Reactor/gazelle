# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000121-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000121-1.c" 2
void big(long long u) { }

void doit(unsigned int a,unsigned int b,char *id)
{
  big(*id);
  big(a);
  big(b);
}

int main(void)
{
  doit(1,1,"\n");
  return 0;
}
