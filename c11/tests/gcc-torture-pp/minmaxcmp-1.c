# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/minmaxcmp-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/minmaxcmp-1.c" 2
# 36 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/minmaxcmp-1.c"
_Bool min_lt_ (int a, int b) { int x = ((a) < (b) ? (a) : (b)); return ((x) < (a)); } _Bool min_lt_volatile (int a, int b) { volatile int x = ((a) < (b) ? (a) : (b)); return ((x) < (a)); } _Bool max_lt_ (int a, int b) { int x = ((a) > (b) ? (a) : (b)); return ((x) < (a)); } _Bool max_lt_volatile (int a, int b) { volatile int x = ((a) > (b) ? (a) : (b)); return ((x) < (a)); } _Bool min_gt_ (int a, int b) { int x = ((a) < (b) ? (a) : (b)); return ((x) > (a)); } _Bool min_gt_volatile (int a, int b) { volatile int x = ((a) < (b) ? (a) : (b)); return ((x) > (a)); } _Bool max_gt_ (int a, int b) { int x = ((a) > (b) ? (a) : (b)); return ((x) > (a)); } _Bool max_gt_volatile (int a, int b) { volatile int x = ((a) > (b) ? (a) : (b)); return ((x) > (a)); } _Bool min_le_ (int a, int b) { int x = ((a) < (b) ? (a) : (b)); return ((x) <= (a)); } _Bool min_le_volatile (int a, int b) { volatile int x = ((a) < (b) ? (a) : (b)); return ((x) <= (a)); } _Bool max_le_ (int a, int b) { int x = ((a) > (b) ? (a) : (b)); return ((x) <= (a)); } _Bool max_le_volatile (int a, int b) { volatile int x = ((a) > (b) ? (a) : (b)); return ((x) <= (a)); } _Bool min_ge_ (int a, int b) { int x = ((a) < (b) ? (a) : (b)); return ((x) >= (a)); } _Bool min_ge_volatile (int a, int b) { volatile int x = ((a) < (b) ? (a) : (b)); return ((x) >= (a)); } _Bool max_ge_ (int a, int b) { int x = ((a) > (b) ? (a) : (b)); return ((x) >= (a)); } _Bool max_ge_volatile (int a, int b) { volatile int x = ((a) > (b) ? (a) : (b)); return ((x) >= (a)); } _Bool min_ne_ (int a, int b) { int x = ((a) < (b) ? (a) : (b)); return ((x) != (a)); } _Bool min_ne_volatile (int a, int b) { volatile int x = ((a) < (b) ? (a) : (b)); return ((x) != (a)); } _Bool max_ne_ (int a, int b) { int x = ((a) > (b) ? (a) : (b)); return ((x) != (a)); } _Bool max_ne_volatile (int a, int b) { volatile int x = ((a) > (b) ? (a) : (b)); return ((x) != (a)); } _Bool min_eq_ (int a, int b) { int x = ((a) < (b) ? (a) : (b)); return ((x) == (a)); } _Bool min_eq_volatile (int a, int b) { volatile int x = ((a) < (b) ? (a) : (b)); return ((x) == (a)); } _Bool max_eq_ (int a, int b) { int x = ((a) > (b) ? (a) : (b)); return ((x) == (a)); } _Bool max_eq_volatile (int a, int b) { volatile int x = ((a) > (b) ? (a) : (b)); return ((x) == (a)); }







int main()
{
  for(int x = -10; x < 10; x++)
    for(int y = -10; y < 10; y++)
    {
        do { if (min_lt_(x,y) != min_lt_volatile(x,y)) __builtin_abort(); } while(0); do { if (max_lt_(x,y) != max_lt_volatile(x,y)) __builtin_abort(); } while(0); do { if (min_gt_(x,y) != min_gt_volatile(x,y)) __builtin_abort(); } while(0); do { if (max_gt_(x,y) != max_gt_volatile(x,y)) __builtin_abort(); } while(0); do { if (min_le_(x,y) != min_le_volatile(x,y)) __builtin_abort(); } while(0); do { if (max_le_(x,y) != max_le_volatile(x,y)) __builtin_abort(); } while(0); do { if (min_ge_(x,y) != min_ge_volatile(x,y)) __builtin_abort(); } while(0); do { if (max_ge_(x,y) != max_ge_volatile(x,y)) __builtin_abort(); } while(0); do { if (min_ne_(x,y) != min_ne_volatile(x,y)) __builtin_abort(); } while(0); do { if (max_ne_(x,y) != max_ne_volatile(x,y)) __builtin_abort(); } while(0); do { if (min_eq_(x,y) != min_eq_volatile(x,y)) __builtin_abort(); } while(0); do { if (max_eq_(x,y) != max_eq_volatile(x,y)) __builtin_abort(); } while(0);
    }
}
