# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920625-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920625-1.c" 2
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
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920625-1.c" 2

void abort (void);
void exit (int);

typedef struct{double x,y;}point;
point pts[]={{1.0,2.0},{3.0,4.0},{5.0,6.0},{7.0,8.0}};
static int va1(int nargs,...)
{
  va_list args;
  int i;
  point pi;
  __builtin_va_start(args, nargs);
  for(i=0;i<nargs;i++){
    pi=__builtin_va_arg(args, point);
    if(pts[i].x!=pi.x||pts[i].y!=pi.y)abort();
  }
  __builtin_va_end(args);
}

typedef struct{int x,y;}ipoint;
ipoint ipts[]={{1,2},{3,4},{5,6},{7,8}};
static int va2(int nargs,...)
{
  va_list args;
  int i;
  ipoint pi;
  __builtin_va_start(args, nargs);
  for(i=0;i<nargs;i++){
    pi=__builtin_va_arg(args, ipoint);
    if(ipts[i].x!=pi.x||ipts[i].y!=pi.y)abort();
  }
  __builtin_va_end(args);
}

int
main(void)
{
va1(4,pts[0],pts[1],pts[2],pts[3]);
va2(4,ipts[0],ipts[1],ipts[2],ipts[3]);
exit(0);
}
