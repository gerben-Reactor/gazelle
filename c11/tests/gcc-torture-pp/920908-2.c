# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920908-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920908-2.c" 2
# 15 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920908-2.c"
struct T
{
unsigned i:8;
unsigned c:24;
};
f(struct T t)
{
struct T s[1];
s[0]=t;
return(char)s->c;
}
main()
{
struct T t;
t.i=0xff;
t.c=0xffff11;
if(f(t)!=0x11)abort();
exit(0);
}
