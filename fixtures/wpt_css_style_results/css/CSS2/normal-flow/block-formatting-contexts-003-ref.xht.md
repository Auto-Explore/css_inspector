# css/CSS2/normal-flow/block-formatting-contexts-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-formatting-contexts-003-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border-collapse: collapse;
  width: 100%;
  }

  tbody
  {
  border-bottom: black solid 1px;
  border-top: black solid 1px;
  }

  thead {border-bottom: black solid 1px;}
  /*
  Necessary otherwise ( 0 + 1 ) divided by 2 may give unpredictable
  measurements affecting vertical alignment
  */

  tfoot {border-top: black solid 1px;}
  /*
  Necessary otherwise ( 0 + 1 ) divided by 2 may give unpredictable
  measurements affecting vertical alignment
  */

  /*
  "
  User agents must find a consistent rule for rounding off in the
  case of an odd number of discrete units (screen pixels, printer dots).
  "
  http://www.w3.org/TR/CSS21/tables.html#collapsing-borders
  */

  td {padding: 0px;}
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
