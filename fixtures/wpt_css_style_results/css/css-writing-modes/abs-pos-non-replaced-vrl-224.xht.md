# css/css-writing-modes/abs-pos-non-replaced-vrl-224.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-224.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      background: red url("support/bg-red-2col-3row-320x320.png");
      color: transparent;
      direction: rtl;
      font: 80px/1 Ahem;
      height: 320px;
      position: relative;
      width: 320px;
    }

  div#containing-block > span
    {
      background-color: red;
      bottom: 2em;
      color: green;
      height: 1em;
      position: absolute;
      top: 2em;
      writing-mode: vertical-rl;
    }

/*
"
If none of the three are 'auto': If both 'margin-top' and 'margin-bottom' are 'auto', solve the equation under the extra constraint that the two margins get equal values. If one of 'margin-top' or 'margin-bottom' is 'auto', solve the equation for that value. If the values are over-constrained, ignore the value for 'bottom' and solve for that value.
"

'top' + 'margin-top' + 'border-top-width' + 'padding-top' + 'height' + 'padding-bottom' + 'border-bottom-width' + 'margin-bottom' + 'bottom' = height of containing block

So:

    160px : top
  +
      0px : margin-top
  +
      0px : border-top-width
  +
      0px : padding-top
  +
     80px : height
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
    160px : bottom
    =====================
    320px : height of containing block

gives us

    160px : top
  +
      0px : margin-top
  +
      0px : border-top-width
  +
      0px : padding-top
  +
     80px : height
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
  (solve) : bottom
    =====================
    320px : height of containing block

And so computed bottom value must be 80px .
*/

  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
