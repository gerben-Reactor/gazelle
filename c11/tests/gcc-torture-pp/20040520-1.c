# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040520-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040520-1.c" 2


void abort ();
int main () {
        int foo;
        int bar (void)
        {
                int baz = 0;
                if (foo!=45)
                        baz = foo;
                return baz;
        }
        foo = 1;
        if (!bar ())
                abort ();
        return 0;
}
