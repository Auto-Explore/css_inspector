# css/css-values/chrome-interpolation-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-values/chrome-interpolation-crash.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      -webkit-perspective-origin-y: calc(10px * sibling-index());
    }
  }
  #target {
    animation: --anim 1s;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
