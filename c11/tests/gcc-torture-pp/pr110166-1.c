# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110166-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110166-1.c" 2
struct s
{
  int t : 1;
  int t1 : 1;
};

int f(struct s t) __attribute__((noinline));
int f(struct s t)
{
   int c = t.t;
   int d = t.t1;
   if (c > d)
     t.t = d;
   else
     t.t = c;
  return t.t;
}

int main(void)
{
        struct s t;
        for(int i = -1;i <= 0; i++)
        {
          for(int j = -1;j <= 0; j++)
          {
  struct s t = {i, j};
                int r = f(t);
                int exp = i < j ? i : j;
                if (exp != r)
                 __builtin_abort();
          }
        }
}
