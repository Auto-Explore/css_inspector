# css/css-animations/webkit-writing-mode-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/webkit-writing-mode-crash.html"
}
```

## style[0]

```css

  @keyframes test {
    from { -webkit-writing-mode: var(--x) }
    to { -webkit-writing-mode: var(--y) }
  }
  div {
    animation: test 1s;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “-webkit-writing-mode”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-writing-mode”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
