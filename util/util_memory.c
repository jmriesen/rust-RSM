/*
 * Package:  Reference Standard M
 * File:     rsm/util/util_memory.c
 * Summary:  module util - memory subroutines
 *
 * David Wicksell <dlw@linux.com>
 * Copyright © 2020 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Based on MUMPS V1 by Raymond Douglas Newman
 * Copyright (c) 1999-2016
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
 * along with this program. If not, see http://www.gnu.org/licenses/.
 */

#include <stdio.h>                              // always include
#include <stdlib.h>                             // these two
#include <string.h>                             // for bcopy
#include <strings.h>
#include <sys/types.h>                          // for u_char def
#include <ctype.h>				// for isdigit
#include "rsm.h"                                // standard includes
#include "proto.h"                              // standard prototypes
#include "error.h"				// standard errors
#include "compile.h"                            // for rdb def
#include "symbol.h"                             // for NEW stuff

// ** This function is used in place of bcopy() to trap strstk overflows
//
short mcopy(u_char *src, u_char *dst, int bytes) // copy bytes
{ if ((dst >= partab.strstk_start) &&		// if dst is at or after strstk
       (dst < partab.strstk_last)   &&		// and before the end of strstk
       (&dst[bytes] > partab.strstk_last))	// and this will overflow strstk
       	 return -(ERRMLAST+ERRZ8);		// complain
  if (bytes > MAX_STR_LEN)
  { return -ERRM75;
  }
  bcopy(src, dst, bytes);			// if OK - copy it
  dst[bytes] = '\0';				// ensure null terminated
  return bytes;					// and return bytes copied
}

// Convert string (**src) to canonic number at (*dst) returning length
// The source pointer is left pointing at the terminating character.
//

short ncopy(u_char **src, u_char *dst)          // copy as number
{ u_char c;                                     // the character
  u_char *p;					// a pointer
  int i = 0;                                    // a usefull int
  int k = 0;                                    // and another
  int dp = 0;                                   // decimal place flag
  int minus = 0;                                // minus flag
  int exp = 0;					// exponent
  int expsgn = 1;				// exponent sign

  if ((dst >= partab.strstk_start) &&           // if dst is at or after strstk
      (dst < partab.strstk_last)   &&           // and before the end of strstk
      (&dst[MAX_NUM_BYTES] > partab.strstk_last)) // and this will overflow strstk
  { return -(ERRMLAST+ERRZ8);			// complain
  }
  p = *src;					// get pointer
  while (TRUE)                                  // scan the string
  { c = *p++;					// get the character
    if ((i == 0) && (k == 0))			// still on first char (no '0')
    { if (c == '+')				// check for a plus
        continue;                               // go for more
      if (c == '-')                             // check for a minus
      { minus = !minus;                         // negate minus flag
        continue;                               // go for more
      }
      if (minus)
      { dst[i++] = '-';                         // store minus if reqd
      }
    }
    if ((i == minus) && (c == '0'))		// if '0' and nothing saved
    { k = 1;					// flag '0' seen
      continue;					// and keep going
    }
    if (c == '.')                               // check for a dot
    { if (dp != 0) break;                       // quit if already have one
      dp = 1;                                   // indicate 'got one'
      dst[i++] = c;                             // copy the character
      continue;
    }

    if ((systab->historic & HISTORIC_EOK) && (c == 'E')) // bloody E stuff
    { c = *p++;					// get next
      if (c == '-')				// check minus
      { expsgn = -1;				// change sign
      }
      else if (isdigit(c))			// if digit
      { exp = c - '0';				// store value
      }
      else if (c != '+')			// if not a plus
      { break;					// done
      }
      while (TRUE)				// scan exponent
      { c = *p++;				// get next
        if (isdigit(c) == 0) break;		// if not a digit break
	exp = (exp * 10) + (c - '0');		// add to exponent
      }
      break;					// done
    }

    if (isdigit(c) == 0) break;			// if not a digit break
    dst[i++] = c;                               // copy the character
  }                                             // string now copied
  if (dp)                                       // if there was a dot
  { for (k = 0; dst[i-k-1] == '0'; k++)
      ;                                         // check for trailing zeroes
    i -= k;					// remove them (if any)
    if (dst[i - 1] == '.') i--;			// ensure last is not dot
  }
  if ((i) && (dst[i - 1] == '-')) i--;		// ensure last is not minus
  if (i == 0) dst[i++] = '0';			// make sure we have something
  dst[i] = '\0';                                // null terminate it
  --p;                                          // back up the source pointer
  *src = p;					// and store it
  if (!exp)					// if no exponent
  { return (short) i;                           // return the count
  }
  dst[i] = '0';					// jic
  dp = 0;					// clear DP flag
  for (k = 0; k < i; k++)			// scan string again
  { if (dst[k] == '.')				// if we found a dot
    { dp = 1;					// flag it
      break;					// and exit
    }
  }
  if (expsgn > 0)				// if positive
  { if (dp)					// if found a dot
    { for (; k < i; k++)			// scan to eos
      { dst[k] = dst[k + 1];			// copy one char
        dst[k + 1] = '.';			// replace the dot
        exp--;					// count one
        if (!exp)				// if all done
        { goto exit; 				// just exit
        }
      }
    }
    if ((exp + i) > MAX_NUM_BYTES)		// if too big
    { return -ERRM92;				// error
    }
    while (exp)					// while still need zeroes
    { dst[i++] = '0';				// copy a zero
      exp--;					// count it
    }
    dst[i] = '\0';				// null terminate
    goto exit;					// and exit
  }						// end positive exponent
  if (!dp)					// check for assumed dp
  { k = i;					// put it here
    i++;					// and incr length
  }
  if (k > 0)					// some to the left of dp
  { for (dp = k; dp > minus; dp--)		// scan back
    { dst[dp] = dst[dp - 1];			// copy the number
      dst[dp - 1] = '.';			// replace the dot
      exp--;					// count one
      if (!exp)					// if all done
      { goto exit;				// exit
      }
    }
  }
  if ((exp + i) > MAX_NUM_BYTES)		// if too big
  { return -ERRM92;				// error
  }
  bcopy(&dst[minus + 1], &dst[minus + exp + 1], i); // move right exp places
  for (k = minus + 1; k <= (minus + exp); dst[k++] = '0'); // zero fill
  i +=exp;					// add to the length

exit:
  dp = 0;					// assume no dp
  for (k = 0; k < i; k++)			// scan string
  { if (dst[k] == '.')				// if a dp
    { dp = 1;					// flag it
      break;					// and quit
    }
  }
  if (dp)					// if there is a dp
  { while (dst[i - 1] == '0')			// for all trailing 0
    { i--;					// ignore them
    }
    if (dst[i - 1] == '.')			// if a trailing dp
    { i--;					// ignore them
    }
  }
  if (!i)					// if no char
  { dst[i++] = '0';				// put a zero back
  }
  dst[i] = '\0';				// ensure null terminated

  dp = (dst[0] == '-');				// start point
  if (dst[dp] == '0')				// if leading zeroes
  { for (k = dp; (k < i) && (dst[k] == '0'); k++)
      ; 					// find first non-zero
    bcopy(&dst[k], &dst[dp], i - k);		// copy down
    i -= (k - dp);				// adjust size
    if (i == dp)				// if nothing
    { if (dp)					// if a minus
      { i--;					// ignore it
      }
      dst[i++] = '0';				// store a zero
    }
    dst[i] = '\0';				// null terminate
  }

  return (short) i;				// exit
}

