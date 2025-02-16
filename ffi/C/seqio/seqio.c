/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 *
 *
 * Extended Summary:
 *
 * This module can be viewed as the "main" program for all matters relating
 * to sequential Input/Output (IO). It provides the following functions:
 *
 *     SQ_Init        - Initializes channel 0, signal handlers...
 *     SQ_Open        - Obtain ownership of an object
 *     SQ_Use         - Make an owned object current for IO
 *     SQ_Close       - Relinquish ownership of an object
 *     SQ_Write       - Output one or more bytes to the current IO object
 *     SQ_WriteStar   - Output one byte to the current IO object
 *     SQ_WriteFormat - Output a format byte(s) to the current IO object
 *     SQ_Read        - Read a byte(s) from the current IO object
 *     SQ_ReadStar    - Reads one byte from the current IO object
 *     SQ_Flush       - Flushes input buffered for the current IO object
 *     SQ_Device      - Gathers information about the current IO object
 *     SQ_Force       - Forces data to an object
 *
 * NOTE: An object is one of a file, device, named pipe or TCP/IP socket
 */

#include <errno.h>
#include <sys/file.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <sys/ioctl.h>                                                          // for ioctl
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <ctype.h>
#include <netdb.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <termios.h>
#include <unistd.h>
#include "rsm.h"
#include "error.h"
#include "proto.h"
#include "seqio.h"

// Set bit flags
#define UNSET -1                                                                // Unset bit
#define LEAVE 0                                                                 // Leave bit
#define SET   1                                                                 // Set bit

// Object types
#define UNKNOWN 0                                                               // Unknown object
#define DIR     1                                                               // Directory
#define CHR     2                                                               // Character special
#define BLK     3                                                               // Block special
#define REG     4                                                               // Regular
#define FIFO    5                                                               // Named pipe (i.e., FIFO)
#define LNK     6                                                               // Symbolic link
#define SOCK    7                                                               // Socket
#define WHT     8                                                               // Whiteout

// Miscellaneous buffer sizes
#define MASKSIZE  64                                                            // Maximum bits in set bit mask
#define WRITESIZE 132                                                           // Maximum write buffer size
#define BUFSIZE   MAX_STR_LEN                                                   // Maximum buffer size
#define READSIZE  1024                                                          // Maximum read buffer size
#define OPSIZE    30                                                            // Maximum operation size
#define TTYSIZE   128                                                           // Maximim tty name size
#define ERRSIZE   100                                                           // Maximum error buffer size

// SQ_Chan.options bit mask
#define INTERM    0                                                             // Input terminator(s) bit (ALL)
#define OUTERM    1                                                             // Output terminator bit (ALL)

#define ESC       2                                                             // Escape sequence bit (SQ_TERM)
#define TTYECHO   3                                                             // Echo bit (SQ_TERM)
#define DEL8      4                                                             // Delete key 8 bit (SQ_TERM)
#define DEL127    5                                                             // Delete key 127 bit (SQ_TERM)
#define TYPEAHEAD 6                                                             // Type-ahead bit (SQ_TERM)

#define IPV6      2                                                             // Socket IPv6 bit (SQ_SOCK)
#define UDP       3                                                             // Socket UDP bit (SQ_SOCK)

// Miscellaneous
#define STDCHAN 0                                                               // stdin, stdout and stderr

// The following required for linux
#ifdef linux
#   include <sys/ttydefaults.h>
#endif

#ifndef S_ISWHT
#   define S_ISWHT(m) (((m) & 0170000) == 0160000)                              // whiteout
#endif

static u_int64 MASK[MASKSIZE];                                                  // Set bit mask
static u_int64 CRLF;                                                            // CRLF

short proto_family = PF_INET;                                                   // PF_INET or PF_INET6
short addr_family  = AF_INET;                                                   // AF_INET or AF_INET6
short sock_type    = SOCK_STREAM;                                               // SOCK_STREAM or SOCK_DGRAM
short sock_proto   = IPPROTO_TCP;                                               // IPPROTO_TCP or IPPROTO_UDP

extern char    history[MAX_HISTORY][MAX_STR_LEN];                               // history buffer
extern u_short hist_next;                                                       // next history pointer
extern u_short hist_curr;                                                       // history entry pointer
extern short   in_hist;                                                         // are we in the history buffer
extern u_short prompt_len;                                                      // length of the current direct mode prompt

// Common functions

/*
 * This function checks the character array "bytestr" for the following:
 *   - "bytestr" is NULL;
 *
 * If any of the forementioned statements are true, this function returns a
 * negative integer value to indicate the error. Otherwise, a 0 is returned.
 */
int checkBytestr(const char *bytestr)
{
    if (bytestr == NULL) return getError(INT, ERRZ28);
    return 0;
}

/*
 * This function checks the integer "nbytes" for the following:
 *   - "nbytes" < 0; and
 *   - "nbytes" > BUFSIZE.
 *
 * If any of the forementioned statements are true, this function returns a
 * negative integer value to indicate the error. Otherwise, a 0 is returned.
 */
int checkNbytes(int nbytes)
{
    if ((nbytes < 0) || (nbytes > BUFSIZE)) return getError(INT, ERRZ33);
    return 0;
}

/*
 * This functions checks the cstring "cstr" for the following:
 *   - "cstr" is NULL;
 *   - "cstr->buf" is invalid (refer to checkBytestr); and
 *   - "cstr->len" is invalid (refer to checkNbytes).
 *
 * If any of the forementioned statements are true, this function returns a
 * negative integer value to indicate the error. Otherwise, a 0 is returned.
 */
int checkCstring(const cstring *cstr)
{
    int ret;

    if (cstr == NULL) return getError(INT, ERRZ28);
    ret = checkBytestr((char *) cstr->buf);
    if (ret < 0) return ret;
    ret = checkNbytes(cstr->len);
    if (ret < 0) return ret;
    return 0;
}

/*
 * This function determines the type of an object "object". If the object can
 * be successfully identified, a postive integer is returned (which indicates
 * the objects type). If the object can not be identified, 0 is returned.
 * Otherwise, a negative integer value is returned to indicate the error that
 * has occurred.
 *
 * NOTE: If stat fails because the object does not exist, then it is possible
 *       that the object is a file still to be created; hence, the object could
 *       be a file, but this function is unable to confirm this as the file is
 *       not yet created. Thus, a 0 is returned as opposed to a negative integer
 *       value
 */
int getObjectType(char *object)
{
    struct stat sb;                                                             // Object status
    int         ret;                                                            // Useful variable

    ret = stat(object, &sb);

    if (ret == -1) {
        if (errno == ENOENT) {
            return UNKNOWN;                                                     // File to be created ??
        } else {
            return getError(SYS, errno);
        }
    }

    if (S_ISDIR(sb.st_mode) == 1) {
        return DIR;
    } else if (S_ISCHR(sb.st_mode) == 1) {
        return CHR;
    } else if (S_ISBLK(sb.st_mode) == 1) {
        return BLK;
    } else if (S_ISREG(sb.st_mode) == 1) {
        return REG;
    } else if (S_ISFIFO(sb.st_mode) == 1) {
        return FIFO;
    } else if (S_ISLNK(sb.st_mode) == 1) {
        return LNK;
    } else if (S_ISSOCK(sb.st_mode) == 1) {
        return SOCK;
    } else if (S_ISWHT(sb.st_mode) == 1) {
        return WHT;
    } else {
        return getError(INT, ERRZ30);
    }
}

int getObjectMode(int fd)
{
    struct stat sb;
    int    ret;

    ret = fstat(fd, &sb);
    if (ret == -1) return getError(SYS, errno);

    if (S_ISDIR(sb.st_mode) == 1) {
        return DIR;
    } else if (S_ISCHR(sb.st_mode) == 1) {
        return CHR;
    } else if (S_ISBLK(sb.st_mode) == 1) {
        return BLK;
    } else if (S_ISREG(sb.st_mode) == 1) {
        return REG;
    } else if (S_ISFIFO(sb.st_mode) == 1) {
        return FIFO;
    } else if (S_ISLNK(sb.st_mode) == 1) {
        return LNK;
    } else if (S_ISSOCK(sb.st_mode) == 1) {
        return SOCK;
    } else if (S_ISWHT(sb.st_mode) == 1) {
        return WHT;
    } else {
        return getError(INT, ERRZ30);
    }
}

int getModeCategory(int mode)
{
    if (mode == REG) return SQ_FILE;
    if (mode == SOCK) return SQ_SOCK;
    if (mode == FIFO) return SQ_PIPE;
    if (mode == CHR) return SQ_TERM;
    return getError(INT, ERRZ30);
}

/*
 * This function checks if "chan" is a valid channel. If "chan" is not valid,
 * then 0 is returned. Otherwise, 1 is returned.
 */
int isChan(int chan)
{
    if ((chan < MIN_SEQ_IO) || (chan >= MAX_SEQ_IO)) return 0;
    return 1;
}

/*
 * This function checks if "type" is a valid type for an object. If "type" is
 * not valid, then 0 is returned. Otherwise, 1 is returned.
 */
/*
int isType(int type)
{
    if ((type != SQ_FREE) && (type != SQ_FILE) && (type != SQ_SOCK) && (type != SQ_PIPE) && (type != SQ_TERM)) {
        return 0;
    }

    return 1;
}
*/

/*
 * This function checks if channel "chan" is free. If "chan" is not free, then
 * 0 is returned. Otherwise, 1 is returned.
 */
int isChanFree(int chan)
{
    if ((int) partab.jobtab->seqio[chan].type != SQ_FREE) return 0;
    return 1;
}

/*
 * This function checks the cstring "cstr" for the following:
 *
 *   - "cstr" is invalid (refer to checkCstring); and
 *   - one or more characters in the character array "cstr->buf" have an
 *     integer value not between 0 and 127 inclusive (ASCII character set)
 *
 * If any of the forementioned statements are true, this function returns a
 * negative integer value to indicate the error. Otherwise, a 0 is returned.
 */
int checkAsciiChars(const cstring *cstr)
{
    int index;
    int ret;

    ret = checkCstring(cstr);
    if (ret < 0) return ret;

    for (index = 0; index < cstr->len; index++) {
        if ((u_char) cstr->buf[index] > 127) {
            return getError(INT, ERRZ34);
        }
    }

    return 0;
}

/*
 * This function returns an array with two 64-bit integers, which represents
 * a bit mask corresponding to the array of control characters "cstr->buf".
 */
IN_Term getBitMask(cstring *cstr, IN_Term in_term)
{
    int     index;                                                              // Index
    u_int64 chmask;                                                             // Characters bit mask

    in_term.interm[0] = 0;
    in_term.interm[1] = 0;

    for (index = 0; index < cstr->len; index++) {
        if ((u_char) cstr->buf[index] > 127) continue;                          // Only allow ASCII characters in the bitmask
        chmask = (1UL << ((int) cstr->buf[index] % 64));
        in_term.interm[cstr->buf[index] / 64] |= chmask;
    }

    return in_term;
}

/*
 * This function sets the bit "bit" in the integer "options" according to the
 * flag "flag".
 */
int setOptionsBitMask(int options, int bit, int flag)
{
    int mask;

    mask = (1U << bit);
    if (flag == SET) return (options | mask);
    if (flag == UNSET) return (options & ~mask);
    return options;
}

/*
 * This function returns an integer which uniquely identifies the operation
 * "op->buf"; hence, "read", "write", "append", "io", "tcpip[6]", "udpip[6]",
 * "server[6]/tcpserver[6]", "udpserver[6]", "pipe", or "newpipe". Otherwise,
 * it returns a negative integer to indicate the error that has occurred.
 */
