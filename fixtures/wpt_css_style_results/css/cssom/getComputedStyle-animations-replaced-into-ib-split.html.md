# css/cssom/getComputedStyle-animations-replaced-into-ib-split.html

```json
{
  "format_version": 3,
  "file": "css/cssom/getComputedStyle-animations-replaced-into-ib-split.html"
}
```

## style[0]

```css

  @keyframes my-animation {
    from { color: green; }
    to { color: green; }
  }
  div {
    color: red;
    animation: my-animation 1s infinite linear paused;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
