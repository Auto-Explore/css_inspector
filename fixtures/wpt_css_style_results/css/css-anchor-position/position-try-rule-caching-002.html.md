# css/css-anchor-position/position-try-rule-caching-002.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-rule-caching-002.html"
}
```

## style[0]

```css

  @position-try --try {
    position-area: right top;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

  @position-try --try {
    position-area: right top;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
