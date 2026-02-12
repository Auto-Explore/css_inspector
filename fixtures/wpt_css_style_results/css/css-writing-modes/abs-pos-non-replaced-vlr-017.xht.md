# css/css-writing-modes/abs-pos-non-replaced-vlr-017.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vlr-017.xht"
}
```

## style[0]

```css
<![CDATA[
  div#containing-block
    {
      background: red url("support/bg-red-2col-2row-320x320.png");
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
      color: green;
      left: auto;
      position: absolute;
      right: 2em;
      width: auto;
      writing-mode: vertical-lr;
    }

/*
"
1. 'left' and 'width' are 'auto' and 'right' is not 'auto', then the width is shrink-to-fit. Then solve for 'left'
"

'left' + 'margin-left' + 'border-left-width' + 'padding-left' + 'width' + 'padding-right' + 'border-right-width' + 'margin-right' + 'right' = width of containing block

So:

  (solve) : left: auto
  +
      0px : margin-left
  +
      0px : border-left-width
  +
      0px : padding-left
  +
  (shrink-to-fit) : width: auto
  +
      0px : padding-right
  +
      0px : border-right-width
  +
      0px : margin-right
  +
    160px : right
    =====================
    320px : width of containing block

gives us:

  (solve) : left: auto
  +
      0px : margin-left
  +
      0px : border-left-width
  +
      0px : padding-left
  +
     80px : (shrink-to-fit) : width: auto
  +
      0px : padding-right
  +
      0px : border-right-width
  +
      0px : margin-right
  +
    160px : right
    =====================
    320px : width of containing block

And so computed left value must be 80px .
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
