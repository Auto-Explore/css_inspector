# css/css-flexbox/gap-004-rtl.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-004-rtl.html"
}
```

## style[0]

```css

  body {
    direction: rtl;
  }
  section {
    background-color: green;
    block-size: 100px;
    display: inline-flex;
    gap: 20px;
  }
  section > div{
    background-color: grey;
    color: white;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
