# css/css-writing-modes/overconstrained-rel-pos-ltr-top-bottom-vrl-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/overconstrained-rel-pos-ltr-top-bottom-vrl-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      direction: rtl;
    }

  p
    {
      position: absolute;
      right: 16px;
      top: -8px;
    }

  img
    {
      vertical-align: top;
    }

  img + img
    {
      position: relative;
      right: 16px;
      top: 80px;
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
