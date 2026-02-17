# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920428-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920428-2.c" 2




s(i){if(i>0){__label__ l1;int f(int i){if(i==2)goto l1;return 0;}return f(i);l1:;}return 1;}
x(){return s(0)==1&&s(1)==0&&s(2)==1;}
main(){if(x()!=1)abort();exit(0);}
