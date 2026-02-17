# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-5.c" 2
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
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/va-arg-5.c" 2

void abort (void);
void exit (int);

void
va_double (int n, ...)
{
  va_list args;

  __builtin_va_start(args, n);

  if (__builtin_va_arg(args, double) != 3.141592)
    abort ();
  if (__builtin_va_arg(args, double) != 2.71827)
    abort ();
  if (__builtin_va_arg(args, double) != 2.2360679)
    abort ();
  if (__builtin_va_arg(args, double) != 2.1474836)
    abort ();

  __builtin_va_end(args);
}

void
va_long_double (int n, ...)
{
  va_list args;

  __builtin_va_start(args, n);

  if (__builtin_va_arg(args, long double) != 3.141592L)
    abort ();
  if (__builtin_va_arg(args, long double) != 2.71827L)
    abort ();
  if (__builtin_va_arg(args, long double) != 2.2360679L)
    abort ();
  if (__builtin_va_arg(args, long double) != 2.1474836L)
    abort ();

  __builtin_va_end(args);
}

int
main (void)
{
  va_double (4, 3.141592, 2.71827, 2.2360679, 2.1474836);
  va_long_double (4, 3.141592L, 2.71827L, 2.2360679L, 2.1474836L);
  exit (0);
}
