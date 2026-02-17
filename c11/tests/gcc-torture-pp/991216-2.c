# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991216-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991216-2.c" 2
# 1 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 1 3
# 55 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 3
# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg___gnuc_va_list.h" 1 3
# 12 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg___gnuc_va_list.h" 3
typedef __builtin_va_list __gnuc_va_list;
# 56 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_list.h" 1 3
# 12 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_list.h" 3
typedef __builtin_va_list va_list;
# 61 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_arg.h" 1 3
# 66 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg___va_copy.h" 1 3
# 71 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3




# 1 "/usr/lib/llvm-18/lib/clang/18/include/__stdarg_va_copy.h" 1 3
# 76 "/usr/lib/llvm-18/lib/clang/18/include/stdarg.h" 2 3
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991216-2.c" 2

void abort (void);
void exit (int);




void
test (int n, ...)
{
  va_list ap;
  int i;

  __builtin_va_start(ap, n);
  for (i = 2; i <= n; i++)
    {
      if (__builtin_va_arg(ap, int) != i)
 abort ();
    }

  if (__builtin_va_arg(ap, long long) != 0x123456789abcdefLL)
    abort ();

  if (__builtin_va_arg(ap, int) != 0x55)
    abort ();

  __builtin_va_end(ap);
}

int
main ()
{
  test (1, 0x123456789abcdefLL, 0x55);
  test (2, 2, 0x123456789abcdefLL, 0x55);
  test (3, 2, 3, 0x123456789abcdefLL, 0x55);
  test (4, 2, 3, 4, 0x123456789abcdefLL, 0x55);
  test (5, 2, 3, 4, 5, 0x123456789abcdefLL, 0x55);
  test (6, 2, 3, 4, 5, 6, 0x123456789abcdefLL, 0x55);
  test (7, 2, 3, 4, 5, 6, 7, 0x123456789abcdefLL, 0x55);
  test (8, 2, 3, 4, 5, 6, 7, 8, 0x123456789abcdefLL, 0x55);
  exit (0);
}
