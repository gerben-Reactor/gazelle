# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr29006.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr29006.c" 2
struct __attribute__((__packed__)) s { char c; unsigned long long x; };
void __attribute__((__noinline__)) foo (struct s *s) { s->x = 0; }
int main (void) { struct s s = { 1, ~0ULL }; foo (&s); return s.x != 0; }
