# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920501-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920501-1.c" 2


int s[2];
x(){if(!s[0]){s[1+s[1]]=s[1];return 1;}}
main(){s[0]=s[1]=0;if(x(0)!=1)abort();exit(0);}