int getOperation(const cstring *op)
{
    char str[OPSIZE];                                                           // Useful buffer
    char *ptr;                                                                  // Pointer to '=' in operation

    if (op->len > OPSIZE) return getError(INT, ERRZ33);
    ptr = strcpy(str, (char *) op->buf);
    ptr = strchr(ptr, '=');
    if (ptr != NULL) *ptr = '\0';

    proto_family = PF_INET;                                                     // PF_INET or PF_INET6
    addr_family  = AF_INET;                                                     // AF_INET or AF_INET6
    sock_type    = SOCK_STREAM;                                                 // SOCK_STREAM or SOCK_DGRAM
    sock_proto   = IPPROTO_TCP;                                                 // IPPROTO_TCP or IPPROTO_UDP

    if (strlen(str) == 1) {
        char ch = tolower(op->buf[0]);                                          // Operation identifier

        if (ch == 'r') {
            return READ;
        } else if (ch == 'w') {
            return WRITE;
        } else if (ch == 'a') {
            return APPEND;
        } else if (ch == 'i') {
            return IO;
        } else if (ch == 't') {
            return TCPIP;
        } else if (ch == 'u') {
            sock_type = SOCK_DGRAM;
            sock_proto = IPPROTO_UDP;
            return TCPIP;
        } else if (ch == 's') {
            return SERVER;
        } else if (ch == 'p') {
            return PIPE;
        } else if (ch == 'n') {
            return NEWPIPE;
        }
    } else {
        if (strcasecmp(str, "read") == 0) {
            return READ;
        } else if (strcasecmp(str, "write") == 0) {
            return WRITE;
        } else if (strcasecmp(str, "append") == 0) {
            return APPEND;
        } else if (strcasecmp(str, "io") == 0) {
            return IO;
        } else if (strcasecmp(str, "tcpip") == 0) {
            return TCPIP;
        } else if ((strcasecmp(str, "t6") == 0) || (strcasecmp(str, "tcpip6") == 0)) {
            proto_family = PF_INET6;
            addr_family = AF_INET6;
            return TCPIP;
        } else if (strcasecmp(str, "udpip") == 0) {
            sock_type = SOCK_DGRAM;
            sock_proto = IPPROTO_UDP;
            return TCPIP;
        } else if ((strcasecmp(str, "u6") == 0) || (strcasecmp(str, "udpip6") == 0)) {
            proto_family = PF_INET6;
            addr_family = AF_INET6;
            sock_type = SOCK_DGRAM;
            sock_proto = IPPROTO_UDP;
            return TCPIP;
        } else if ((strcasecmp(str, "server") == 0) || (strcasecmp(str, "tcpserver") == 0)) {
            return SERVER;
        } else if ((strcasecmp(str, "s6") == 0) || (strcasecmp(str, "server6") == 0) || (strcasecmp(str, "tcpserver6") == 0)) {
            proto_family = PF_INET6;
            addr_family = AF_INET6;
            return SERVER;
        } else if ((strcasecmp(str, "us") == 0) || (strcasecmp(str, "udpserver") == 0)) {
            sock_type = SOCK_DGRAM;
            sock_proto = IPPROTO_UDP;
            return SERVER;
        } else if ((strcasecmp(str, "us6") == 0) || (strcasecmp(str, "udpserver6") == 0)) {
            proto_family = PF_INET6;
            addr_family = AF_INET6;
            sock_type = SOCK_DGRAM;
            sock_proto = IPPROTO_UDP;
            return SERVER;
        } else if (strcasecmp(str, "pipe") == 0) {
            return PIPE;
        } else if (strcasecmp(str, "newpipe") == 0) {
            return NEWPIPE;
        }
    }

    return getError(INT, ERRZ21);
}

/*
 * This function searches for an input terminator in the buffer "readbuf". If
 * the input terminator bit has been set in the integer "options", and a set
 * input terminator has been received (i.e., "interm"), this function will return
 * the position of the input terminator in the buffer. If no input terminator
 * can be found, the the number of bytes in the buffer searched (i.e., "nbytes")
 * is returned.
 *
 * NOTE: A set input terminator is an ASCII character (i.e., a character with an
 *       integer value between 0 and 127, inclusive) whose integer value
 *       corresponds to a set bit in the integer array "in_term". For example,
 *       the character NULL has an integer value of 0. If bit 0 in the integer
 *       array "in_term.interm[]" is equal to 1, then this bit has been set
 */
/*
int isINTERM(char *readbuf, int nbytes, int options, IN_Term in_term)           // NOTE: should we use this instead of the 4 loops?
{
    int index;
    int value;

    if (options & MASK[INTERM]) {
        for (index = 0; index < nbytes; index++) {
            if ((u_char) readbuf[index] > 127) continue;
            value = (int) readbuf[index];
            if (in_term.interm[value / 64] & MASK[value % 64]) return index;
        }
    }

    return nbytes;
}
*/

/*
 * This function accepts an error number argument "errnum" and returns a pointer
 * to the corresponding message string "errmsg".
 */
void getErrorMsg(int errnum, char  *errmsg)
{
    UTIL_strerror(-errnum, (u_char *) errmsg);
}

int signalCaught(SQ_Chan *c)
{
    c->dkey_len = 0;
    c->dkey[0] = '\0';

    if (partab.jobtab->trap & MASK[SIGALRM]) {
        partab.jobtab->trap &= ~MASK[SIGALRM];
        partab.jobtab->test = 0;
        return SIGALRM;
    } else {
        return -1;
    }
}

/*
 * This function tries to find out the current row and column of the
 * terminal device, returning a 0 on success, or errno on failure.
 */
/*
static int cursor_position(int oid, int *x, int *y)
{
    char cpr[12] = {'\0'};                                                      // For cursor position report
    char *cptr = cpr;                                                           // Cursor position report character pointer
    int  cprs = sizeof(cpr);
    int  i = 0;
    int  j = 0;
    int  ret;

    if ((ret = SQ_Flush()) < 0) return ret;                                     // Flush type-ahead input
    if (write(oid, (u_char *) "\033[6n", 4) == -1) return getError(SYS, errno); // Report cursor position

    do {                                                                        // Read the current position report
        if (read(oid, cptr++, 1) == -1) return getError(SYS, errno);
        if (*(cptr - 1) == '\033') i = j;
        if (++j >= cprs) break;
    } while (strchr(cpr, 'R') == NULL);

    if (sscanf(&cpr[i], "\033[%d;%dR", x, y) == EOF) return getError(SYS, errno);
    return 0;
}
*/

// SERVER SOCKET SPECIFIC

/*
 * This function initializes the forktab structure. It does not generate
 * any errors.
 */
void initFORK(forktab *f)
{
    f->job_no = -1;
    f->pid = -1;
}

/*
 * This function initializes the servertab structure. "chan" is a server
 * socket, where "size" represents the maximum number of processes this
 * socket can spawn to handle incoming client connections. It returns 0 on
 * success. Otherwise, it returns a negative integer value to indicate the
 * error that has occurred.
 */
int initSERVER(int chan, u_int size)
{
    servertab *s;
    forktab   *f = NULL;                                                        // TEMP FIX

    if (size > (systab->maxjob - 1)) return getError(INT, ERRZ42);
    s = &partab.jobtab->seqio[chan].s;

    if (size) {                                                                 // TEMP FIX
        f = malloc(sizeof(forktab) * size);                                     // to forktab from fork
        if (f == NULL) return getError(SYS, errno);
    }                                                                           // TEMP FIX

    for (u_int index = 0; index < size; index++) initFORK(&f[index]);
    s->slots = size;
    s->taken = 0;
    s->cid = -1;
    s->name[0] = '\0';
    s->forked = f;
    return 0;
}

/*
 * This function opens a server socket on channel "chan". "oper" is used
 * to determine the type of server socket to open:
 *     SERVER - Forking server
 *     NOFORK - Non-forking server
 *
 * If successful, this function returns 0. Otherwise, it returns a
 * negative integer value to indicate the error that has occurred.
 */
int openSERVER(int chan, const char *oper)
{
    SQ_Chan *c;
    char    *ptr;
    int     ret;

    // Acquire a pointer to the SQ_Chan structure
    c = &partab.jobtab->seqio[chan];

    // Extract size component from oper (SERVER[=size])
    ptr = strchr(oper, (int) '=');

    if (ptr == NULL) {
        c->mode = (u_char) NOFORK;
        if ((ret = initSERVER(chan, 0)) < 0) return ret;
    } else {
        if (sock_type == SOCK_DGRAM) return getError(INT, ERRZ21);
        ptr++;

        if (*ptr == '\0') {
            c->mode = (u_char) NOFORK;
            if ((ret = initSERVER(chan, 0)) < 0) return ret;
        } else {
            int size = atoi(ptr);

            if (size < 1) {
                return getError(SYS, errno);
            } else {
                if ((ret = initSERVER(chan, (u_int) size)) < 0) return ret;
            }
        }
    }

    return 0;
}

/*
 * This function:
 *   - Accepts a pending connection (if no clients are currently
 *     connected) on the channel "chan";
 *   - Forks a new process to handle the connection if any slots are
 *     available; and
 *   - Maintains a table of forked child processes (removing "dead"
 *     processes as required).
 *
 * If successful, this function will return the descriptor which references
 * the current connected client. Otherwise, it will return a negative integer
 * value to indicate the error that has occurred.
 */
int acceptSERVER(int chan, int tout)
{
    servertab *s;                                                               // Forked process table
    SQ_Chan   *c;                                                               // Server socket
    int       index;                                                            // Useful variable

    // Acquire a pointer to the SQ_CHAN structure
    c = &partab.jobtab->seqio[chan];

    // Acquire a pointer to the SERVERTAB structure
    s = &c->s;

    // Removing any dead child processes is only required if:
    //   s->slots > 0 AND s->taken > 0
    if ((s->slots > 0) && (s->taken > 0)) {
        for (index = 0; index < s->slots; index++) {
            if (s->forked[index].pid != -1) {
                if (partab.job_table[(s->forked[index].job_no - 1)].pid != s->forked[index].pid) {
                    // Child dead
                    initFORK(&s->forked[index]);
                    s->taken--;
                }
            }
        }
    }

    // An accept is only required if s->cid == -1 and it's a TCP socket
    // A recvfrom is only required if it's a UDP socket
    if ((s->cid == -1) || (sock_type == SOCK_DGRAM)) {
        int ret;                                                                // Return value

        if (sock_type == SOCK_DGRAM) {
            s->cid = c->fid;
        } else {
            s->cid = SQ_Tcpip_Accept(c->fid, tout);
        }

        if (s->cid < 0) {
            int error = s->cid;

            s->cid = -1;
            return error;
        }

        if (addr_family == AF_INET6) {
            struct sockaddr_in6 sin6;
            int                 len6 = sizeof(sin6);
            char                ipstr6[INET6_ADDRSTRLEN];

            if (sock_type == SOCK_DGRAM) {
                char buf[1];

                ret = recvfrom(s->cid, buf, 1, MSG_PEEK, (struct sockaddr *) &sin6, (socklen_t *) &len6);
                if (ret == -1) return getError(SYS, errno);
            } else {
                ret = getpeername(s->cid, (struct sockaddr *) &sin6, (socklen_t *) &len6);
                if (ret == -1) return getError(SYS, errno);
            }

            snprintf((char *) s->name, MAX_SEQ_NAME, "%s %u",
                     inet_ntop(addr_family, &sin6.sin6_addr, ipstr6, INET6_ADDRSTRLEN), ntohs(sin6.sin6_port));
        } else {
            struct sockaddr_in sin;
            int                len = sizeof(sin);
            char               ipstr[INET_ADDRSTRLEN];

            if (sock_type == SOCK_DGRAM) {
                char buf[1];

                ret = recvfrom(s->cid, buf, 1, MSG_PEEK, (struct sockaddr *) &sin, (socklen_t *) &len);
                if (ret == -1) return getError(SYS, errno);
            } else {
                ret = getpeername(s->cid, (struct sockaddr *) &sin, (socklen_t *) &len);
                if (ret == -1) return getError(SYS, errno);
            }

            snprintf((char *) s->name, MAX_SEQ_NAME, "%s %u",
                     inet_ntop(addr_family, &sin.sin_addr, ipstr, INET_ADDRSTRLEN), ntohs(sin.sin_port));
        }
    }

    // A fork/rfork is only required if:
    //   s->slots > 0 AND s->taken < s->slots
    if ((s->slots > 0) && (s->taken < s->slots)) {
        forktab *slot = NULL;                                                   // Find first available slot
        int     jobno;                                                          // M job number

        for (index = 0; index < s->slots; index++) {
            if (s->forked[index].pid == -1) {
                slot = &s->forked[index];
                index = s->slots;
            }
        }

        if (slot == NULL) return getError(INT, ERRZ20);

        // Spawn server client process
        jobno = ForkIt(1);                                                      // copy the file table

        if (jobno > 0) {                                                        // Parent process; child jobno
            slot->job_no = jobno;
            slot->pid = partab.job_table[jobno - 1].pid;
            s->taken++;
            close(s->cid);
            s->cid = -1;
            s->name[0] = '\0';
            return 0;
        } else if (jobno < 0) {                                                 // Child process; parent jobno
            c = &partab.jobtab->seqio[chan];
            s = &c->s;
            s->slots = 0;
            s->taken = 0;
            free(s->forked);
            close(c->fid);
            c->mode = (u_char) FORKED;
            s->forked = NULL;
            c->fid = s->cid;
            strncpy((char *) c->name, (char *) s->name, MAX_SEQ_NAME);
            s->cid = -1;
            s->name[0] = '\0';
            return c->fid;
        } else {                                                                // fork/rfork failed
            if (errno) return getError(SYS, errno);
            return getError(INT, ERRZ49);                                       // Job table full
        }

        // An unknown error has occurred
        return getError(INT, ERRZ20);
    }

    // Accepted by parent
    return s->cid;
}

