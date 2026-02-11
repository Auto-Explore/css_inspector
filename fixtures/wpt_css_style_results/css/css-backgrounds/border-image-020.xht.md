# css/css-backgrounds/border-image-020.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-image-020.xht"
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
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5 10 15 20 / 1 1 1 1 / 0 repeat repeat;

  /*
  Equivalent to this shorthand form (see border-image-017 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5% 10% 15% 20% / 15% 20% 35% 30% / 0 repeat repeat;

  or to this shorthand form  (see border-image-018 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5 10 15 20 / 15% 20% 35% 30% / 0 repeat repeat;

  or to this shorthand form (see border-image-019 test):
  border-image: url("support/outline-5px-10px-15px-20px-green.png") 5% 10% 15% 20% / 1 1 1 1 / 0 repeat repeat;
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
  "errors": 3,
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
      "message": "Invalid value for property “border-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
