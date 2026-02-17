# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040625-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040625-1.c" 2

void abort (void);
void exit (int);

struct __attribute__ ((packed)) s { struct s *next; };

struct s * __attribute__ ((noinline))
maybe_next (struct s *s, int t)
{
  if (t)
    s = s->next;
  return s;
}

int main ()
{
  struct s s1, s2;

  s1.next = &s2;
  if (maybe_next (&s1, 1) != &s2)
    abort ();
  exit (0);
}
