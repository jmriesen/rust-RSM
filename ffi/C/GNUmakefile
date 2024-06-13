#
# Package: Reference Standard M
# File:    rsm/GNUmakefile
# Summary: Makefile for Linux, macOS, Solaris, AIX, HP-UX, and RPi
#          See rsm/Makefile for FreeBSD, NetBSD, and OpenBSD
#
# David Wicksell <dlw@linux.com>
# Copyright © 2020-2024 Fourth Watch Software LC
# https://gitlab.com/Reference-Standard-M/rsm
#
# Based on MUMPS V1 by Raymond Douglas Newman
# Copyright © 1999-2018
# https://gitlab.com/Reference-Standard-M/mumpsv1
#
# This program is free software: you can redistribute it and/or modify it
# under the terms of the GNU Affero General Public License (AGPL) as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
# or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
# License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see https://www.gnu.org/licenses/.
#
# SPDX-FileCopyrightText:  © 2020 David Wicksell <dlw@linux.com>
# SPDX-License-Identifier: AGPL-3.0-or-later

OS      := $(shell uname)
CC      := gcc
PROG    := rsm
RM      := rm -f
DEPS    := $(wildcard include/*.h)
SRC     := $(wildcard */*.c)
OBJ     := $(SRC:.c=.o)
CFLAGS  := -Wall -Wextra -fsigned-char -fwrapv -std=gnu99 -Iinclude -D_FILE_OFFSET_BITS=64
PREFIX  := /usr/local
GIT_SHA := $(shell git rev-parse --short=10 HEAD 2>/dev/null; true)
DOCS    := $(wildcard doc/adoc/*.adoc)

ifdef GIT_SHA
    CFLAGS  += -DGIT_SHA=$(GIT_SHA)
endif

ifdef dbver
    CFLAGS  += -DRSM_DBVER=$(dbver)
endif

ifeq ($(OS),HP-UX)
    LDLIBS  := -lm
else
    LDLIBS  := -lcrypt
endif

ifneq ($(OS),AIX)
    ifneq ($(OS),HP-UX)
        LDLIBS  += -lm
    endif

    INSTLPR := install -o 0 -g 0 -m 755 -s ${PROG} ${PREFIX}/bin
    INSTLUT := install -D -o 0 -g 0 -m 644 utils.rsm -t ${PREFIX}/share/rsm
    INSTLDC := install -D -o 0 -g 0 -m 644 ${DOCS} -t ${PREFIX}/share/doc/rsm
    INSTLMN := install -D -o 0 -g 0 -m 644 doc/man/rsm.1 -t ${PREFIX}/share/man/man1
else
    INSTLPR := install -O 0 -G 0 -M 755 -S -f ${PREFIX}/bin ${PROG}
    INSTLUT := install -O 0 -G 0 -M 644 -f ${PREFIX}/share/rsm utils.rsm
    INSTLDC := install -O 0 -G 0 -M 644 -f ${PREFIX}/share/doc/rsm ${DOCS}
    INSTLMN := install -O 0 -G 0 -M 644 -f ${PREFIX}/share/man/man1 doc/man/rsm.1
endif

ifeq ($(OS),SunOS)
    LDLIBS  += -lnsl -lsocket -lrt
endif

ifeq ($(OS),Darwin)
    CFLAGS  += -Wno-deprecated-declarations
    LDLIBS  := -framework CoreServices -framework DirectoryService -framework Security -lm
endif

ifeq ($(MAKECMDGOALS),debug)
    CONFIG  := -O0 -g3

    ifdef options
        ifeq ($(options),profile)
            CFLAGS  += -pg
            LDLIBS  += -lc -pg
        else
            ifeq ($(options),sanitize)
                CFLAGS  += -fsanitize=address,undefined
                LDLIBS  += -fsanitize=address,undefined
            endif
        endif
    endif
else
    CONFIG  := -O3
    CFLAGS  += -DNDEBUG
endif

.PHONY: all
all: ${PROG}

.PHONY: debug
debug: ${PROG}

${PROG}: ${OBJ}
	${CC} -o ${PROG} $^ ${LDLIBS}

%.o: %.c ${DEPS}
	${CC} ${CONFIG} ${CFLAGS} -o $@ -c $<

<<<<<<< HEAD:ffi/C/GNUmakefile
all: ${OBJ}
	${CC} -o ${PROG} $^ ${LDFLAGS}

lib: ${OBJ}
	ar rcs libmylib.a ${OBJ}

debug: ${OBJ}
	${CC} -o ${PROG} $^ ${LDFLAGS}

install: ${PROG}

	@if [ "$(USER)" != "root" ]; then \
	    echo "You must install ${PROG} as root"; \
	    exit 1; \
	fi

ifeq ($(OS),AIX)
	@if [ -d ${EXECDIR} ]; then \
	    echo install -O root -G 0 -M 755 -S -f ${EXECDIR} ${PROG}; \
	    install -O root -G 0 -M 755 -S -f ${EXECDIR} ${PROG}; \
	else \
	    echo "${EXECDIR} does not exist"; \
	    exit 1; \
	fi
else
	@if [ -d ${EXECDIR} ]; then \
	    echo install -o root -g 0 -m 755 -s ${PROG} ${EXECDIR}; \
	    install -o root -g 0 -m 755 -s ${PROG} ${EXECDIR}; \
	else \
	    echo "${EXECDIR} does not exist"; \
	    exit 1; \
	fi
endif

uninstall:

	@if [ "$(USER)" != "root" ]; then \
	    echo "You must uninstall ${PROG} as root"; \
	    exit 1; \
	fi

	@if [ -f ${EXECDIR}/${PROG} -a -x ${EXECDIR}/${PROG} ]; then \
	    echo ${RM} ${EXECDIR}/${PROG}; \
	    ${RM} ${EXECDIR}/${PROG}; \
	fi

.PHONY: uninstall-docs
uninstall-docs:
	@if [ "$(USER)" != "root" ]; then \
	    echo "You must uninstall documentation as root"; \
	    exit 1; \
	elif [ -d ${PREFIX}/share/doc/rsm ]; then \
	    echo ${RM} -r ${PREFIX}/share/doc/rsm; \
	    ${RM} -r ${PREFIX}/share/doc/rsm; \
	    if [ -f ${PREFIX}/share/man/man1/rsm.1.gz ]; then \
	        echo ${RM} ${PREFIX}/share/man/man1/rsm.1.gz; \
	        ${RM} ${PREFIX}/share/man/man1/rsm.1.gz; \
	        echo mandb -q; \
	        mandb -q; \
	    fi \
	fi
