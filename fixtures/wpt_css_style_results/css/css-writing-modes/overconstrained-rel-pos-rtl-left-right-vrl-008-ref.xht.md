# css/css-writing-modes/overconstrained-rel-pos-rtl-left-right-vrl-008-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/overconstrained-rel-pos-rtl-left-right-vrl-008-ref.xht"
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
      bottom: -8px;
      position: absolute;
      right: 16px;
    }

  img
    {
      vertical-align: bottom;
    }

  img + img
    {
      position: relative;
      right: 96px;
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
