# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110165-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110165-1.c" 2
struct s
{
  int t : 1;
};

int f(struct s t, int a, int b) __attribute__((noinline));
int f(struct s t, int a, int b)
{
        int bd = t.t;
        if (bd) a|=b;
        return a;
}

int main(void)
{
        struct s t;
        for(int i = -1;i <= 1; i++)
        {
                int a = 0x10;
                int b = 0x0f;
                int c = a | b;
  struct s t = {i};
                int r = f(t, a, b);
                int exp = (i != 0) ? a | b : a;
                if (exp != r)
                 __builtin_abort();
        }
}
