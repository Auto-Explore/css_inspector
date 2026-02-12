# css/css-writing-modes/overconstrained-rel-pos-ltr-top-bottom-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/overconstrained-rel-pos-ltr-top-bottom-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      background: transparent url("support/bg-red-1col-2row-320x320.png") no-repeat 278px 8px;
      /*
        16px : p's margin-right
       246px : img's width
        16px : p's margin-left
      ========
       278px : background-position from the left edge of document box
      */

      /* top = 8px since the red fail square is already on 2nd row in bg-red-1col-2row-320x320 */

      direction: ltr;
      writing-mode: vertical-lr;
    }

/*
Layout calculation rules (such as those in CSS2.1, Section 9.4.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.

So here, top and bottom offset properties are input into the §9.4.3 algorithms where top offset property refer to left offset property in the layout rules and where bottom offset property refer to right offset property in the layout rules.
*/

  div
    {
      background-color: green;
      bottom: 80px;
      height: 80px;
      position: relative;
      top: 80px;
      width: 80px;
    }

  /*
  Here, 'top' should win, 'bottom' should be ignored and the used bottom value should become -'top'.
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
