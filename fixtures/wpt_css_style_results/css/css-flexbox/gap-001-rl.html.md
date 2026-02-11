# css/css-flexbox/gap-001-rl.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-001-rl.html"
}
```

## style[0]

```css

  body {
    writing-mode: vertical-rl;
  }

  section {
    background-color: green;
    block-size: 100px;
    display: flex;
    gap: 20px;
  }
  section > div{
    background-color: grey;
    flex: 1 1 auto;
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
