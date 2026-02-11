# css/css-overflow/logical-overflow-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/logical-overflow-001.html"
}
```

## style[0]

```css

#d1, #d2 {
  overflow-block: hidden;
  overflow-inline: scroll
}
#d1 {
  writing-mode: horizontal-tb;
}
#d2 {
  writing-mode: vertical-rl;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “overflow-block”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “overflow-inline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
