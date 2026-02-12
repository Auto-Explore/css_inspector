# css/css-writing-modes/abs-pos-non-replaced-vlr-049.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-vlr-049.xht"
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
      top: 1em;
    }

/*
"
Layout calculation rules (such as those in CSS2.1, Section 10.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.
"
7.1 Principles of Layout in Vertical Writing Modes
http://www.w3.org/TR/css-writing-modes-3/#vertical-layout

So here, *-top and *-bottom properties are input into the §10.3.7 algorithms where *-top properties refer to *-left properties in the layout rules and where *-bottom properties refer to *-right properties in the layout rules.

"
3. 'width' and 'right' are 'auto' and 'left' is not 'auto', then the width is shrink-to-fit . Then solve for 'right'
"

'top' + 'margin-top' + 'border-top-width' + 'padding-top' + 'height' + 'padding-bottom' + 'border-bottom-width' + 'margin-bottom' + 'bottom' = height of containing block

So:

     80px : top
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
  (solve) : bottom: auto
    =====================
    320px : height of containing block

gives us:

     80px : top
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
