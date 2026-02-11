# css/css-flexbox/gap-002-rtl-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-002-rtl-ref.html"
}
```

## style[0]

```css

  body {
    direction: rtl;
  }

  section {
    background-color: green;
    block-size: 200px;
    display: flex;
    flex-direction: column;
  }
  section > div{
    background-color: grey;
    flex: 1 1 auto;
  }
  section > div:not(:first-child) {
    margin-block-start: 20px;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
