# css/css-flexbox/gap-002-lr-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-002-lr-ref.html"
}
```

## style[0]

```css

  body {
    writing-mode: vertical-lr;
  }

  section {
    background-color: green;
    block-size: 200px;
    display: flex;
    flex-direction: column;
  }
  section > div{
    background-color: gray;
    flex: 1 1 auto;
  }
  section > div:not(:first-child) {
    margin-block-start: 20px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
