# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/frame-address.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/frame-address.c" 2

void abort (void);

int check_fa_work (const char *, const char *) __attribute__((noinline,noipa));
int check_fa_mid (const char *) __attribute__((noinline,noipa));
int check_fa (char *) __attribute__((noinline,noipa));
int how_much (void) __attribute__((noinline,noipa));

int check_fa_work (const char *c, const char *f)
{
  const char d = 0;

  if (c >= &d)
    return c >= f && f >= &d;
  else
    return c <= f && f <= &d;
}

int check_fa_mid (const char *c)
{
  const char *f = __builtin_frame_address (0);


  return check_fa_work (c, f) != 0;
}

int check_fa (char *unused)
{
  const char c = 0;


  return check_fa_mid (&c) != 0;
}

int how_much (void)
{
 return 8;
}

int main (void)
{
  char *unused = __builtin_alloca (how_much ());

  if (!check_fa(unused))
    abort();
  return 0;
}
