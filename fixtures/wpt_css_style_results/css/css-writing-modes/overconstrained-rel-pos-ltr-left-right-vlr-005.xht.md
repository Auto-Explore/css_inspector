# css/css-writing-modes/overconstrained-rel-pos-ltr-left-right-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/overconstrained-rel-pos-ltr-left-right-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      background: transparent url("support/bg-red-upp-left-corn-320x320.png") no-repeat 358px 8px;
      /*
        16px : p's margin-right
       246px : img's width
        16px : p's margin-left
        80px : since we would need a bg-red which would be in 1st row and 2nd column
               (and not 1st column)
      ========
       358px : background-position from the right edge of document box
      */

      /* top = 8px since the red fail square is already on 1st row in bg-red-upp-left-corn-320x320 */

      direction: ltr;
      writing-mode: vertical-lr;
    }

/*
Layout calculation rules (such as those in CSS2.1, Section 9.4.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.

So here, left and right offset properties are input into the §9.4.3 algorithms where left offset property refer to top offset property in the layout rules and where right offset property refer to bottom offset property in the layout rules.
*/

  div
    {
      background-color: green;
      left: 80px;
      height: 80px;
      position: relative;
      right: 80px;
      width: 80px;
    }

  /*
  Here, 'left' should win, 'right' should be ignored and the used right value should become -'left'.
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
