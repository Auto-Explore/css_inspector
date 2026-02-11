# css/css-writing-modes/abs-pos-non-replaced-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-004.xht"
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
      color: green;
      left: auto;
      position: absolute;
      right: auto;
      width: auto;
      writing-mode: vertical-rl;
    }

/*
"
If all three of 'left', 'width', and 'right' are 'auto': First set any 'auto' values for 'margin-left' and 'margin-right' to 0. Then, if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the static position and apply rule number three below; otherwise, set 'right' to the static position and apply rule number one below.

3. 'width' and 'right' are 'auto' and 'left' is not 'auto', then the width is shrink-to-fit . Then solve for 'right'
"

'left' + 'margin-left' + 'border-left-width' + 'padding-left' + 'width' + 'padding-right' + 'border-right-width' + 'margin-right' + 'right' = width of containing block

So:

    160px : left: auto: set to static position
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
  (solve) : right: auto
    =====================
    320px : width of containing block

gives us:

    160px : left: auto: set to static position
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
  (solve) : right: auto
    =====================
    320px : width of containing block

And so computed right value must be 80px .
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
