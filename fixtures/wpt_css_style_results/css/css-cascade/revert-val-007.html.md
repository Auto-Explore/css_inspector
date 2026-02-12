# css/css-cascade/revert-val-007.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-val-007.html"
}
```

## style[0]

```css

  @keyframes test {
    from { margin-top: revert; }
    to { margin-top: 100px; }
  }
  .anim {
    margin-top: 0px;
    animation: test linear 1s paused;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
