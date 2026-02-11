# css/css-writing-modes/overconstrained-rel-pos-rtl-left-right-vlr-009-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/overconstrained-rel-pos-rtl-left-right-vlr-009-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  p
    {
      bottom: -8px;
      left: 16px;
      position: absolute;
    }

  img
    {
      vertical-align: bottom;
    }

  img + img
    {
      left: 96px;
      position: relative;
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
