# css/css-pseudo/target-text-dynamic-003.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/target-text-dynamic-003.html"
}
```

## style[0]

```css

  span::target-text {
    background-color: cyan;
  }

  span {
    background-color: magenta;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
