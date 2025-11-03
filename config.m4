if test "$PHP_LZ4FRAME" != "no"; then
  dnl Check cargo command exists or not.
  AC_PATH_PROG(CARGO, cargo, no)
  if ! test -x "$CARGO"; then
    AC_MSG_ERROR([cargo command missing, please reinstall the cargo distribution])
  fi

  AC_DEFINE([HAVE_DIRS], [1], [ Define to 1 if the PHP extension 'dirs' is available. ])

  PHP_NEW_EXTENSION(lz4frame)
  PHP_ADD_MAKEFILE_FRAGMENT()

  PHP_SUBST([PHP_CONFIG])
fi
