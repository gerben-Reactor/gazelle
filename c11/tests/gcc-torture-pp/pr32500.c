# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr32500.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr32500.c" 2
extern void abort(void);
extern void exit(int);
void foo(int) __attribute__((noinline));
void bar(void) __attribute__((noinline));


int x;
void foo(int i) { x = i; }
void bar(void) { exit(0); }

int
main(int argc, char *argv[])
{
 int i;
 int numbers[4] = { 0xdead, 0xbeef, 0x1337, 0x4242 };

 for (i = 1; i <= 12; i++) {
  if (i <= 4)
   foo(numbers[i-1]);
  else if (i >= 7 && i <= 9)
   bar();
 }

 abort();
}
