![Raison d'être](/very-descriptive-image.jpg)

**WARNING**: Quickly hacked together single purpose tool!  
All I needed is irradiance from some cubemaps I had around already.
For something more serious have a look at the source code of the quite amazing [Probulator](https://github.com/kayru/Probulator/)


How to use:  
`cargo run <somepath>`

Path must point to a directory containing the following files: "px.hdr", "nx.hdr", "py.hdr", "ny.hdr", "pz.hdr", "nz.hdr"  
All square, making up a cubemap together.

-------------

Windowing code is from [Probulator](https://github.com/kayru/Probulator/) which is licensed under MIT:

The MIT License (MIT)

Copyright (c) 2015 Yuriy O'Donnell

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.