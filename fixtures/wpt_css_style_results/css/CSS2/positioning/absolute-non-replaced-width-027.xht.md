# css/CSS2/positioning/absolute-non-replaced-width-027.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/absolute-non-replaced-width-027.xht"
}
```

## style[0]

```css
<![CDATA[
  div#rel-pos-container
  {
  background-color: green;
  height: 100px;
  position: relative;
  width: 100px;
  }

  div#rel-pos-container > div {position: absolute;}

  div#reference-red-overlapped
  {
  background-color: red;
  height: 33px;
  left: 33px;
  top: 33px;
  width: 33px;
  }

  div#test-green-overlapping
  {
  background-color: green;
  bottom: 0;
  height: auto;
  left: 0;
  margin: auto;
  max-height: 34px;
  max-width: 34px;
  right: 0;
  top: 0;
  width: auto;
  }

  /*

  First we set both margin-left and margin-right to 0 since

  "
  set 'auto' values for 'margin-left' and 'margin-right' to 0
  (...)
  5. 'width' is 'auto', 'left' and 'right' are not 'auto', then solve for 'width'
  "
  10.3.7 Absolutely positioned, non-replaced elements
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width


      0px       : left
  +
      0px (set) : margin-left
  +
      0px       : border-left
  +
      0px       : padding-left
  +
      (solve)   : width (not constrained yet by max-width)
  +
      0px       : padding-right
  +
      0px       : border-right
  +
      0px (set) : margin-right
  +
      0px       : right
  =============
    100px       : width of containing block

  So, (tentative) width is 100px but now we must
  constrain it by computed max-width value and so we
  must now re-enter the algorithm but this time,
  'width' is not 'auto': therefore horizontal margins
  must not be set to 0:

  "
  If none of the three (left, width, right) is 'auto':
  If both 'margin-left' and 'margin-right' are 'auto',
  solve the equation under the extra constraint that
  the two margins get equal values
  "
  10.3.7 Absolutely positioned, non-replaced elements
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width

  So:

      0px       : left
  +
      (solve)   : margin-left
  +
      0px       : border-left
  +
      0px       : padding-left
  +
     34px       : width (constrained by max-width)
  +
      0px       : padding-right
  +
      0px       : border-right
  +
      (solve)   : margin-right
  +
      0px       : right
  =============
    100px       : width of containing block

  Therefore, margin-left and margin-right used values are
  each respectively equal to ((100px minus 34px) divided by 2) == 33px.

  -----------------------------------------------

  First we set both margin-top and margin-bottom to 0 since

  "
  5. 'height' is 'auto', 'top' and 'bottom' are not 'auto',
  then 'auto' values for 'margin-top' and 'margin-bottom' are set to 0
  and solve for 'height'
  "
  10.6.4 Absolutely positioned, non-replaced elements
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-height

      0px       : top
  +
      0px (set) : margin-top
  +
      0px       : border-top
  +
      0px       : padding-top
  +
      (solve)   : height (not constrained yet by max-height)
  +
      0px       : padding-bottom
  +
      0px       : border-bottom
  +
      0px (set) : margin-bottom
  +
      0px       : bottom
  =============
    100px       : height of containing block

  So, (tentative) height is 100px but now we must
  constrain it by computed max-height value and so we
  must now re-enter the algorithm but this time,
  'height' is not 'auto': therefore vertical margins
  must not be set to 0:

  "
  If none of the three (top, height, bottom) are 'auto':
  If both 'margin-top' and 'margin-bottom' are 'auto',
  solve the equation under the extra constraint that
  the two margins get equal values.
  "
  10.6.4 Absolutely positioned, non-replaced elements
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-height

  So:

      0px       : top
  +
      (solve)   : margin-top
  +
      0px       : border-top
  +
      0px       : padding-top
  +
     34px       : height (constrained by max-height)
  +
      0px       : padding-bottom
  +
      0px       : border-bottom
  +
      (solve)   : margin-bottom
  +
      0px       : bottom
  =============
    100px       : height of containing block

  Therefore, margin-top and margin-bottom used values are
  each respectively equal to ((100px minus 34px) divided by 2) == 33px.

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