/*
 * This function closes a connected client. This function will never
 * exit with an error (i.e., will always return 0).
 */
int closeSERVERClient(int chan)
{
    SQ_Chan *c;                                                                 // Socket channel

    // Acquire a pointer to the channel
    c = &partab.jobtab->seqio[chan];

    // Determine socket to close and close it (if required)
    if (((int) c->type == SQ_SOCK) && (sock_type == SOCK_STREAM)) {
        if ((int) c->mode == NOFORK) {
            if (c->s.cid != -1) {
                close(c->s.cid);
                free(c->s.forked);
                initSERVER(chan, 0);
            }
        } else if ((int) c->mode == SERVER) {
            if (c->s.cid != -1) {
                close(c->s.cid);
                c->s.cid = -1;
                c->s.name[0] = '\0';
            }
        }
    }

    return 0;
}

/*
 * This function closes the socket on channel "chan". This function is
 * called whenever a channel of type SQ_SOCK is closed. It will never exit
 * with an error (i.e., will always return 0).
 */
int closeSERVER(int chan)
{
    SQ_Chan   *c;                                                               // Socket
    servertab *s;                                                               // Forked process table

    c = &partab.jobtab->seqio[chan];
    s = &c->s;

    switch ((int) c->mode) {                                                    // Close socket client
    case TCPIP:
        close(c->fid);
        c->type = (u_char) SQ_FREE;
        break;

    // Does not close all forked child processes (just the server socket and
    // the client (if one exists) accepted by the server parent)
    case SERVER:
        if (s->cid != -1) close(s->cid);
        free(s->forked);                                                        // Free fork structure
        close(c->fid);
        c->type = (u_char) SQ_FREE;
        break;

    // Closes the connected client (if one exists) and the server socket
    case NOFORK:
        if (s->cid != -1) close(s->cid);
        close(c->fid);
        c->type = (u_char) SQ_FREE;
        break;

    // Closes the connected client accepted by a forked child process
    case FORKED:
        close(c->fid);
        c->type = (u_char) SQ_FREE;
        break;

    // Unknown class of socket
    default:
        break;
    }

    return 0;
}

// WRITE SPECIFIC

/*
 * This function writes "nbytes" bytes from the buffer "writebuf" to the object
 * associated with the channel "chan". Upon successful completion, the number
 * of bytes actually written is returned. Otherwise, a negative integer value
 * is returned to indicate the error that has occurred.
 */
int objectWrite(int chan, char *writebuf, int nbytes)
{
    SQ_Chan *c;                                                                 // Channel to write to
    int     oid;                                                                // Channel descriptor
    int     byteswritten;                                                       // Bytes written
    int     ret;                                                                // Return value

    c = &partab.jobtab->seqio[chan];                                            // Acquire a pointer to current channel

    if (chan == STDCHAN) {                                                      // Get appropriate channel descriptor
        oid = STDOUT_FILENO;
    } else if ((int) c->type == SQ_SOCK) {
        if ((int) c->mode == TCPIP) {
            oid = c->fid;
        } else if ((int) c->mode == FORKED) {
            oid = c->fid;
        } else if ((int) c->mode == NOFORK) {
            if (c->s.cid == -1) {
                return getError(INT, ERRZ47);
            } else {
                oid = c->s.cid;
            }
        } else if ((int) c->mode == SERVER) {
            if (c->s.cid == -1) {
                return getError(INT, ERRZ47);
            } else {
                oid = c->s.cid;
            }
        } else {
            return getError(INT, ERRZ20);
        }
    } else {
        oid = c->fid;
    }

    byteswritten = 0;                                                           // Initialize bytes written

    while (byteswritten < nbytes) {                                             // Write bytes
        int bytestowrite = nbytes - byteswritten;                               // Bytes left to write

        switch ((int) c->type) {
        case SQ_FILE:
            ret = SQ_File_Write(oid, (u_char *) &writebuf[byteswritten], bytestowrite);
            break;

        case SQ_TERM:
            ret = SQ_Device_Write(oid, (u_char *) &writebuf[byteswritten], bytestowrite);
            break;

        case SQ_PIPE:
            ret = SQ_Pipe_Write(oid, (u_char *) &writebuf[byteswritten], bytestowrite);
            break;

        case SQ_SOCK:
            ret = SQ_Tcpip_Write(oid, (u_char *) &writebuf[byteswritten], bytestowrite);
            break;

        default:
            return getError(INT, ERRZ30);
        }

        // Increment byteswritten or return error
        if (ret < 0) {
            if (partab.jobtab->trap & MASK[SIGPIPE]) {
                partab.jobtab->trap &= ~MASK[SIGPIPE];
                return getError(INT, ERRZ47);
            } else {
                return ret;
            }
        } else {
            byteswritten += ret;
        }
    }

    return byteswritten;
}

/*
 * READ SPECIFIC:
 *     "chan"   - Guaranteed to be valid
 *     "buf"    - Guaranteed not to be NULL, and large enough to store
 *                "maxbyt" bytes
 *     "maxbyt" - Guaranteed to be a positive integer value less than
 *                BUFSIZE
 *     "tout"   - If 0, forces the read to retrieve the next character
 *                from the object. If no character is immediately
 *                available, a timeout will occur (this does not
 *                apply to files). All other "tout" values are
 *                effectively ignored; that is, they are handled at a
 *                higher level.
 */

/*
 * This function reads at most "maxbyt" bytes from the file referenced by
 * the channel "chan" into the buffer "buf". Upon success, it returns the
 * number of bytes read. Otherwise, it returns a negative integer value to
 * indicate the error that has occurred.
 */
int readFILE(int chan, u_char *buf, int maxbyt)
{
    SQ_Chan *c;                                                                 // Current channel
    int     bytesread = 0;                                                      // Bytes read
    int     crflag = FALSE;                                                     // CR received

    c = &partab.jobtab->seqio[chan];                                            // Acquire a pointer to current channel

    for (;;) {                                                                  // Read in bytes
        int ret;                                                                // Return value

        if (bytesread >= maxbyt) {                                              // Check if # bytes required are over max
            c->dkey_len = 0;
            c->dkey[0] = '\0';
            return bytesread;
        }

        ret = SQ_File_Read(c->fid, &buf[bytesread]);                            // Read in one byte

        if (ret < 0) {                                                          // An error has occurred
            c->dkey_len = 0;
            c->dkey[0] = '\0';
            return ret;
        } else if (ret == 0) {                                                  // EOF reached
            c->dkey_len = 1;
            c->dkey[0] = (char) 255;
            c->dkey[1] = '\0';
            return bytesread;
        }

        // Check if an input terminator has been received
        if (c->options & MASK[INTERM]) {
            if (c->in_term.iterm == CRLF) {
                if ((u_char) buf[bytesread] == 13) {
                    crflag = TRUE;
                } else if (((u_char) buf[bytesread] == 10) && (crflag == TRUE)) {
                    c->dkey_len = 2;
                    c->dkey[0] = (char) 13;
                    c->dkey[1] = (char) 10;
                    c->dkey[2] = '\0';
                    return bytesread - 1;
                } else if (crflag == TRUE) {
                    crflag = FALSE;
                }
            } else if ((u_char) buf[bytesread] < 128) {
                if (c->in_term.interm[(u_char) buf[bytesread] / 64] & MASK[(u_char) buf[bytesread] % 64]) {
                    c->dkey_len = 1;
                    c->dkey[0] = buf[bytesread];
                    c->dkey[1] = '\0';
                    return bytesread;
                }
            }
        }

        // Support to echo the last byte read is still to be implemented
        bytesread++;                                                            // Increment total number of bytes read
    }
}

/*
 * This function reads at most "maxbyt" bytes from the socket referenced by
 * the channel "chan" into the buffer "buf". Upon success, it returns the
 * number of bytes read. Otherwise, it returns a negative integer value to
 * indicate the error that has occurred.
 */
int readTCP(int chan, u_char *buf, int maxbyt, int tout)
{
    SQ_Chan *c;                                                                 // Useful variable
    int     oid;                                                                // Object descriptor
    int     bytesread = 0;                                                      // Bytes read
    int     crflag = FALSE;                                                     // CR received
    int     ret;                                                                // Return value

    // Aquire a pointer to the appropriate channel structure
    c = &partab.jobtab->seqio[chan];

    /*
     * Get peer's socket descriptor
     * SERVER - Forking TCP server socket
     * NOFORK - Non-forking TCP server socket or UDP server socket
     * FORKED - Forked TCP server client
     * TCPIP  - Client socket
     */

    if (c->mode == (u_char) SERVER) {
        ret = 0;
        while (ret == 0) ret = acceptSERVER(chan, tout);
        oid = ret;
    } else if (c->mode == (u_char) NOFORK) {
        ret = acceptSERVER(chan, tout);
        oid = ret;
    } else {
        oid = c->fid;
    }

    if (oid < 0) {                                                              // Check for timeout
        c->dkey_len = 0;
        c->dkey[0] = '\0';

        if (partab.jobtab->trap & MASK[SIGALRM]) {
            c->dkey_len = 0;
            c->dkey[0] = '\0';
            partab.jobtab->trap &= ~MASK[SIGALRM];
            partab.jobtab->test = 0;
            return 0;
        } else {
            return oid;
        }
    }

    for (;;) {                                                                  // Read in bytes
        if (bytesread >= maxbyt) {                                              // Check for bytes read
            c->dkey_len = 0;
            c->dkey[0] = '\0';
            return bytesread;
        }

        ret = SQ_Tcpip_Read(oid, &buf[bytesread], tout);                        // Read one byte

        if (partab.jobtab->attention) {                                         // Check if signal received
            ret = signalCaught(c);

            if (ret == SIGALRM) {
                return bytesread;
            } else {
                return -1;
            }
        } else if ((ret < 0) || (sock_type == SOCK_DGRAM)) {                    // Check if an error has occurred, or it's using UDP
            return ret;
        } else if (ret == 0) {                                                  // EOF received
            c->dkey_len = 1;
            c->dkey[0] = (char) 255;
            c->dkey[1] = '\0';
            closeSERVERClient(chan);
            return bytesread;
        }

        // Support for escape sequences with sockets is still to be implemented

        // Check if an input terminator has been received
        if (c->options & MASK[INTERM]) {
            if (c->in_term.iterm == CRLF) {
                if ((u_char) buf[bytesread] == 13) {
                    crflag = TRUE;
                } else if (((u_char) buf[bytesread] == 10) && (crflag == TRUE)) {
                    c->dkey_len = 2;
                    c->dkey[0] = (char) 13;
                    c->dkey[1] = (char) 10;
                    c->dkey[2] = '\0';
                    return bytesread - 1;
                } else if (crflag == TRUE) {
                    crflag = FALSE;
                }
            } else if ((u_char) buf[bytesread] < 128) {
                if (c->in_term.interm[(u_char) buf[bytesread] / 64] & MASK[(u_char) buf[bytesread] % 64]) {
                    c->dkey_len = 1;
                    c->dkey[0] = buf[bytesread];
                    c->dkey[1] = '\0';
                    return bytesread;
                }
            }
        }

        // Support to echo the last byte read is still to be implemented
        bytesread++;                                                            // Increment total number of bytes read
    }
}

