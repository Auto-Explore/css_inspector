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
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-perspective-origin-y”.",
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
