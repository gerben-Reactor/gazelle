# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980608-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980608-1.c" 2


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
# 4 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980608-1.c" 2

extern void abort(void);
extern void exit (int);

void f1(int a,int b,int c,int d,int e, int f,int g,int h,int i,int j, int k,int
l,int m,int n,int o)
{
    return;
}

inline void debug(const char *msg,...)
{
    va_list ap;
    __builtin_va_start(ap, msg);

    f1(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);

    if ( __builtin_va_arg(ap, int) != 101)
        abort();
    if ( __builtin_va_arg(ap, int) != 102)
        abort();
    if ( __builtin_va_arg(ap, int) != 103)
        abort();
    if ( __builtin_va_arg(ap, int) != 104)
        abort();
    if ( __builtin_va_arg(ap, int) != 105)
        abort();
    if ( __builtin_va_arg(ap, int) != 106)
        abort();

    __builtin_va_end(ap);
}

int main(void)
{
  debug("%d %d %d  %d %d %d\n", 101, 102, 103, 104, 105, 106);
  exit(0);
}
