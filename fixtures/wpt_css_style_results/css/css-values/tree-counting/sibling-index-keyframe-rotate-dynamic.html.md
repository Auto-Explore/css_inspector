# css/css-values/tree-counting/sibling-index-keyframe-rotate-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-rotate-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      rotate: x calc(10deg * sibling-index());
    }
    to {
      rotate: x 90deg;
    }
  }
  #target {
    animation: --anim 1000s step-end;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “rotate”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “rotate”.",
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
