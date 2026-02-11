# css/css-writing-modes/text-indent-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-indent-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      color: green;
      font: 80px/1 Ahem; /* computes to 80px/80px */
      height: 320px;
      width: 320px;
    }

  div
    {
      background: red url("support/bg-red-1col-3row-320x320.png");
      direction: rtl;
      text-indent: 80px;
      writing-mode: vertical-lr;
    }
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
