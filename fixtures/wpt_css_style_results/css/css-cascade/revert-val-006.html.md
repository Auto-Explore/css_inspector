# css/css-cascade/revert-val-006.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-val-006.html"
}
```

## style[0]

```css

  @keyframes test {
    from { margin-top: 0px; }
    50% { margin-top: revert; }
    to { margin-top: 0px; }
  }
  #h1 {
    margin-top: 0px;
    animation: test linear 1000s -500s paused;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
