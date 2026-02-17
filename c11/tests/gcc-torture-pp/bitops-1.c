# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitops-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitops-1.c" 2
# 17 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitops-1.c"
__attribute__((noinline,noclone)) int h0( int A, int B){ return (A | B) & (A == B); } __attribute__((noinline,noclone)) int i0( int A, int B){ return A | (A == B); } __attribute__((noinline,noclone)) int k0( int A, int B){ return (A & B) | (A == B); }
__attribute__((noinline,noclone)) int h1(volatile int A, volatile int B){ return (A | B) & (A == B); } __attribute__((noinline,noclone)) int i1(volatile int A, volatile int B){ return A | (A == B); } __attribute__((noinline,noclone)) int k1(volatile int A, volatile int B){ return (A & B) | (A == B); }

int values[] = { 0, 1, 2, 3, -1, -2, -3, 0x10080 };
int numvalues = sizeof(values)/sizeof(values[0]);

int main(){
    for(int A = 0; A < numvalues; A++)
      for(int B = 0; B < numvalues; B++)
 {
   int a = values[A];
   int b = values[B];
   if (h0 (a, b) != h1 (a, b)) __builtin_abort();
   if (i0 (a, b) != i1 (a, b)) __builtin_abort();
   if (k0 (a, b) != k1 (a, b)) __builtin_abort();
 }
}
