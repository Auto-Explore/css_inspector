# css/css-writing-modes/abs-pos-non-replaced-vlr-013.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vlr-013.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

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
      bottom: auto;
      color: green;
      height: auto;
      position: absolute;
      top: auto;
    }

/*
"
Layout calculation rules (such as those in CSS2.1, Section 10.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.
"
7.1 Principles of Layout in Vertical Writing Modes
http://www.w3.org/TR/css-writing-modes-3/#vertical-layout

So here, *-top and *-bottom properties are input into the §10.3.7 algorithms where *-top properties refer to *-left properties in the layout rules and where *-bottom properties refer to *-right properties in the layout rules.

"
If all three of 'left', 'width', and 'right' are 'auto': First set any 'auto' values for 'margin-left' and 'margin-right' to 0. Then, if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the static position and apply rule number three below; otherwise, set 'right' to the static position and apply rule number one below.

1. 'left' and 'width' are 'auto' and 'right' is not 'auto', then the width is shrink-to-fit. Then solve for 'left'
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
  (based on the content) : height: auto
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
    160px : bottom: auto: set to static position
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
     80px : (based on the content) : height: auto
  +
      0px : padding-bottom
  +
      0px : border-bottom-width
  +
      0px : margin-bottom
  +
    160px : bottom: auto: set to static position
    =====================
    320px : height of containing block

And so computed top value must be 80px .
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
