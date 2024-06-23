/*
 * Package: Reference Standard M
 * File:    rsm/include/error.h
 * Summary: module RSM header file - error definitions
 *
 * David Wicksell <dlw@linux.com>
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Based on MUMPS V1 by Raymond Douglas Newman
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
 * SPDX-FileCopyrightText:  © 2020 David Wicksell <dlw@linux.com>
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

/*
 * Errors returned by functions internally are minus one of the following:
 *   Specifically -1 = ERRM1
 *   Functions return  -ERRMn
 *                     -(ERRZn + ERRMLAST)
 *                     -(ERRMLAST + ERRZLAST + errno)
 *
 * USE: short UTIL_strerror(int err, u_char *buf) to return the error string
 *
 * Add ERRZn definitions to the end of this file and the text form to rsm/util/strerror.c
 */

#ifndef RSM_ERROR_H
#define RSM_ERROR_H

#define ERRM6    6                                                              // Undefined local variable
#define ERRM75   75                                                             // String length exceeds implementation's limit

#define ERRMLAST 200                                                            // Must equal last MDC assigned error
#define ERRZ55   55                                                             // End of linked data reached
#define ERRZ56   56                                                             // Symbol table full

#endif
