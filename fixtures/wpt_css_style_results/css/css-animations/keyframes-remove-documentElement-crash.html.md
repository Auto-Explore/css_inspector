# css/css-animations/keyframes-remove-documentElement-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/keyframes-remove-documentElement-crash.html"
}
```

## style[0]

```css

  @keyframes anim {
    from { color: pink }
    to { color: purple }
  }
  div {
    animation: notfound 1s;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
