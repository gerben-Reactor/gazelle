# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111431-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111431-1.c" 2
int
foo (int a)
{
  int b = a == 0;
  return (a & b);
}
# 25 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111431-1.c"
__attribute__((noipa)) _Bool func_0_( int a) { int b = a == 0; return (a & b); } __attribute__((noipa)) _Bool func_0_volatile(volatile int a) { volatile int b = a == 0; return (a & b); } __attribute__((noipa)) _Bool func_1_( int a) { int b = a == 1; return (a & b); } __attribute__((noipa)) _Bool func_1_volatile(volatile int a) { volatile int b = a == 1; return (a & b); } __attribute__((noipa)) _Bool func_5_( int a) { int b = a == 5; return (a & b); } __attribute__((noipa)) _Bool func_5_volatile(volatile int a) { volatile int b = a == 5; return (a & b); }






int main(void)
{
  for(int a = -10; a <= 10; a++)
   {
     do { if(func_0_(a) != func_0_volatile(a)) __builtin_abort(); } while(0); do { if(func_1_(a) != func_1_volatile(a)) __builtin_abort(); } while(0); do { if(func_5_(a) != func_5_volatile(a)) __builtin_abort(); } while(0);
   }
}
