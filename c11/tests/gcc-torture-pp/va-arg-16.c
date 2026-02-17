# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-16.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-16.c" 2
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
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-16.c" 2

void abort (void);
void exit (int);

typedef double TYPE;

void vafunction (TYPE dummy1, TYPE dummy2, ...)
{
  va_list ap;

  __builtin_va_start(ap, dummy2);
  if (dummy1 != 888.)
    abort();
  if (dummy2 != 999.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 1.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 2.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 3.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 4.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 5.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 6.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 7.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 8.)
    abort();
  if (__builtin_va_arg(ap, TYPE) != 9.)
    abort();
  __builtin_va_end(ap);
}


int main (void)
{
  vafunction( 888., 999., 1., 2., 3., 4., 5., 6., 7., 8., 9. );
  exit(0);
  return 0;
}
