# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr33631.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr33631.c" 2
typedef union
{
  int __lock;
} pthread_mutex_t;

extern void abort (void);

int main()
{
    struct { int c; pthread_mutex_t m; } r = { .m = 0 };
    if (r.c != 0)
      abort ();
    return 0;
}
