# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920429-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920429-1.c" 2

typedef unsigned char t;int i,j;
t*f(t*p){t c;c=*p++;i=((c&2)?1:0);j=(c&7)+1;return p;}
main(){t*p0="ab",*p1;p1=f(p0);if(p0+1!=p1)abort();exit(0);}
