# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/mul-sext.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/mul-sext.c" 2


typedef long int int64_t;
typedef int int32_t;


__attribute__((noipa, noinline)) int64_t
f(int64_t a, int64_t b)
{
  return (int64_t)(int32_t)a * (int64_t)(int32_t)b;
}

int
main()
{
  int64_t a = 0x1145140000000001;
  int64_t b = 0x1919810000000001;
  if (f(a, b) != 1)
    __builtin_abort();
}
