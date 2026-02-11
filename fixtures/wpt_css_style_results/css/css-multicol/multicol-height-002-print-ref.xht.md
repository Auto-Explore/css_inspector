# css/css-multicol/multicol-height-002-print-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-height-002-print-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    html, body { height: 100%; }
    * { margin: 0; }
    div {
      border: double blue 12px;
      height: 150%;
      padding: 0 12px;
    }
    div > p {
      border-right: solid orange 12px;
      border-left: solid orange 12px;
      width: 50%;
      margin: 0 auto;
      height: 100%;
    }
    blockquote {
      border: solid thick yellow;
    }  ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
