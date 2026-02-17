# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070517-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070517-1.c" 2



extern void abort (void);

static int get_kind(int) __attribute__ ((noinline));

static int get_kind(int v)
{
  volatile int k = v;
  return k;
}

static int some_call(void) __attribute__ ((noinline));

static int some_call(void)
{
  return 0;
}

static void example (int arg)
{
  int tmp, kind = get_kind (arg);

  if (kind == 9 || kind == 10 || kind == 5)
    {
      if (some_call() == 0)
        {
          if (kind == 9 || kind == 10)
            tmp = arg;
          else
            abort();
        }
    }
}

int main(void)
{
  example(10);
  return 0;
}
