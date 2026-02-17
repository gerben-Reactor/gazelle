# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-11.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-11.c" 2


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
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-11.c" 2

void abort (void);
void exit (int);

static int
foo (int a, ...)
{
  va_list va;
  int i, res;

  __builtin_va_start(va, a);

  for (i = 0; i < 4; ++i)
    (void) __builtin_va_arg(va, int);

  res = __builtin_va_arg(va, int);

  __builtin_va_end(va);

  return res;
}

int
main (void)
{
  if (foo (5, 4, 3, 2, 1, 0))
    abort ();
  exit (0);
}