/*
 * This function reads at most "maxbyt" bytes from the pipe referenced by
 * the channel "chan" into the buffer "buf". Upon success, it returns the
 * number of bytes read. Otherwise, it returns a negative integer value to
 * indicate the error that has occurred.
 */
int readPIPE(int chan, u_char *buf, int maxbyt, int tout)
{
    SQ_Chan *c;                                                                 // Current channel
    int     oid;                                                                // Object descriptor
    int     bytesread = 0;                                                      // Bytes read
    int     crflag = FALSE;                                                     // CR received
    int     tmp;                                                                // Return value

    // Acquire a pointer to the current channel
    c = &partab.jobtab->seqio[chan];

    if (chan == STDCHAN) {
        oid = 0;                                                                // stdin
    } else {
        oid = c->fid;
    }

    for (;;) {                                                                  // Read in bytes
        int ret;                                                                // Return value

        if (bytesread >= maxbyt) {                                              // Check # bytes recd
            c->dkey_len = 0;
            c->dkey[0] = '\0';
            return bytesread;
        }

        ret = SQ_Pipe_Read(oid, &buf[bytesread], tout);                         // Read one byte

        // Support for escape sequences with pipes is still to be implemented

        if (partab.jobtab->attention) {                                         // Check for signal
            tmp = signalCaught(c);

            if (tmp == SIGALRM) {
                return bytesread;
            } else {
                return -1;
            }
        } else if (ret < 0) {
            return ret;
        } else if (ret == 0) {                                                  // EOF reached
            c->dkey_len = 1;
            c->dkey[0] = (char) 255;
            c->dkey[1] = '\0';
            return bytesread;
        } else if (c->options & MASK[INTERM]) {                                 // Check if an input terminator has been received
            if (c->in_term.iterm == CRLF) {
                if ((u_char) buf[bytesread] == 13) {
                    crflag = TRUE;
                } else if (((u_char) buf[bytesread] == 10) && (crflag == TRUE)) {
                    c->dkey_len = 2;
                    c->dkey[0] = (char) 13;
                    c->dkey[1] = (char) 10;
                    c->dkey[2] = '\0';
                    return bytesread - 1;
                } else if (crflag == TRUE) {
                    crflag = FALSE;
                }
            } else if ((u_char) buf[bytesread] < 128) {
                if (c->in_term.interm[(u_char) buf[bytesread] / 64] & MASK[(u_char) buf[bytesread] % 64]) {
                    c->dkey_len = 1;
                    c->dkey[0] = buf[bytesread];
                    c->dkey[1] = '\0';
                    return bytesread;
                }
            }
        }

        // Support to echo the last byte read is still to be implemented
        bytesread += ret;                                                       // Increment total number of bytes read
    }
}

/*
 * This function reads at most "maxbyt" bytes from the device referenced by
 * the channel "chan" into the buffer "buf". Upon success, it returns the
 * number of bytes read. Otherwise, it returns a negative integer value to
 * indicate the error that has occurred.
 *
 * NOTE: Terminal input buffer handling, including editing and history,
 *       conforms to ANSI X3.64-1979 R1990 (ISO 6429:1992 / ECMA-48:1991)
 */
