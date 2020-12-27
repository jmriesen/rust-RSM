/*
 * Package:  Reference Standard M
 * File:     rsm/database/db_mount.c
 * Summary:  module RSM - mount a dataset
 *
 * David Wicksell <dlw@linux.com>
 * Copyright © 2020 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Based on MUMPS V1 by Raymond Douglas Newman
 * Copyright (c) 1999-2018
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
 *
 * ### UNDER DEVELOPMENT ###
 */

#include <stdio.h>                      // always include
#include <stdlib.h>                     // these two
#include <sys/types.h>                  // for u_char def
#include <string.h>                     // string controls always handy
#include <strings.h>
#include <ctype.h>                      // this is handy too
#include <sys/param.h>                  // for realpath() function
#include <errno.h>                      // error stuff
#include <fcntl.h>                      // file stuff
#include <unistd.h>                     // database access
#include <sys/ipc.h>                    // shared memory
#include <sys/shm.h>                    // shared memory
#include <sys/sem.h>                    // semaphores
#include <sys/stat.h>                   // stat
#include "rsm.h"                        // standard includes
#include "proto.h"                      // standard includes
#include "database.h"                   // database includes

//****************************************************************************
// Mount an environment
//
short DB_Mount(char *file,                      // database
                int volnum,                     // mount as volnum 2 -> MAX_VOL
                int gmb)                        // mb of global buf
{ int dbfd;                                     // database file descriptor
  int i;                                        // usefull int
  int n_gbd;					// number of gbd
  int indx;                                     // loop control
  int avbl;					// available space
  int map_size;                               	// size of database map (bytes)
  int volset_size;                              // size of volset struct (bytes)
  int pagesize;                                 // system pagesize (bytes)
  char fullpathvol[MAXPATHLEN];			// full pathname of vol file
  gbd *gptr;					// a gbd pointer
  u_char *ptr;					// and a byte one
  u_char *ptr2;					// and another
  label_block *labelblock;			// label block pointer

  if ((volnum < 2) || (volnum > MAX_VOL))	// valid volnum range for mount
  { return -(ERRZ71+ERRMLAST);
  }
  if (systab->vol[volnum - 1]->file_name[0] != 0) // check for already open
  { return -(ERRZ71+ERRMLAST);
  }
  if (systab->addsize / MBYTE < 2)		// check for spare buffer space
  { return -(ERRZ60+ERRMLAST);
  }
  if (systab->maxjob == 1)			// if in single user mode
  { return -(ERRZ60+ERRMLAST);			// complain
  }

  dbfd = open(file, O_RDWR);                    // open the database read/write
  if (dbfd < 1)                                 // if that failed
  { return -(errno+ERRMLAST+ERRZLAST);          // exit with error
  }                                             // end file create test

  labelblock = systab + systab->addoff;		// point at the spare memory

  i = read(dbfd, labelblock, sizeof(label_block)); // read label block
  if (i < sizeof(label_block))			// in case of error
  { return -(ERRZ73+ERRMLAST);
  }

  if (labelblock->magic != RSM_MAGIC)
  { return -(ERRZ73+ERRMLAST);
  }
  if (labelblock->db_ver != DB_VER)		// if we need to upgrade
  { return -(ERRZ73+ERRMLAST);
  }
  map_size = labelblock->header_bytes - sizeof(label_block); // fetch map size
  avbl = systab->addsize;			// and available size
  avbl -= map_size;				// subtract map size
  avbl -= sizeof(label_block);			// and label block size
  avbl -= sizeof(vol_def);			// and the vol structure

  systab->vol[volnum - 1] = (vol_def *) (labelblock + labelblock->header_bytes + sizeof(label_block)); // the vol structure
  systab->vol[volnum - 1]->vollab = labelblock;	// and point to label blk

  ptr = systab->vol[volnum - 1]->vollab + sizeof(vol_def); // up to here
  ptr2 = ptr;					// save

  pagesize = getpagesize();                     // get sys pagesize (bytes)

  ptr = ((ptr / pagesize) + 1) * pagesize; 	// round up to next page boundary
  avbl -= (ptr - ptr2);				// and adjust size

  n_gbd = avbl / systab->vol[volnum - 1]->vollab->block_size; // This many

  // remember the zero block
  // ???

  n_gbd = (gmb * MBYTE) / hbuf[3];		// number of gbd
  while (n_gbd < MIN_GBD)			// if not enough
  { gmb++;					// increase it
    n_gbd = (gmb * MBYTE) / hbuf[3];		// number of gbd
  }

  sjlt_size = (((sjlt_size - 1) / pagesize) + 1) * pagesize; // round up
  volset_size = sizeof(vol_def)			// size of VOL_DEF (one for now)
	      + hbuf[2]				// size of head and map block
	      + (n_gbd * sizeof(struct GBD))	// the gbd
              + (gmb * MBYTE)		  	// mb of global buffers
              + hbuf[3]			       	// size of block (zero block)
              + (rmb * MBYTE);		 	// mb of routine buffers
  volset_size = (((volset_size - 1) / pagesize) + 1) * pagesize; // round up
  share_size = sjlt_size + volset_size;		// shared memory size

  systab->vol[0]->map = (void*) ((char *) systab->vol[0]->vollab + sizeof(label_block));
						// and point to map
  systab->vol[0]->first_free = systab->vol[0]->map; // init first free

  systab->vol[0]->gbd_head = (gbd *) ((char *) systab->vol[0]->vollab + hbuf[2]); // gbds
  systab->vol[0]->num_gbd = n_gbd;		// number of gbds

  systab->vol[0]->global_buf = (void *) &systab->vol[0]->gbd_head[n_gbd]; 	//glob buffs
  systab->vol[0]->zero_block = (void *) &(((u_char *) systab->vol[0]->global_buf)[gmb * MBYTE]);
						// pointer to zero blk
  systab->vol[0]->rbd_head = (void *) ((char *) systab->vol[0]->zero_block + hbuf[3]); //rbds
  systab->vol[0]->rbd_end = (void *) ((char *) systab + share_size); // end of share

  systab->vol[0]->shm_id = shar_mem_id;		// set up share id
  systab->sem_id = sem_id;			// set up semaphore id
  systab->vol[0]->map_dirty_flag = 0;		// clear dirty map flag

  if ((realpath(file, fullpathvol)))		// get full path
    { if (strlen(fullpathvol) < VOL_FILENAME_MAX) // if can fit in our struct
        { strcpy(systab->vol[0]->file_name, 	// copy this full path into
		  fullpathvol);			// the vol_def structure
        }					// end if path will fit
      else					// otherwise
        { i = VOL_FILENAME_MAX - strlen(fullpathvol);	// copy as much as
          strcpy(systab->vol[0]->file_name,	// is possible, thats
		  &fullpathvol[i]);		// the best we can do
        }					// end length testing
    }						// end realpath worked
  else						// otherwise there was an error
    { i = shmdt(systab);			// detach the shared mem
      i = shmctl(shar_mem_id, (IPC_RMID), &sbuf); // remove the share
      i = semctl(sem_id, 0, (IPC_RMID), NULL);	// and the semaphores
      return(errno);				// exit with error
    }

  i = shmctl(shar_mem_id, (IPC_STAT), &sbuf);	// get status for later
  i = lseek(dbfd, 0, SEEK_SET);			// re-point at start of file
  i = read(dbfd, systab->vol[0]->vollab, hbuf[2]); // read label & map block
  if (i < hbuf[2])                              // in case of error
  { fprintf(stderr, "Read of label/map block failed\n - %s\n", //complain
            strerror(errno));                   // what was returned
    i = shmdt(systab);                          // detach the shared mem
    i = shmctl(shar_mem_id, (IPC_RMID), &sbuf);	// remove the share
    i = semctl(sem_id, 0, (IPC_RMID), NULL);	// and the semaphores
    return(errno);                              // exit with error
  }

  if (systab->vol[0]->vollab->clean == 0)	// if not a clean dismount
  { fprintf(stderr, "WARNING: Volume was not dismounted properly!\n");
    systab->vol[0]->upto = 1;			// mark for cleaning
  }
  else
  { systab->vol[0]->vollab->clean = 1;		// mark as mounted
    systab->vol[0]->map_dirty_flag = 1;		// and map needs writing
  }
  jobs = jobs/DAEMONS;                          // number of daemons
  if (jobs < MIN_DAEMONS) jobs = MIN_DAEMONS;   // minimum of MIN_DAEMONS
  if (jobs > MAX_DAEMONS) jobs = MAX_DAEMONS;	// and the max
  systab->vol[0]->num_of_daemons = jobs;	// initalise this
  while (SemOp(SEM_WD, WRITE));			// lock WD
  for (indx=0; indx<jobs; indx++)		// for each required daemon
  { i = DB_Daemon(indx, 1);			// start each daemon (volume 1)
    if (i != 0)                                 // in case of error
    { fprintf(stderr, "**** Died on error - %s ***\n\n", // complain
              strerror(errno));                 // what was returned
      i = shmdt(systab);                        // detach the shared mem
      return(errno);                            // exit with error
    }
  }                                             // all daemons started

  if ((systab->vol[0]->vollab->journal_requested) &&
           (systab->vol[0]->vollab->journal_file[0]))
  { struct stat   sb;				// File attributes
    off_t jptr;					// file ptr
    jrnrec jj;					// to write with
    int jfd;					// file descriptor

    while (SemOp(SEM_GLOBAL, WRITE));		// lock GLOBAL
    systab->vol[0]->vollab->journal_available = 0; // assume fail
    i = stat(systab->vol[0]->vollab->journal_file, &sb); // check for file
    if ((i < 0) && (errno != ENOENT))		// if that's junk
    { fprintf(stderr, "Failed to access journal file %s\n",
    		systab->vol[0]->vollab->journal_file);
    }
    else					// do something
    { if (i < 0)				// if doesn't exist
      { ClearJournal(0);			// create it
      }						// end create code
      jfd = open(systab->vol[0]->vollab->journal_file, O_RDWR);
      if (jfd < 0)				// on fail
      { fprintf(stderr, "Failed to open journal file %s\nerrno = %d\n",
		systab->vol[0]->vollab->journal_file, errno);
      }
      else					// if open OK
      { u_char tmp[12];

	lseek(jfd, 0, SEEK_SET);
	errno = 0;
	i = read(jfd, tmp, 4);			// read the magic
	if ((i != 4) || (*(u_int *) tmp != (RSM_MAGIC - 1)))
	{ fprintf(stderr, "Failed to open journal file %s\nWRONG MAGIC\n",
		  systab->vol[0]->vollab->journal_file);
	  close(jfd);
	}
	else
	{ i = read(jfd, &systab->vol[0]->jrn_next, sizeof(off_t));
	  if (i != sizeof(off_t))
	  { fprintf(stderr, "Failed to use journal file %s\nRead failed - %d\n",
		    systab->vol[0]->vollab->journal_file, errno);
	    close(jfd);
	  }
	  else
	  { jptr = lseek(jfd, systab->vol[0]->jrn_next, SEEK_SET);
	    if (jptr != systab->vol[0]->jrn_next)
	    { fprintf(stderr, "Failed journal file %s\nlseek failed - %d\n",
		      systab->vol[0]->vollab->journal_file, errno);
	      close(jfd);
	    }
	    else
	    {
#if RSM_DBVER == 1
              jj.size = 8;
#else
              jj.size = 12;
#endif
	      jj.action = JRN_START;
	      jj.uci = 0;
#if RSM_DBVER == 1
	      jj.time = current_time(TRUE);
#else
	      jj.time = (u_int64) current_time(TRUE);
#endif
	      i = write(jfd, &jj, jj.size);	// write the create record
	      systab->vol[0]->jrn_next += jj.size; // adjust pointer
	      lseek(jfd, 4, SEEK_SET);
	      i = write(jfd, &systab->vol[0]->jrn_next, sizeof(off_t));
	      i = close(jfd);			// and close it
	      systab->vol[0]->vollab->journal_available = 1;
	      fprintf(stderr, "Journaling started to %s.\n", systab->vol[0]->vollab->journal_file); // say it worked
	    }
	  }
	}
      }
    }
    SemOp(SEM_GLOBAL, -WRITE);			// unlock global
  }						// end journal stuff

  SemOp(SEM_WD, -WRITE);			// release WD lock

  gptr = systab->vol[0]->gbd_head; 		// get start of gbds
  ptr = (u_char *) systab->vol[0]->global_buf;	// get start of Globuff
  for (i = 0; i < systab->vol[0]->num_gbd; i++)	// for each GBD
  { gptr[i].mem = (struct DB_BLOCK *) ptr;	// point at block
    ptr += systab->vol[0]->vollab->block_size;	// point at next
    if (i < (systab->vol[0]->num_gbd - 1))	// all but the last
    { gptr[i].next = &gptr[i+1];		// link to next
    }
    else					// the last
    { gptr[i].next = NULL;			// end of list
    }
    systab->vol[0]->gbd_hash[ GBD_HASH ] = gptr; // head of free list
  }						// end setup gbds

  Routine_Init();				// and the routine junk
  i = shmdt(systab);                            // detach the shared mem
  i = close(dbfd);                              // close the database
  printf("RSM environment initialized.\n");     // say something
  return (0);                                   // indicate success
}
