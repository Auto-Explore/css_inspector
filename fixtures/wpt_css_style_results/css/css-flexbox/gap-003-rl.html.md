# css/css-flexbox/gap-003-rl.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/gap-003-rl.html"
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
    flex-wrap: wrap;
    flex-direction: column;
  }
  section > div{
    background-color: grey;
    width: calc(50% - 10px);
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
