# css/css-flexbox/gap-001-lr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-001-lr-ref.html"
}
```

## style[0]

```css

  body {
    writing-mode: vertical-lr;
  }

  section {
    background-color: green;
    block-size: 100px;
    display: flex;
  }
  section > div{
    background-color: grey;
    flex: 1 1 auto;
  }
  section > div:not(:first-child) {
    margin-inline-start: 20px;
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