int readTERM(int chan, u_char *buf, int maxbyt, int tout)
{
    SQ_Chan        *c;                                                          // Current channel
    int            oid;                                                         // Object descriptor
    int            ret;                                                         // Return value
    int            bytesread = 0;                                               // Bytes read
    int            crflag = FALSE;                                              // CR received
    int            i;
    int            len;
    char           val;                                                         // Useful variable
    u_char         curr;                                                        // Current character
    cstring        writebuf;                                                    // Bytes to echo
    struct winsize w;                                                           // For ioctl
    static char    editing = FALSE;                                             // In editing mode
    u_short        start;                                                       // The starting column of the input buffer

    // Aquire a pointer to the appropriate channel structure
    c = &partab.jobtab->seqio[chan];

    if (in_hist == OFF) {
        start = c->dx;                                                          // Input for M [READ]
    } else {
        start = prompt_len;                                                     // Input for direct and debug modes
    }

    // Initialize local variables
    if (chan == STDCHAN) {
        oid = STDCHAN;                                                          // stdin
    } else {
        oid = c->fid;
    }

    // Get the current terminal device screen dimensions
    if (ioctl(oid, TIOCGWINSZ, &w) == -1) return getError(SYS, errno);

    if (w.ws_col == 0) {                                                        // If oid stdin is [likely] redirected, use stdout
        if ((oid == STDCHAN) && isatty(STDOUT_FILENO)) {                        // If oid is principal and stdout is a TTY
            if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) == -1) return getError(SYS, errno); // Use stdout to get terminal dimensions
            if (w.ws_col == 0) w.ws_col = 80;                                   // Give up and set default column width
        } else {
            w.ws_col = 80;                                                      // Give up and set default column width
        }
    }

    if ((in_hist == OFF) && !(c->options & MASK[TYPEAHEAD])) {                  // Flush type-ahead input [read]
        if ((ret = SQ_Flush()) < 0) return ret;
    }

    // Read in bytes
    for (;;) {
        if (bytesread >= maxbyt) {                                              // Check bytes received
            c->dkey_len = 0;
            c->dkey[0] = '\0';
            return bytesread;
        }

        if (editing && (c->dx == (start + bytesread))) editing = FALSE;         // No longer editing if cursor is at buffer end

        // Read in one byte
        ret = SQ_Device_Read(oid, &curr, tout);
        buf[bytesread] = curr;

        if (ret < 0) {
            c->dkey_len = 0;
            c->dkey[0] = '\0';

            if (partab.jobtab->trap & MASK[SIGALRM]) {
                partab.jobtab->trap &= ~MASK[SIGALRM];
                partab.jobtab->test = 0;
                return bytesread;
            }

            return ret;
        } else if (ret == 0) {                                                  // EOF received
            c->dkey_len = 1;
            c->dkey[0] = (char) 255;
            c->dkey[1] = '\0';
            return bytesread;
        }

        // Check for Ctrl-H (ASCII 8 BS) or (ASCII 127 DEL) from Backspace key
        if (((curr == 8) && ((c->options & MASK[DEL8]) || (in_hist != OFF))) ||
          ((curr == 127) && ((c->options & MASK[DEL127]) || (in_hist != OFF)))) {
            if (bytesread <= 0) continue;

            if (in_hist == OFF) {
                if (c->options & MASK[TTYECHO]) {
                    ret = SQ_Device_Write(oid, (u_char *) "\b \b", 3);          // Backspace, rubout, then backspace
                    if (ret < 0) return ret;
                    c->dx--;
                }
            } else {
                if (c->dx <= start) continue;

                if (!(c->dx % w.ws_col)) {                                      // Cursor has hit the beginning of the line
                    // Move cursor to the beginning of the previous line
                    ret = SQ_Device_Write(oid, (u_char *) "\033[F", 3);
                    if (ret < 0) return ret;

                    // Move cursor to the end of the line
                    ret = SQ_Device_Write(oid, (u_char *) "\033[", 2);
                    if (ret < 0) return ret;
                    writebuf.len = ultocstring(writebuf.buf, w.ws_col);
                    ret = SQ_Device_Write(oid, writebuf.buf, writebuf.len);
                    if (ret < 0) return ret;
                    ret = SQ_Device_Write(oid, (u_char *) "G", 1);
                    if (ret < 0) return ret;
                } else {
                    ret = SQ_Device_Write(oid, (u_char *) "\b", 1);             // Backspace (ASCII)
                    if (ret < 0) return ret;
                }

                // Delete current character
                ret = SQ_Device_Write(oid, (u_char *) "\033[P", 3);
                if (ret < 0) return ret;

                if (editing && ((start + bytesread) > w.ws_col)) {              // Input buffer is longer than a single line
                    // Erase from current position to the end of the display
                    ret = SQ_Device_Write(oid, (u_char *) "\033[J", 3);
                    if (ret < 0) return ret;

                    // Save cursor position (DEC private)
                    ret = SQ_Device_Write(oid, (u_char *) "\0337", 2);
                    if (ret < 0) return ret;

                    // Write from the current position in the buffer to the end of the buffer
                    ret = SQ_Device_Write(oid, (u_char *) &buf[c->dx - start], start + bytesread - c->dx);
                    if (ret < 0) return ret;

                    // Restore cursor position (DEC private)
                    ret = SQ_Device_Write(oid, (u_char *) "\0338", 2);
                    if (ret < 0) return ret;
                }

                c->dx--;

                if (editing) {
                    for (i = c->dx - start; i < bytesread; i++) buf[i] = buf[i + 1]; // Shift buffer to the left by one
                }
            }

            bytesread--;
            continue;
        }

        // Check to see if an escape sequence is about to be received
        if ((curr == 27) && ((c->options & MASK[ESC]) || (in_hist != OFF))) {
            c->dkey_len = 1;
            c->dkey[0] = (char) 27;

            for (;;) {
                if (c->dkey_len > MAX_DKEY_LEN) return getError(INT, ERRZ39);
                ret = SQ_Device_Read(oid, &c->dkey[c->dkey_len], tout);

                if (ret < 0) {
                    c->dkey_len = 0;
                    c->dkey[0] = '\0';

                    if (partab.jobtab->trap & MASK[SIGALRM]) {                  // Operation timed out
                        partab.jobtab->trap &= ~MASK[SIGALRM];
                        partab.jobtab->test = 0;
                        return bytesread;
                    } else {
                        return ret;                                             // Read error
                    }
                } else if (ret == 0) {                                          // EOF received
                    c->dkey_len = 1;
                    c->dkey[0] = (char) 255;
                    c->dkey[1] = '\0';
                    return bytesread;
                }

                val = c->dkey[c->dkey_len];

                // Arrow keys and Delete key
                if ((val != 'O') && (((val >= 'A') && (val <= 'Z')) || ((val >= 'a') && (val <= 'z')) || (val == '~'))) {
                    c->dkey_len++;
                    c->dkey[c->dkey_len] = '\0';
                    if (in_hist == OFF) return bytesread;

                    // Up or Down Arrow
                    if ((c->dkey_len == 3) && ((val == 'A') || (val == 'B'))) {
                        if (in_hist == FALSE) in_hist = TRUE;

                        // Move to beginning of the line that starts the current command buffer
                        if (c->dx >= w.ws_col) {
                            ret = SQ_Device_Write(oid, (u_char *) "\033[", 2);
                            if (ret < 0) return ret;
                            writebuf.len = ultocstring(writebuf.buf, c->dx / w.ws_col);
                            ret = SQ_Device_Write(oid, writebuf.buf, writebuf.len);
                            if (ret < 0) return ret;
                            ret = SQ_Device_Write(oid, (u_char *) "F", 1);
                            if (ret < 0) return ret;
                        }

                        // Move to the beginning of the input buffer that starts the current command buffer (after the prompt)
                        ret = SQ_Device_Write(oid, (u_char *) "\033[", 2);
                        if (ret < 0) return ret;
                        writebuf.len = ultocstring(writebuf.buf, start + 1);
                        ret = SQ_Device_Write(oid, writebuf.buf, writebuf.len);
                        if (ret < 0) return ret;
                        ret = SQ_Device_Write(oid, (u_char *) "G", 1);
                        if (ret < 0) return ret;

                        // Erase from the current position to the end of the display
                        ret = SQ_Device_Write(oid, (u_char *) "\033[J", 3);
                        if (ret < 0) return ret;

                        // Change the input buffer to the previous or next command in the history buffer
                        if (val == 'A') {
                            if (hist_curr == 0) {
                                hist_curr = ((history[MAX_HISTORY - 1][0] != '\0') ? (MAX_HISTORY - 1) : hist_next);
                            } else {
                                hist_curr--;
                            }
                        } else {
                            if ((hist_curr < (MAX_HISTORY - 1) && (history[hist_curr][0] != '\0'))) {
                                hist_curr++;
                            } else {
                                hist_curr = 0;
                            }
                        }

                        len = strlen((char *) history[hist_curr]);
                        c->dx = start + len;
                        bytesread = len;
                        ret = SQ_Device_Write(oid, (u_char *) history[hist_curr], len); // Write out the buffer from history
                        if (ret < 0) return ret;
                        snprintf((char *) buf, maxbyt, "%s", history[hist_curr]);

                        if (!(c->dx % w.ws_col)) {                              // Cursor has hit the end of the line
                            // Move cursor to the beginning of the next line
                            ret = SQ_Device_Write(oid, (u_char *) "\033E", 2);
                            if (ret < 0) return ret;
                        }

                        break;
                    } else if ((maxbyt > 1) && (c->dkey_len == 3) && (val == 'C')) { // Right arrow
                        if (editing == FALSE) editing = TRUE;
                        if (c->dx >= (start + bytesread)) break;
                        c->dx++;

                        if (!(c->dx % w.ws_col)) {                              // Cursor has hit the end of the line
                            // Move cursor to the beginning of the next line
                            ret = SQ_Device_Write(oid, (u_char *) "\033[E", 3);
                            if (ret < 0) return ret;
                            break;
                        }

                        // Move cursor forward one position
                        ret = SQ_Device_Write(oid, (u_char *) "\033[C", 3);
                        if (ret < 0) return ret;
                        break;
                    } else if ((maxbyt > 1) && (c->dkey_len == 3) && (val == 'D')) { // Left arrow
                        if (editing == FALSE) editing = TRUE;
                        if (c->dx <= start) break;
                        c->dx--;

                        if (!((c->dx + 1) % w.ws_col)) {                        // Cursor has hit the beginning of the line
                            // Move cursor to the beginning of the previous line
                            ret = SQ_Device_Write(oid, (u_char *) "\033[F", 3);
                            if (ret < 0) return ret;

                            // Move cursor to the end of the current line
                            ret = SQ_Device_Write(oid, (u_char *) "\033[", 2);
                            if (ret < 0) return ret;
                            writebuf.len = ultocstring(writebuf.buf, w.ws_col);
                            ret = SQ_Device_Write(oid, writebuf.buf, writebuf.len);
                            if (ret < 0) return ret;
                            ret = SQ_Device_Write(oid, (u_char *) "G", 1);
                            if (ret < 0) return ret;
                            break;
                        }

                        // Move cursor back one position, stopping at column 1
                        ret = SQ_Device_Write(oid, (u_char *) "\033[D", 3);
                        if (ret < 0) return ret;
                        break;
                    } else if ((maxbyt > 1) && (c->dkey_len == 4) && (c->dkey[2] == '3') && (val == '~')) { // Delete key
                        if (editing == FALSE) editing = TRUE;
                        if (c->dx >= (start + bytesread)) break;

                        if ((start + bytesread) > w.ws_col) {                   // Input buffer is longer than a single line
                            // Erase from the current position to the end of the display
                            ret = SQ_Device_Write(oid, (u_char *) "\033[J", 3);
                            if (ret < 0) return ret;

                            // Save cursor position (DEC private)
                            ret = SQ_Device_Write(oid, (u_char *) "\0337", 2);
                            if (ret < 0) return ret;

                            // Write from the current position in the buffer to the end of the buffer
                            ret = SQ_Device_Write(oid, (u_char *) &buf[c->dx - start + 1], start + bytesread - c->dx - 1);
                            if (ret < 0) return ret;

                            // Restore cursor position (DEC private)
                            ret = SQ_Device_Write(oid, (u_char *) "\0338", 2);
                            if (ret < 0) return ret;
                        } else {
                            // Delete current character
                            ret = SQ_Device_Write(oid, (u_char *) "\033[P", 3);
                            if (ret < 0) return ret;
                        }

                        for (i = c->dx - start; i < bytesread; i++) buf[i] = buf[i + 1]; // Shift buffer to the left by one
                        bytesread--;
                        break;
                    }

                    return bytesread;
                }

                c->dkey_len++;

                if ((val == 27) || (val == 13)) {                               // End when a second escape or return key is hit
                    c->dkey[c->dkey_len] = '\0';
                    return bytesread;
                }
            }

            if (editing || (in_hist == TRUE)) continue;
        }                                                                       // End ESCAPE Processing Options

        // Check if an input terminator has been received
        if ((in_hist != OFF) || (c->options & MASK[INTERM])) {
            if ((in_hist == OFF) && (c->in_term.iterm == CRLF)) {
                if (curr == 13) {
                    crflag = TRUE;
                } else if ((curr == 10) && (crflag == TRUE)) {
                    c->dkey_len = 2;
                    c->dkey[0] = (char) 13;
                    c->dkey[1] = (char) 10;
                    c->dkey[2] = '\0';
                    return bytesread - 1;
                } else if (crflag == TRUE) {
                    crflag = FALSE;
                }
            } else if (curr < 128) {
                if ((in_hist != OFF) && (curr == 13)) {
                    if (in_hist == TRUE) in_hist = FALSE;
                    if (editing) editing = FALSE;
                    c->dkey_len = 1;
                    c->dkey[0] = curr;
                    c->dkey[1] = '\0';
                    return bytesread;
                } else if ((in_hist == OFF) && (c->in_term.interm[curr / 64] & MASK[curr % 64])) {
                    c->dkey_len = 1;
                    c->dkey[0] = curr;
                    c->dkey[1] = '\0';
                    return bytesread;
                }
            }
        }

        // Ignore non-printable characters when in direct and debug modes
        if ((in_hist != OFF) && !isprint(curr)) continue;

        if (editing) {
            if ((start + bytesread) < w.ws_col) {                               // Input buffer is a single line
                // Insert space and move the line to the right
                ret = SQ_Device_Write(oid, (u_char *) "\033[@", 3);
                if (ret < 0) return ret;
            }

            for (i = bytesread - 1; i >= (c->dx - start); i--) buf[i + 1] = buf[i]; // Shift buffer to the right by one
            buf[c->dx - start] = curr;                                          // Add new character in the correct position
        }

        // Echo last byte read
        if ((in_hist != OFF) || (c->options & MASK[TTYECHO])) {
            if (curr == '\t') curr = ' ';                                       // Convert tab to space
            writebuf.len = 1;
            snprintf((char *) writebuf.buf, 2, "%c", curr);
            ret = SQ_Write(&writebuf);
            if (ret < 0) return ret;
        }

        if (c->dx && (in_hist != OFF) && !(c->dx % w.ws_col)) {                 // Cursor has hit the end of the line
            // Move cursor to the beginning of the next line
            ret = SQ_Device_Write(oid, (u_char *) "\033E", 2);
            if (ret < 0) return ret;
        }

        if (editing) {
            if ((start + bytesread + 1) >= w.ws_col) {                          // Input buffer is longer than one line
                // Erase from current position to the end of the display
                ret = SQ_Device_Write(oid, (u_char *) "\033[J", 3);
                if (ret < 0) return ret;

                // Save cursor position (DEC private)
                ret = SQ_Device_Write(oid, (u_char *) "\0337", 2);
                if (ret < 0) return ret;

                // Write from current position in the buffer to the end of the buffer
                ret = SQ_Device_Write(oid, (u_char *) &buf[c->dx - start], start + bytesread - c->dx + 1);
                if (ret < 0) return ret;

                // Restore cursor position (DEC private)
                ret = SQ_Device_Write(oid, (u_char *) "\0338", 2);
                if (ret < 0) return ret;

                if (!((start + bytesread) % w.ws_col) && (c->dx % w.ws_col)) {  // Buffer hit the end of the line (cursor has not)
                    // Move cursor up one line to restore the correct position
                    ret = SQ_Device_Write(oid, (u_char *) "\033[A", 3);
                    if (ret < 0) return ret;
                }
            }
        }

        bytesread++;                                                            // Increment number of bytes read
    }
}

// INITIALIZE SPECIFIC

/*
 * This function initializes the channel "chan" to the defaults setting for
 * an object of type "type". If successful, it returns 0. Otherwise, it
 * returns a negative integer value to indicate the error that has occurred.
 */
int initObject(int chan, int type)
{
    SQ_Chan        *c;                                                          // Channel to initialize
    int            par;                                                         // Channel's options
    cstring        interm;                                                      // Input terminator(s)
    cstring        outerm;                                                      // Output terminator
    struct termios settings;                                                    // man 3 termios
    char           io;                                                          // Current IO channel

    c = &partab.jobtab->seqio[chan];
    par = 0;

    switch (type) {
    case SQ_FREE:
        c->type = (u_char) SQ_FREE;
        break;

    case SQ_FILE:
        c->type = (u_char) SQ_FILE;
        snprintf((char *) outerm.buf, 2, "%c", (char) 10);
        snprintf((char *) interm.buf, 2, "%c", (char) 10);
        break;

    case SQ_SOCK:
        c->type = (u_char) SQ_SOCK;
        snprintf((char *) outerm.buf, 3, "%c%c", (char) 13, (char) 10);
        snprintf((char *) interm.buf, 3, "%c%c", (char) 13, (char) 10);
        break;

    case SQ_PIPE:
        c->type = (u_char) SQ_PIPE;
        snprintf((char *) outerm.buf, 2, "%c", (char) 10);
        snprintf((char *) interm.buf, 2, "%c", (char) 10);
        break;

    case SQ_TERM:
        c->type = (u_char) SQ_TERM;

        if ((chan == STDCHAN) && isatty(STDCHAN)) {                             // Setup tty settings (if STDCHAN)
            if (tcgetattr(STDCHAN, &settings) == -1) return getError(SYS, errno); // Get parameters
            settings.c_lflag &= ~ICANON;                                        // Non-canonical mode
            settings.c_lflag &= ~ECHO;                                          // Do not echo
            settings.c_oflag &= ~ONLCR;                                         // Do not map NL to CR-NL out
            settings.c_iflag &= ~ICRNL;                                         // Do not map CR to NL in
            settings.c_cc[VMIN] = 1;                                            // Guarantees one byte is read
            settings.c_cc[VTIME] = 0;                                           //   in at a time
            settings.c_cc[VSUSP] = _POSIX_VDISABLE;                             // Disables SIGTSTP signal (^Z)
#if defined(__FreeBSD__) || defined(__NetBSD__)
            settings.c_cc[VDSUSP] = _POSIX_VDISABLE;                            // Funny <Ctrl><Y> thing
            settings.c_cc[VSTATUS] = _POSIX_VDISABLE;                           // Disables status
#endif
            settings.c_cc[VINTR] = '\003';                                      // ^C
            settings.c_cc[VQUIT] = '\024';                                      // ^T (use for status)
            if (tcsetattr(STDCHAN, TCSANOW, &settings) == -1) return getError(SYS, errno); // Set parameters
        }

        par |= (SQ_USE_ECHO | SQ_USE_ESCAPE | SQ_USE_TYPEAHEAD | SQ_USE_DEL127 | SQ_CONTROLC);
        snprintf((char *) outerm.buf, 3, "%c%c", (char) 13, (char) 10);
        snprintf((char *) interm.buf, 2, "%c", (char) 13);
        break;

    default:
        return getError(INT, ERRZ20);
    }

    c->options = 0;
    c->mode = (u_char) PRINCIPAL;
    c->fid = 0;
    initSERVER(chan, 0);
    c->dx = 0;
    c->dy = 0;
    c->name[0] = '\0';
    c->dkey_len = 0;
    c->dkey[0] = '\0';
    outerm.len = strlen((char *) outerm.buf);
    interm.len = strlen((char *) interm.buf);
    io = partab.jobtab->io;
    SQ_Use(chan, &interm, &outerm, par);
    partab.jobtab->io = io;
    return 0;
}

