# css/css-writing-modes/abs-pos-non-replaced-vrl-126.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-126.xht"
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
      writing-mode: vertical-rl;
    }

  div#containing-block > span
    {
      background-color: red;
      color: green;
      left: 1em;
      position: absolute;
      right: auto;
      width: auto;
    }

/*
"
Layout calculation rules (such as those in CSS2.1, Section 10.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.
"
7.1 Principles of Layout in Vertical Writing Modes
http://www.w3.org/TR/css-writing-modes-3/#vertical-layout

So here, *right and *left properties are input into the §10.6.4 algorithms where *right properties refer to *top properties in the layout rules and where *left properties refer to *bottom properties in the layout rules.

"
1. 'top' and 'height' are 'auto' and 'bottom' is not 'auto', then the height is based on the content per 10.6.7, set 'auto' values for 'margin-top' and 'margin-bottom' to 0, and solve for 'top'
"

'left' + 'margin-left' + 'border-left-width' + 'padding-left' + 'width' + 'padding-right' + 'border-right-width' + 'margin-right' + 'right' = width of containing block

So:

  (solve) : right: auto
  +
      0px : margin-right
  +
      0px : border-right-width
  +
      0px : padding-right
  +
  (based on the content) : width: auto
  +
      0px : padding-left
  +
      0px : border-left-width
  +
      0px : margin-left
  +
     80px : left
    =====================
    320px : width of containing block

gives us:

  (solve) : right: auto
  +
      0px : margin-right
  +
      0px : border-right-width
  +
      0px : padding-right
  +
     80px : (based on the content) : width: auto
  +
      0px : padding-left
  +
      0px : border-left-width
  +
      0px : margin-left
  +
     80px : left
    =====================
    320px : width of containing block

And so computed right value must be 160px .
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
