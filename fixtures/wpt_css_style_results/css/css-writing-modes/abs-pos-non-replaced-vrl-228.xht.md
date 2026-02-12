# css/css-writing-modes/abs-pos-non-replaced-vrl-228.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vrl-228.xht"
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
      left: 2em;
      position: absolute;
      right: 2em;
      width: 1em;
      writing-mode: horizontal-tb;
    }

/*
"
Layout calculation rules (such as those in CSS2.1, Section 10.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.
"
7.1 Principles of Layout in Vertical Writing Modes
http://www.w3.org/TR/css-writing-modes-3/#vertical-layout

So here, *right and *left properties are input into the §10.6.4 algorithms where *right properties refer to *top properties in the layout rules and where *left properties refer to *bottom properties in the layout rules.

"
If none of the three are 'auto': If both 'margin-top' and 'margin-bottom' are 'auto', solve the equation under the extra constraint that the two margins get equal values. If one of 'margin-top' or 'margin-bottom' is 'auto', solve the equation for that value. If the values are over-constrained, ignore the value for 'bottom' and solve for that value.
"

'left' + 'margin-left' + 'border-left-width' + 'padding-left' + 'width' + 'padding-right' + 'border-right-width' + 'margin-right' + 'right' = width of containing block

So:

    160px : right
  +
      0px : margin-right
  +
      0px : border-right-width
  +
      0px : padding-right
  +
     80px : width
  +
      0px : padding-left
  +
      0px : border-left-width
  +
      0px : margin-left
  +
    160px : left
    =====================
    320px : width of containing block

give us

    160px : right
  +
      0px : margin-right
  +
      0px : border-right-width
  +
      0px : padding-right
  +
     80px : width
  +
      0px : padding-left
  +
      0px : border-left-width
  +
      0px : margin-left
  +
  (solve) : left
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