// CleanJob is called to release all locks and unwind any routine attaches
// It is called with zero (current job) or the job# (ie. internal job+1)
// If not the current job, also free jobtab entry.
//

void CleanJob(int job)				// tidy up a job
{ int j;					// the job number
  int i;					// a handy int

  j = job - 1;					// copy argument to int job form
  if (!job) j = partab.jobtab - systab->jobtab; // or get current int job#
  LCK_Remove(j + 1);				// remove locks
  i = systab->jobtab[j].cur_do;			// get current do

  while (i)					// for each i
  { if (!job)					// called by ourselves ?
    { if (systab->jobtab[j].dostk[i].newtab != NULL)
	ST_Restore((ST_newtab *) systab->jobtab[j].dostk[i].newtab);
      if ((systab->jobtab[j].dostk[i].flags & DO_FLAG_ATT) &&
	  (systab->jobtab[j].dostk[i].symbol != NULL))
	ST_SymDet(((rbd *) systab->jobtab[j].dostk[i].routine)->num_vars,
	            systab->jobtab[j].dostk[i].symbol);	// detach symbols
    }
    if (systab->jobtab[j].dostk[i].flags & DO_FLAG_ATT) // if we attached
      Routine_Detach((rbd *) systab->jobtab[j].dostk[i].routine); // detach rou

    i--;					// decrement do ptr
  }						// end routine detach while
  if (!job)					// called by ourselves ?
  { i = ST_KillAll(0, NULL);			// kill all vars
    partab.src_var.volset = 0;			// clear vol
    partab.src_var.slen = 0;			// and slen
    partab.src_var.uci = UCI_IS_LOCALVAR;	// say - local variable
    VAR_CLEAR(partab.src_var.name);
    bcopy("$ETRAP", partab.src_var.name.var_cu, 6);
    i = ST_Kill(&partab.src_var);		// Kill $ETRAP
    VAR_CLEAR(partab.src_var.name);
    bcopy("$ECODE", partab.src_var.name.var_cu, 6);
    i = ST_Kill(&partab.src_var);		// Kill $ECODE
  }
  for (i = 0; i < MAX_VOL; i++)			// scan view table
    if (systab->jobtab[j].view[i] != NULL)	// if buffer locked
    { DB_ViewRel(i + 1, systab->jobtab[j].view[i]); // release it
      systab->jobtab[j].view[i] = NULL;		// clear entry
    }

  systab->jobtab[j].cur_do = 0;			// in case we get back here
  if (!job)					// if current job
  { for (i = 1; i < MAX_SEQ_IO; SQ_Close(i++))
      ;						// close all io
    partab.jobtab = NULL;			// clear jobtab
  }
  bzero(&systab->jobtab[j], sizeof(jobtab));	// zot all
  return;					// and exit
}
