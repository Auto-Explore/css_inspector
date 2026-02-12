# css/css-writing-modes/ortho-htb-alongside-vrl-floats-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/ortho-htb-alongside-vrl-floats-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  /*
    minimum used height of the document root element is 176px;
    less than 176px will make the reference file no longer reliable.

       36px : image intrinsic height
     +
        4px : descender space below baseline
     =========
       40px : 1st table with 'height: 25%'
    multiplied by
        4
     =========
      160px : height of the 3 tables
    +
        8px : body's margin-top
    +
        8px : body's margin-bottom
     =========
      176px
  */

  div
    {
      width: 100px;
    }

  div#first-blue-float
    {
      background-color: blue;
      float: left;
      height: 25%;
    }

  div#second-olive-float-with-clear
    {
      background-color: olive;
      clear: left;
      float: left;
      height: 50%;
    }

  div#orange-horiz-tb
    {
      background-color: orange;
      height: 75%;
      writing-mode: horizontal-tb;
    }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
