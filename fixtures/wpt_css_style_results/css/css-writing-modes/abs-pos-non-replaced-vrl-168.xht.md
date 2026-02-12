# css/css-writing-modes/abs-pos-non-replaced-vrl-168.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-168.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      background: red url("support/bg-red-3col-3row-320x320.png");
      color: transparent;
      direction: ltr;
      font: 80px/1 Ahem;
      height: 320px;
      position: relative;
      width: 320px;
    }

  div#containing-block > span
    {
      background-color: green;
      bottom: 1em;
      color: green;
      height: 1em;
      position: absolute;
      top: auto;
      width: 1em;
      writing-mode: vertical-rl;
    }

/*
"
4. 'top' is 'auto', 'height' and 'bottom' are not 'auto', then set 'auto' values for 'margin-top' and 'margin-bottom' to 0, and solve for 'top'
"

'top' + 'margin-top' + 'border-top-width' + 'padding-top' + 'height' + 'padding-bottom' + 'border-bottom-width' + 'margin-bottom' + 'bottom' = height of containing block

So:

  (solve) : top: auto
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
     80px : bottom
    =====================
    320px : height of containing block

And so computed top value must be 160px .
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
