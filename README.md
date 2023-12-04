# MAX78000 Peripherals API

A peripheral access crate for the Analog Devices MAX78000. Auto-generated with `svd2rust`. The included SVD file has not been checked for errors. Registers and fields should be verified prior to use.

## How to patch Analog Devices MSDK's SVD file

`patch max78000.svd.original max78000.svd.patch -o max78000.svd.patched`

## How to generate a new SVD patch file

`diff -u max78000.svd.original max78000.svd.patched > max78000.svd.patch`

## How to re-generate the peripherals API (requires `svd2rust` and `form`)

`svd2rust --reexport-core-peripherals -i max78000.svd.patched && rm -rf src && form -i lib.rs -o src && rm lib.rs && cargo fmt`

## `svd2rust` license

Copyright (c) 2016 Jorge Aparicio

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.

## `max78000.svd.original` license

Copyright (C) Maxim Integrated Products, Inc., All rights Reserved.

This software is protected by copyright laws of the United States and of foreign countries. This material may also be protected by patent laws and technology transfer regulations of the United States and of foreign countries. This software is furnished under a license agreement and/or a nondisclosure agreement and may only be used or reproduced in accordance with the terms of those agreements. Dissemination of this information to any party or parties not specified in the license agreement and/or nondisclosure agreement is expressly prohibited.

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL MAXIM INTEGRATED BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of Maxim Integrated Products, Inc. shall not be used except as stated in the Maxim Integrated Products, Inc. Branding Policy.

The mere transfer of this software does not imply any licenses of trade secrets, proprietary technology, copyrights, patents, trademarks, maskwork rights, or any other form of intellectual property whatsoever. Maxim Integrated Products, Inc. retains all ownership rights.
