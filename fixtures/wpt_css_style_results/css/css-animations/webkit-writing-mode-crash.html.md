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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
