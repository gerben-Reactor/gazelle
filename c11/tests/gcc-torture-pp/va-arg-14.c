# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-14.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-14.c" 2
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
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-14.c" 2

void abort (void);
void exit (int);

va_list global;

void vat(va_list param, ...)
{
  va_list local;

  __builtin_va_start(local, param);
  __builtin_va_copy(global, local);
  __builtin_va_copy(param, local);
  if (__builtin_va_arg(local, int) != 1)
    abort();
  __builtin_va_end(local);
  if (__builtin_va_arg(global, int) != 1)
    abort();
  __builtin_va_end(global);
  if (__builtin_va_arg(param, int) != 1)
    abort();
  __builtin_va_end(param);

  __builtin_va_start(param, param);
  __builtin_va_start(global, param);
  __builtin_va_copy(local, param);
  if (__builtin_va_arg(local, int) != 1)
    abort();
  __builtin_va_end(local);
  __builtin_va_copy(local, global);
  if (__builtin_va_arg(local, int) != 1)
    abort();
  __builtin_va_end(local);
  if (__builtin_va_arg(global, int) != 1)
    abort();
  __builtin_va_end(global);
  if (__builtin_va_arg(param, int) != 1)
    abort();
  __builtin_va_end(param);
}

int main(void)
{
  va_list t;
  vat (t, 1);
  exit (0);
}
