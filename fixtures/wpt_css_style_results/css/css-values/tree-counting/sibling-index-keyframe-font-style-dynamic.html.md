# css/css-values/tree-counting/sibling-index-keyframe-font-style-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-font-style-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      font-style: oblique calc(5deg * sibling-index());
    }
    to {
      font-style: oblique 9deg;
    }
  }
  #target {
    animation: --anim 1000s step-end;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-style”.",
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
