# css/css-writing-modes/abs-pos-non-replaced-vrl-120.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-120.xht"
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
      background-color: red;
      bottom: 1em;
      color: green;
      height: auto;
      position: absolute;
      top: auto;
      writing-mode: vertical-rl;
    }

/*
"
1. 'top' and 'height' are 'auto' and 'bottom' is not 'auto', then the height is based on the content per 10.6.7, set 'auto' values for 'margin-top' and 'margin-bottom' to 0, and solve for 'top'
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
  (based on content) : height: auto
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
     80px : bottom: auto
    =====================
    320px : height of containing block

gives us:

  (solve) : top: auto
  +
      0px : margin-top
  +
      0px : border-top-width
  +
      0px : padding-top
  +
     80px : (based on content) : height: auto
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
     80px : bottom: auto
    =====================
    320px : height of containing block

And so computed top value must be 160px .
*/

  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
