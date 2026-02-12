# css/css-writing-modes/abs-pos-non-replaced-vrl-104.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-104.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      background: red url("support/bg-red-3col-2row-320x320.png");
      color: transparent;
      direction: ltr;
      font: 80px/1 Ahem;
      height: 320px;
      position: relative;
      width: 320px;
    }

  div#containing-block > span
    {
      background-color: red;
      bottom: auto;
      color: green;
      height: auto;
      position: absolute;
      top: auto;
      writing-mode: vertical-rl;
    }

/*
"
If all three of 'top', 'height', and 'bottom' are auto, set 'top' to the static position and apply rule number three below.

3. 'height' and 'bottom' are 'auto' and 'top' is not 'auto', then the height is based on the content per 10.6.7, set 'auto' values for 'margin-top' and 'margin-bottom' to 0, and solve for 'bottom'
"

'top' + 'margin-top' + 'border-top-width' + 'padding-top' + 'height' + 'padding-bottom' + 'border-bottom-width' + 'margin-bottom' + 'bottom' = height of containing block

So:

     80px : top: auto: set to static position
  +
      0px : margin-top
  +
      0px : border-top-width
  +
      0px : padding-top
  +
  (shrink-to-fit) : height: auto
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
  (solve) : bottom: auto
    =====================
    320px : height of containing block

gives us:

     80px : top: auto: set to static position
  +
      0px : margin-top
  +
      0px : border-top-width
  +
      0px : padding-top
  +
     80px : (shrink-to-fit) : height: auto
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
  (solve) : bottom: auto
    =====================
    320px : height of containing block

And so computed bottom value must be 160px .
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
