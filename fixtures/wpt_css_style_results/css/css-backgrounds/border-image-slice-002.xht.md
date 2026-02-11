# css/css-backgrounds/border-image-slice-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-slice-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test
  {
  border-color: red;
  border-style: solid;
  border-width: 15px 20px 35px 30px;
  border-image-source: url("support/outline-5px-10px-15px-20px-green.png");
  border-image-slice: 5 10 15 20; /* or 5% 10% 15% 20% : see border-image-slice-001 test */
  border-image-width: 15% 20% 35% 30%;
  /* 1 is default ; <number>s represent multiples of the corresponding
  computed border-width; % unit is a percentage of height or width of border-box.*/
  border-image-outset: 0; /* 0 is default */
  border-image-repeat: repeat repeat;
  /* stretch is default; since the green area we repeat is an
  undifferentiated and homogeneous image of green, then 'repeat' or
  'stretch' will be equivalent, will do and should cause equivalent
  rendered layout. */

  /*
  Equivalent to this shorthand form (see border-image-slice-124 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5% 10% 15% 20% / 15% 20% 35% 30% / 0 repeat repeat;

  or to this shorthand form (see border-image-slice-125 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5 10 15 20 / 15% 20% 35% 30% / 0 repeat repeat;

  or to this shorthand form (see border-image-slice-126 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5% 10% 15% 20% / 1 1 1 1 / 0 repeat repeat;

  or to this shorthand form  (see border-image-slice-127 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5 10 15 20 / 1 1 1 1 / 0 repeat repeat;
  */

  width: 50px;
  }

  img {vertical-align: top;}

  div#overlapped-red
  {
  background-color: red;
  bottom: 100px;
  height: 100px;
  position: relative;
  width: 100px;
  z-index: -1;
  }
  ]]>
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-slice”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-width”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-image-repeat”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
