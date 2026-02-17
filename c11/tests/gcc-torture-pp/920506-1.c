# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920506-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920506-1.c" 2
void abort (void);
void exit (int);
int l[]={0,1};
int main(void){int*p=l;switch(*p++){case 0:exit(0);case 1:break;case 2:break;case 3:case 4:break;}abort();}
