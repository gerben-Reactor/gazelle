# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921013-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921013-1.c" 2

f(d,x,y,n)
int*d;
float*x,*y;
int n;
{
  while(n--){*d++=*x++==*y++;}
}

main()
{
  int r[4];
  float a[]={5,1,3,5};
  float b[]={2,4,3,0};
  int i;
  f(r,a,b,4);
  for(i=0;i<4;i++)
    if((a[i]==b[i])!=r[i])
      abort();
  exit(0);
}
