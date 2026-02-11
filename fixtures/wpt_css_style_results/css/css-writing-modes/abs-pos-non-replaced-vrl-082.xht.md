# css/css-writing-modes/abs-pos-non-replaced-vrl-082.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-082.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      background: red url("support/bg-red-2col-2row-320x320.png");
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
      color: green;
      left: 1em;
      position: absolute;
      right: auto;
      width: 1em;
      writing-mode: vertical-rl;
    }

/*
"
6. 'right' is 'auto', 'left' and 'width' are not 'auto', then solve for 'right'
"

'left' + 'margin-left' + 'border-left-width' + 'padding-left' + 'width' + 'padding-right' + 'border-right-width' + 'margin-right' + 'right' = width of containing block

So:

     80px : left
  +
      0px : margin-left
  +
      0px : border-left-width
  +
      0px : padding-left
  +
     80px : width
  +
      0px : padding-right
  +
      0px : border-right-width
  +
      0px : margin-right
  +
  (solve) : right: auto
    =====================
    320px : width of containing block

And so computed right value must be 160px .
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
