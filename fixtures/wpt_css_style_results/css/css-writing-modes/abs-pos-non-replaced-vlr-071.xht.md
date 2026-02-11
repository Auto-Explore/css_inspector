# css/css-writing-modes/abs-pos-non-replaced-vlr-071.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vlr-071.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      background: red url("support/bg-red-3col-2row-320x320.png");
      color: transparent;
      direction: rtl;
      font: 80px/1 Ahem;
      height: 320px;
      position: relative;
      width: 320px;
    }

  div#containing-block > span
    {
      background-color: green;
      height: 1em;
      left: 2em;
      position: absolute;
      right: 1em;
      width: auto;
      writing-mode: vertical-lr;
    }

/*
"
5. 'width' is 'auto', 'left' and 'right' are not 'auto', then solve for 'width'
"

'left' + 'margin-left' + 'border-left-width' + 'padding-left' + 'width' + 'padding-right' + 'border-right-width' + 'margin-right' + 'right' = width of containing block

So:

    160px : left
  +
      0px : margin-left
  +
      0px : border-left-width
  +
      0px : padding-left
  +
  (solve) : width: auto
  +
      0px : padding-right
  +
      0px : border-right-width
  +
      0px : margin-right
  +
     80px : right
    =====================
    320px : width of containing block

And so computed width value must be 80px .
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