// IO functions

/*
 * This function initializes STDCHAN and other IO related stuff. If
 * successful, this function will return 0. Otherwise, it returns a
 * negative integer value to indicate the error that has occurred.
 */
short SQ_Init(void)
{
    int index;                                                                  // Useful variable
    int mask;                                                                   // Useful variable
    int ret;                                                                    // Return value
    int typ;                                                                    // object type

    // Integer bit mask
    for (index = 0; index < MASKSIZE; index++) MASK[index] = (1UL << index);

    // CRLF
    mask = (1U << 13);
    CRLF = mask;
    mask = (1U << 10);
    CRLF |= mask;

    // Define the set of signals that are to be delivered, ignored etc.
    ret = setSignals();
    if (ret < 0) return (short) ret;

    // Set the parameters associated with STDCHAN, depending on what it is associated with
    ret = getObjectMode(STDCHAN);
    if (ret < 0) return (short) ret;
    typ = getModeCategory(ret);
    if (typ < 0) return (short) typ;

    // Initialize object
    ret = initObject(STDCHAN, typ);
    if (ret < 0) return (short) ret;

    if (typ == SQ_SOCK) {                                                       // from inetd/xinetd?
        int                     len;
        struct sockaddr_storage addr;
        servertab               *s;                                             // Forked process table

        s = &partab.jobtab->seqio[STDCHAN].s;
        ret = openSERVER(STDCHAN, "SERVER");                                    // set it up
        if (ret < 0) return (short) ret;
        s->cid = STDCHAN;                                                       // already accepted
        len = sizeof(struct sockaddr_storage);
        ret = getpeername(s->cid, (struct sockaddr *) &addr, (socklen_t *) &len);
        if (ret == -1) return (short) getError(SYS, errno);

#if defined(_AIX) && !defined(_AIX53)
        if (addr.__ss_family == AF_INET6) {
#else
        if (addr.ss_family == AF_INET6) {
#endif
            struct sockaddr_in6 *sin6 = (struct sockaddr_in6 *) &addr;
            char                ipstr6[INET6_ADDRSTRLEN];
            const char          *ip;

            ip = inet_ntop(AF_INET6, &sin6->sin6_addr, ipstr6, INET6_ADDRSTRLEN);
            if (ip == NULL) return (short) getError(SYS, errno);
            snprintf((char *) s->name, MAX_SEQ_NAME, "%s %u", ipstr6, ntohs(sin6->sin6_port));
        } else {                                                                // AF_INET
            struct sockaddr_in *sin = (struct sockaddr_in *) &addr;
            char               ipstr[INET_ADDRSTRLEN];
            const char         *ip;

            ip = inet_ntop(AF_INET, &sin->sin_addr, ipstr, INET_ADDRSTRLEN);
            if (ip == NULL) return (short) getError(SYS, errno);
            snprintf((char *) s->name, MAX_SEQ_NAME, "%s %u", ipstr, ntohs(sin->sin_port));
        }

        return 0;                                                               // done
    }

    // Determine associated terminal if tty
    if (isatty(STDCHAN)) {
        snprintf((char *) partab.jobtab->seqio[STDCHAN].name, MAX_SEQ_NAME, "%s", ttyname(STDCHAN));
    } else {
        snprintf((char *) partab.jobtab->seqio[STDCHAN].name, MAX_SEQ_NAME, "Not a tty");
    }

    // Indicate success
    return 0;
}

/*
 * This function opens an object "object" on channel "chan". The object can be
 * opened for any (relevant) one of the following operations (i.e., "op"):
 *
 *     read    - Read only
 *     write   - Write only
 *     append  - Append
 *     io      - Read and write
 *     tcpip   - Client socket
 *     server  - Server socket
 *     pipe    - Write to named pipe
 *     newpipe - Read from named pipe
 *
 * A timeout value "tout" (where -1 means unlimited) is also provided. If
 * successful, open returns a non-negative integer, termed a descriptor.
 * Otherwise, it returns a negative integer to indicate the error that has
 * occurred.
 *
 * NOTE: "tout" does not apply for files; and a "tout" of zero is used to
 *       test ownership success without starting a timer
 */
short SQ_Open(int chan, cstring *object, const cstring *op, int tout)
{
    SQ_Chan *c;                                                                 // Channel to open
    int     oper;                                                               // Operation identifier
    int     ford;                                                               // File or device identifier
    int     obj;                                                                // Object identifier
    int     oid;                                                                // Object descriptor
    int     ret;                                                                // Return value
    int     i;                                                                  // a handy int
    short   save_addr;                                                          // save addr_family
    short   save_type;                                                          // save sock_type

    // Check for opening $PRINCIPAL. In RSM, this is a no-op (See close code as well).
    if (chan == STDCHAN) {
        if (tout > UNLIMITED) partab.jobtab->test = 1;                          // if there's a timeout then say open succeeded
        return 0;
    }

    /*
     * TODO: Make sure that object and op are at least one character long. If not,
     *       this means that somebody tried to use the new syntax [OPEN 1::2] on it.
     */

    // Check parameters
    if (isChan(chan) == 0) return (short) getError(INT, ERRZ25);                // Channel out of range
    if (isChanFree(chan) == 0) return (short) getError(INT, ERRZ26);            // Channel not free
    c = &partab.jobtab->seqio[chan];
    ret = checkCstring(object);                                                 // Invalid cstrings
    if (ret < 0) return (short) ret;
    ret = checkCstring(op);
    if (ret < 0) return (short) ret;
    if (object->len == 0) return (short) getError(INT, ERRZ28);                 // Is device descriptor empty?
                                                                                //   Invalid except for OPEN $PRINCIPAL::TO
    // Invalid timeout
    if (tout < UNLIMITED) return (short) getError(INT, ERRZ22);

    // Assume operation won't time out
    if (tout > UNLIMITED) partab.jobtab->test = 1;                              // assume won't time out

    // Determine object type and operation
    oper = getOperation(op);                                                    // Determine operation

    if (oper < 0) {
        return (short) oper;
    } else if ((oper == READ) || (oper == WRITE) || (oper == IO)) {             // Determine object
        ford = getObjectType((char *) object->buf);

        if (ford < 0) {
            return (short) ford;
        } else if ((ford == UNKNOWN) && (oper == WRITE)) {
            obj = SQ_FILE;
        } else if (ford == CHR) {
            obj = SQ_TERM;
        } else if (ford == REG) {
            obj = SQ_FILE;
        } else {
            return (short) getError(INT, ERRZ29);
        }
    } else if (oper == APPEND) {
        obj = SQ_FILE;
    } else if (oper == TCPIP) {
        obj = SQ_SOCK;
    } else if (oper == SERVER) {
        obj = SQ_SOCK;
    } else if (oper == PIPE) {
        obj = SQ_PIPE;
    } else if (oper == NEWPIPE) {
        obj = SQ_PIPE;
    } else {
        return (short) getError(INT, ERRZ29);
    }

    // Start timer
    if ((tout > 0) && (obj != SQ_FILE)) alarm(tout);

    // Open object
    switch (obj) {
    case SQ_FILE:
        oid = SQ_File_Open((char *) object->buf, oper);
        break;

    case SQ_TERM:
        oid = SQ_Device_Open((char *) object->buf, oper);
        break;

    case SQ_PIPE:
        oid = SQ_Pipe_Open((char *) object->buf, oper);
        break;

    case SQ_SOCK:
        oid = SQ_Tcpip_Open((char *) object->buf, oper);
        break;

    default:
        return (short) getError(INT, ERRZ30);
    }

    /*
     * Open completed (with or without error) - prevent alarm signal from being
     * sent, if it hasn't already done so.
     */
    alarm(0);

    if (oid < 0) {
        if (tout == 0) partab.jobtab->test = 0;                                 // Failed to obtain ownership

        if (partab.jobtab->trap & MASK[SIGALRM]) {                              // Timeout received
            partab.jobtab->trap &= ~MASK[SIGALRM];
            partab.jobtab->test = 0;
            return 0;
        } else if (partab.jobtab->trap & MASK[SIGINT]) {                        // Caught SIGINT
            return 0;
        } else {                                                                // Open error
            return (short) oid;
        }
    }

    save_addr = addr_family;                                                    // save addr_family, which is reset in initObject
    save_type = sock_type;                                                      // save sock_type, which is reset in initObject

    // Set default channel attributes
    ret = initObject(chan, obj);
    if (ret < 0) return (short) ret;
    c->mode = (u_char) oper;
    c->fid = oid;

    if (obj == SQ_SOCK) {
        if (save_addr == AF_INET6) c->options = setOptionsBitMask(c->options, IPV6, SET);
        if (save_type == SOCK_DGRAM) c->options = setOptionsBitMask(c->options, UDP, SET);
    }

    i = snprintf((char *) c->name, MAX_SEQ_NAME, "%s", object->buf);
    if (i < 0) fprintf(stderr, "errno = %d - %s\n", errno, strerror(errno));

    // Set SQ_Chan appropriately (if server socket)
    if (oper == SERVER) {
        ret = openSERVER(chan, (char *) op->buf);
        if (ret < 0) return (short) ret;
    }

    // Return object's descriptor
    return (short) oid;
}

/*
 * This function modifies the characteristics of a channel; that is, it:
 *
 *   - sets the channel as the current IO channel;
 *   - sets the channel's input terminator bit mask;
 *   - sets the channel's output terminator;
 *   - ignores or catches the SIGINT signal;
 *   - sets the channel's escape sequence set bit flag;
 *   - sets the channel's echo set bit flag;
 *   - sets the channel's delete key bit mask; and
 *   - disconnects a client from a server socket (if client exists).
 *
 * Upon success, this function will return 0. Otherwise, a negative integer
 * value is returned to indicate the error that has occurred.
 */
short SQ_Use(int chan, cstring *interm, const cstring *outerm, int par)
{
    SQ_Chan *c;                                                                 // Pointer to channel
    int     ret;                                                                // Return value
    int     flag;                                                               // UNSET, LEAVE, SET

    // Check parameters
    if (isChan(chan) == 0) return (short) getError(INT, ERRZ25);

    if (isChanFree(chan) == 1) {
        if (chan == STDCHAN) {
            return 0;
        } else {
            return (short) getError(INT, ERRZ27);
        }
    }

    if (interm != NULL) {
        ret = checkAsciiChars(interm);
        if (ret < 0) return (short) ret;
    }

    if (outerm != NULL) {
        ret = checkCstring(outerm);
        if (ret < 0) return (short) ret;
        if (outerm->len > MAX_SEQ_OUT) return (short) getError(INT, ERRZ33);
    }

    if (par < 0) return (short) getError(INT, ERRZ45);
    partab.jobtab->io = (u_char) chan;                                          // Set "chan" as the current IO channel
    c = &partab.jobtab->seqio[chan];                                            // Acquire a pointer to the channel

    if (interm != NULL) {                                                       // Set channel's input term(s) bit mask
        if (interm->len != 0) {
            c->in_term = getBitMask(interm, c->in_term);
            flag = SET;
        } else {
            c->in_term.interm[0] = 0;
            c->in_term.interm[1] = 0;
            flag = UNSET;
        }
    } else {
        flag = LEAVE;
    }

    // Set bit INTERM in channel's options
    c->options = setOptionsBitMask(c->options, INTERM, flag);

    if (outerm != NULL) {                                                       // Set channel's output terminator
        if (outerm->len != 0) {
            c->out_len = outerm->len;
            memcpy(c->out_term, outerm->buf, outerm->len);
            flag = SET;
        } else {
            c->out_len = 0;
            c->out_term[0] = '\0';
            flag = UNSET;
        }
    } else {
        flag = LEAVE;
    }

    // Set bit OUTERM in channel's options
    c->options = setOptionsBitMask(c->options, OUTERM, flag);

    // Set bit CONTROLC/CONTROLT in channel's options
    if (!chan && (par & (SQ_CONTROLC | SQ_NOCONTROLC | SQ_CONTROLT | SQ_NOCONTROLT))) { // Chan zero only
        struct termios settings;                                                // man 3 termios

        if (tcgetattr(STDCHAN, &settings) == -1) return (short) getError(SYS, errno); // Get parameters

        if (par & SQ_CONTROLC) {
            settings.c_cc[VINTR] = '\003';                                      // ^C
        } else if (par & SQ_NOCONTROLC) {
            settings.c_cc[VINTR] = _POSIX_VDISABLE;                             // No ^C
        }

        if (par & SQ_CONTROLT) {
            settings.c_cc[VQUIT] = '\024';                                      // ^T
        } else if (par & SQ_NOCONTROLT) {
            settings.c_cc[VQUIT] = _POSIX_VDISABLE;                             // No ^T
        }

        if (tcsetattr(STDCHAN, TCSANOW, &settings) == -1) return (short) getError(SYS, errno); // Set parameters
    }

    if (c->type == SQ_TERM) {
        // Set bit ESC in channel's options
        if (par & SQ_USE_ESCAPE) {
            c->options = setOptionsBitMask(c->options, ESC, SET);
        } else if (par & SQ_USE_NOESCAPE) {
            c->options = setOptionsBitMask(c->options, ESC, UNSET);
        } else {
            c->options = setOptionsBitMask(c->options, ESC, LEAVE);
        }

        // Set bit TTYECHO in channel's options
        if (par & SQ_USE_ECHO) {
            c->options = setOptionsBitMask(c->options, TTYECHO, SET);
        } else if (par & SQ_USE_NOECHO) {
            c->options = setOptionsBitMask(c->options, TTYECHO, UNSET);
        } else {
            c->options = setOptionsBitMask(c->options, TTYECHO, LEAVE);
        }

        // Set bit TYPEAHEAD in channel's options
        if (par & SQ_USE_TYPEAHEAD) {
            c->options = setOptionsBitMask(c->options, TYPEAHEAD, SET);
        } else if (par & SQ_USE_NOTYPEAHEAD) {
            c->options = setOptionsBitMask(c->options, TYPEAHEAD, UNSET);
        } else {
            c->options = setOptionsBitMask(c->options, TYPEAHEAD, LEAVE);
        }

        // Set bit DEL127 and bit DEL8 in channel's options
        if (par & SQ_USE_DELBOTH) {
            c->options = setOptionsBitMask(c->options, DEL127, SET);
            c->options = setOptionsBitMask(c->options, DEL8, SET);
        } else if (par & SQ_USE_DEL127) {
            c->options = setOptionsBitMask(c->options, DEL127, SET);
            c->options = setOptionsBitMask(c->options, DEL8, UNSET);
        } else if (par & SQ_USE_DEL8) {
            c->options = setOptionsBitMask(c->options, DEL127, UNSET);
            c->options = setOptionsBitMask(c->options, DEL8, SET);
        } else if (par & SQ_USE_DELNONE) {
            c->options = setOptionsBitMask(c->options, DEL127, UNSET);
            c->options = setOptionsBitMask(c->options, DEL8, UNSET);
        } else {
            c->options = setOptionsBitMask(c->options, DEL127, LEAVE);
            c->options = setOptionsBitMask(c->options, DEL8, LEAVE);
        }
    } else if (c->type == SQ_SOCK) {
        // Disconnect client from socket
        if (par & SQ_USE_DISCON) {
            closeSERVERClient(chan);
        }

        proto_family = PF_INET;                                                 // PF_INET or PF_INET6
        addr_family  = AF_INET;                                                 // AF_INET or AF_INET6
        sock_type    = SOCK_STREAM;                                             // SOCK_STREAM or SOCK_DGRAM
        sock_proto   = IPPROTO_TCP;                                             // IPPROTO_TCP or IPPROTO_UDP

        if (c->options & MASK[IPV6]) {
            proto_family = PF_INET6;
            addr_family  = AF_INET6;
        }

        if (c->options & MASK[UDP]) {
            sock_type = SOCK_DGRAM;
            sock_proto = IPPROTO_UDP;
        }
    }

    return 0;
}

/*
 * This function closes the channel "chan". It will always return 0; that
 * is, it will never return an error.
 */
short SQ_Close(int chan)
{
    SQ_Chan     *c;                                                             // Channel to close
    struct stat sb;                                                             // File properties

    // Check parameters
    if (isChan(chan) == 0) return 0;                                            // Channel out of range
    if (isChanFree(chan) == 1) return 0;                                        // Channel free
    if (chan == STDCHAN) return 0;                                              // Never close STDCHAN

    // If channel is current, the set current channel to STDCHAN
    if (chan == partab.jobtab->io) partab.jobtab->io = (u_char) STDCHAN;

    // Close channel
    c = &partab.jobtab->seqio[chan];

    switch ((int) c->type) {
    case SQ_FILE:
        // If the file is opened for writing or appending and it is empty, then delete it
        if (((int) c->mode == WRITE) || ((int) c->mode == APPEND)) {
            int ret = fstat(c->fid, &sb);

            if ((ret == 0) && (sb.st_size == 0)) {
                close(c->fid);
                unlink((char *) c->name);
            } else {
                close(c->fid);
            }
        } else {
            close(c->fid);
        }

        c->type = (u_char) SQ_FREE;
        break;

    case SQ_SOCK:
        closeSERVER(chan);
        break;

    case SQ_PIPE:
        if ((int) c->mode == NEWPIPE) {
            SQ_Pipe_Close(c->fid, (char *) c->name);
        } else {
            close(c->fid);
        }

        c->type = (u_char) SQ_FREE;
        break;

    case SQ_TERM:
        close(c->fid);
        c->type = (u_char) SQ_FREE;
        break;
    }

    return 0;                                                                   // Return success
}

/*
 * This function writes "writebuf->len" bytes from the buffer "writebuf->buf" to
 * the current IO channel. It also increments $X to the number of bytes
 * written. Upon successful completion, the number of bytes actually written
 * is returned. Otherwise, a negative integer value is returned to indicate
 * the error that has occurred.
 */
int SQ_Write(cstring *writebuf)
{
    int chan;                                                                   // Current IO channel
    int ret;                                                                    // Return value

    ret = checkCstring(writebuf);
    if (ret < 0) return ret;
    chan = (int) partab.jobtab->io;
    if (isChan(chan) == 0) return getError(INT, ERRZ25);
    if (isChanFree(chan) == 1) return getError(INT, ERRZ27);
    if (writebuf->len == 0) return 0;
    ret = objectWrite(chan, (char *) writebuf->buf, writebuf->len);
    if (ret < 0) return ret;

    if (partab.jobtab->seqio[chan].type == SQ_TERM) {
        int cnt = 0;                                                            // Printable count for $X

        for (int i = 0; i < writebuf->len; i++) {
            if (writebuf->buf[i] == '\t') {
                cnt += 8 - ((partab.jobtab->seqio[chan].dx + cnt) % 8);         // Increment $X by tab-over
            } else if (isprint(writebuf->buf[i])) {
                cnt++;                                                          // Increment $X by one
            }
        }

        partab.jobtab->seqio[chan].dx += cnt;
        if (partab.jobtab->seqio[chan].dx == 0) partab.jobtab->seqio[chan].dx++; // Increment $X by one if $X overflows
    } else {
        partab.jobtab->seqio[chan].dx += ret;
    }

    return ret;
}

/*
 * This function writes a character "c" to the current IO channel. Upon
 * successful completion, the number of bytes actually written is returned.
 * Otherwise, a negative integer value is returned to indicate the error that
 * has occurred.
 */
short SQ_WriteStar(u_char c)
{
    int chan;                                                                   // Current IO channel
    int ret;                                                                    // Return value

    chan = (int) partab.jobtab->io;
    if (isChan(chan) == 0) return (short) getError(INT, ERRZ25);
    if (isChanFree(chan) == 1) return (short) getError(INT, ERRZ27);
    ret = objectWrite(chan, (char *) &c, 1);
    if (ret < 0) return ret;

    if (partab.jobtab->seqio[chan].type == SQ_TERM) {
        if (c == '\t') {
            partab.jobtab->seqio[chan].dx += 8 - (partab.jobtab->seqio[chan].dx % 8); // Increment $X by tab-over
        } else if (isprint(c)) {
            partab.jobtab->seqio[chan].dx++;                                    // Increment $X by one
        }

        if (partab.jobtab->seqio[chan].dx == 0) partab.jobtab->seqio[chan].dx++; // Increment $X by one if $X overflows
    } else {
        partab.jobtab->seqio[chan].dx += ret;
    }

    return ret;
}

/*
 * This function works as follows:
 *
 *     "count" = SQ_FF (-2)
 *
 * Writes seven characters with integer values of 27, 91, 50, 74, 27, 91, and 72
 * to the current IO channel respectively. It also clears $X and $Y.
 *
 *     "count" = SQ_LF (-1)
 *
 * Writes the output terminator (if set) to the current IO channel. It also
 * clears $X, but adds 1 to $Y.
 *
 *     "count" >= 0
 *
 * Writes spaces to the current IO channel until $X is equal to "count".
 *
 * Upon successful completion the number of bytes actually written is returned.
 * Otherwise, a negative integer value is returned to indicate the error that
 * has occurred.
 */
int SQ_WriteFormat(int count)
{
    int     chan;                                                               // Current IO channel
    SQ_Chan *IOptr;                                                             // Useful pointer
    char    writebuf[WRITESIZE];                                                // Buffer to write from
    int     ret;                                                                // Return value
    int     byteswritten;                                                       // Bytes written
    int     numspaces;                                                          // Number of spaces to write
    int     bytestowrite;                                                       // Bytes to write
    int     index;                                                              // Useful integer

    if (count < -2) return getError(INT, ERRZ41);
    chan = (int) partab.jobtab->io;
    if (isChan(chan) == 0) return getError(INT, ERRZ25);
    if (isChanFree(chan) == 1) return getError(INT, ERRZ27);
    IOptr = &partab.jobtab->seqio[chan];

    switch (count) {
    case SQ_FF:
        if (IOptr->type == SQ_TERM) {
            // Erase entire display
            writebuf[0] = (char) 27;
            writebuf[1] = '[';
            writebuf[2] = '2';
            writebuf[3] = 'J';

            // Place cursor at home [0,0]
            writebuf[4] = (char) 27;
            writebuf[5] = '[';
            writebuf[6] = 'H';
            ret = objectWrite(chan, writebuf, 7);
        } else {
            // ASCII CR FF
            writebuf[0] = (char) 13;
            writebuf[1] = (char) 12;
            ret = objectWrite(chan, writebuf, 2);
        }

        if (ret < 0) return ret;
        IOptr->dx = 0;
        IOptr->dy = 0;
        return ret;

    case SQ_LF:
        ret = 0;

        if (IOptr->options & MASK[OUTERM]) {
            // Output for M [WRITE !] or not a principal TTY
            if ((in_hist == OFF) || (chan != STDCHAN) || !isatty((chan == STDCHAN) ? STDOUT_FILENO : chan)) {
                ret = objectWrite(chan, (char *) IOptr->out_term, IOptr->out_len);
            } else {                                                            // Output for direct and debug modes
                ret = objectWrite(chan, "\r\n", 2);
            }

            if (ret < 0) return ret;
        }

        IOptr->dx = 0;
        IOptr->dy++;
        return ret;

    default:
        byteswritten = 0;
        numspaces = count - IOptr->dx;

        while (numspaces > 0) {
            if (numspaces <= WRITESIZE) {
                bytestowrite = numspaces;
            } else {
                bytestowrite = WRITESIZE;
            }

            for (index = 0; index < bytestowrite; index++) writebuf[index] = ' ';
            ret = objectWrite(chan, writebuf, bytestowrite);

            if (ret < 0) {
                return ret;
            } else if (ret == 0) {
                if (partab.jobtab->trap & MASK[SIGINT]) return byteswritten;
            }

            IOptr->dx += ret;
            byteswritten += ret;
            numspaces -= bytestowrite;
        }

        if ((IOptr->type == SQ_TERM) && (IOptr->dx == 0)) IOptr->dx++;          // Increment $X by one if $X overflows
        return byteswritten;
    }
}

/*
 * This function reads a maximum of "maxbyt" bytes from the current IO
 * channel into the buffer "buf". Upon successful completion, the number of
 * bytes read into the buffer is returned. Otherwise, a negative integer
 * value is returned to indicate the error which has occurred.
 *
 * NOTE: That a timeout value (i.e., "tout") or "maxbyt" value of -1 means
 *       unlimited
 */
int SQ_Read(u_char *buf, int tout, int maxbyt)
{
    int chan;                                                                   // Channel to read from
    int type;                                                                   // Channel type
    int ret;                                                                    // Return value

    // Check parameters
    if (buf == NULL) return getError(INT, ERRZ28);
    chan = (int) partab.jobtab->io;
    if (isChan(chan) == 0) return getError(INT, ERRZ25);
    if (isChanFree(chan) == 1) return getError(INT, ERRZ27);
    if (tout < UNLIMITED) return getError(INT, ERRZ22);
    if (maxbyt < UNLIMITED) return getError(INT, ERRZ36);
    if (maxbyt > BUFSIZE) return getError(INT, ERRZ37);

    // Initialize variables
    partab.jobtab->seqio[chan].dkey_len = 0;
    partab.jobtab->seqio[chan].dkey[0] = '\0';
    if (maxbyt == UNLIMITED) maxbyt = BUFSIZE;
    type = (int) partab.jobtab->seqio[chan].type;

    if ((tout > 0) && (type != SQ_FILE)) alarm(tout);
    if ((tout > UNLIMITED) && (type != SQ_FILE)) partab.jobtab->test = 1;       // assume won't time out

    // Read from object
    switch (type) {
    case SQ_FILE:
        ret = readFILE(chan, buf, maxbyt);
        break;

    case SQ_SOCK:
        ret = readTCP(chan, buf, maxbyt, tout);
        break;

    case SQ_PIPE:
        ret = readPIPE(chan, buf, maxbyt, tout);
        break;

    case SQ_TERM:
        ret = readTERM(chan, buf, maxbyt, tout);
        break;

    default:
        return getError(INT, ERRZ30);
    }

    // Void the current alarm (even if one is not pending)
    alarm(0);

    // Return bytes read or error
    if (ret >= 0) {                                                             // Read successful
        if ((ret == 0) && (tout == 0)) partab.jobtab->test = 0;                 // no char and 0 TO so say failed
        buf[ret] = '\0';                                                        // NULL terminate
        return ret;
    } else if (partab.jobtab->trap & MASK[SIGINT]) {                            // SIGINT caught
        buf[0] = '\0';
        return 0;
    } else {                                                                    // Error
        if (tout == 0) partab.jobtab->test = 0;                                 // Failed to obtain ownership
        return ret;
    }
}

/*
 * This function reads one character from $IO and returns its ASCII value in
 * "result". If an escape sequence or input terminator has been received,
 * then "result" is equal to the first character in the escape sequence.
 * If a timeout expires or the signal SIGINT has been caught, then "result"
 * is equal to -1. This function will return 1 if a character was acquired.
 * It will return 0 if an escape sequence, input terminator, timeout or
 * SIGINT has been received. Otherwise, a negative integer value to indicate
 * the error that has occurred.
 *
 * NOTE: $X and $Y are unchanged, and any key pressed does not echo on a
 *       terminal device
 */
short SQ_ReadStar(int *result, int timeout)
{
    int    chan;                                                                // $IO
    char   origopt;                                                             // Original options
    u_char buf[2];                                                              // Read buffer
    int    ret;                                                                 // Return value

    // Check parameters
    if (result == NULL) return (short) getError(INT, ERRZ28);
    if (timeout < UNLIMITED) return (short) getError(INT, ERRZ36);
    chan = (int) partab.jobtab->io;
    if (isChan(chan) == 0) return (short) getError(INT, ERRZ25);
    if (isChanFree(chan) == 1) return (short) getError(INT, ERRZ27);

    // Save channel's original options
    origopt = partab.jobtab->seqio[chan].options;

    // Read Star options
#ifndef  __APPLE__
    partab.jobtab->seqio[chan].options &= ~MASK[INTERM];                        // macOS doesn't like this!!!
#endif
    partab.jobtab->seqio[chan].options &= ~MASK[DEL8];
    partab.jobtab->seqio[chan].options &= ~MASK[DEL127];

    // Get character
    ret = SQ_Read(buf, timeout, 1);

    // Restore options
    partab.jobtab->seqio[chan].options = origopt;

    // Return character's ASCII value or error
    if (ret < 0) {
        return (short) ret;                                                     // Read error
    } else if (ret == 0) {
        if (partab.jobtab->seqio[chan].dkey_len > 0) {                          // Escape sequence
            *result = partab.jobtab->seqio[chan].dkey[0];
            return 0;
        } else {                                                                // Timeout or SIGINT
            *result = -1;
            return 0;
        }
    } else {                                                                    // Received character
        *result = (int) buf[0];
        return 1;
    }
}

/*
 * This function flushes the input queue on a character special device. If
 * successful, 0 is returned. Otherwise, a negative integer is returned to
 * indicate the error that has occurred.
 */
short SQ_Flush(void)
{
    int     chan;                                                               // $IO
    SQ_Chan *c;                                                                 // Pointer to $IO
    int     what;                                                               // Queue

    chan = (int) partab.jobtab->io;                                             // Determine $IO
    if (isChan(chan) == 0) return (short) getError(INT, ERRZ25);                // $IO out of range
    c = &partab.jobtab->seqio[chan];                                            // Channel to flush
    what = TCIFLUSH;                                                            // Flush input queue

    if ((int) c->type == SQ_TERM) {
        int oid;                                                                // Device

        if (chan == STDCHAN) {                                                  // Flush stdin
            oid = 0;
        } else {                                                                // Flush other device
            oid = c->fid;
        }

        if (isatty(oid)) {
            int ret = tcflush(oid, what);                                       // Do flush

            if (ret == -1) return (short) getError(SYS, errno);
        } else {                                                                // Not a character
            return (short) getError(INT, ERRZ24);                               // Special device
        }
    }

    return 0;
}

/*
 * This function returns in the buffer "buf" the following three pieces of
 * information about the current IO channel:
 *     Piece Description
 *
 *     1     1 or 0
 *     2     error_code or object type (i.e., SQ_FILE, SQ_SOCK, SQ_PIPE, SQ_TERM)
 *     3     error_text or description of channel (e.g., file/device name or
 *           IP address port).
 *
 * Upon successful completion, this function returns the length of the buffer.
 * It returns a negative integer to indicate the internal or system error that
 * has occurred.
 *
 * NOTE: Each piece is comma deliminated
 */
int SQ_Device(u_char *buf)
{
    int        chan;                                                            // Current IO channel
    char       errmsg[2048];                                                    // Error message
    SQ_Chan    *c;                                                              // $IO pointer
    const char *name;                                                           // Channel's attributes

    // Check parameters
    if (buf == NULL) return getError(INT, ERRZ28);
    chan = (int) partab.jobtab->io;                                             // Determine the current IO channel

    if (isChan(chan) == 0) {                                                    // Check if channel is out of range
        getErrorMsg(ERRZ25, errmsg);
        sprintf((char *) buf, "1,%d,%s", ERRZ25, errmsg);
        return strlen((char *) buf);
    }

    c = &partab.jobtab->seqio[chan];                                            // Acquire a point to $IO

    if (isChanFree(chan) == 1) {                                                // Channel free
        sprintf((char *) buf, "1,0,");
        return strlen((char *) buf);
    }

    // Get channel's attributes
    if ((int) c->type == SQ_SOCK) {                                             // Socket specific
        switch ((int) c->mode) {
        case TCPIP:
            name = (char *) c->name;
            break;

        case SERVER:
            if (c->s.cid != -1) {
                name = (char *) c->s.name;
            } else {
                name = (char *) c->name;
            }

            break;

        case NOFORK:
            if (c->s.cid != -1) {
                name = (char *) c->s.name;
            } else {
                name = (char *) c->name;
            }

            break;

        case FORKED:
            name = (char *) c->name;
            break;

        default:
            name = NULL;
            break;
        }
    } else {                                                                    // All other objects
        name = (char *) c->name;
    }

    // Check for NULL pointer (name)
    if (name == NULL) {
        getErrorMsg(ERRZ28 + ERRMLAST, errmsg);
        sprintf((char *) buf, "1,%d,%s", ERRZ28 + ERRMLAST, errmsg);
        return strlen((char *) buf);
    }

    // Return channel's attributes
    sprintf((char *) buf, "0,%d,%s", (int) c->type, name);
    return strlen((char *) buf);
}

// ************************************************************************* //
// This function forces the message "msg" to the character special device
// "device". Upon successful completion, this functions returns 0.
// Otherwise, it returns a negative integer value to indicate the error that
// has occurred.
short SQ_Force(const cstring *device, const cstring *msg)
{
#ifndef __APPLE__
    FILE *fd;                                                                   // Useful variable
#endif
    int  ret;                                                                   // Useful variable

    // Check parameters
    ret = checkCstring(device);
    if (ret < 0) return (short) ret;                                            // Invalid cstring
    ret = checkCstring(msg);
    if (ret < 0) return (short) ret;                                            // Invalid cstring
    if (msg->len == 0) return 0;                                                // Quit without error if "msg" is empty

#ifdef __APPLE__
    return 0;
#else
    fd = fopen((char *) device->buf, "w");
    if (fd == NULL) return 0;
    alarm(2);                                                                   // timeout in two secs
    ret = fprintf(fd, "%s", msg->buf);
    if (fclose(fd) == EOF) return errno;
    if (ret < 0) return ret;
    alarm(0);
    return 0;
#endif
}
