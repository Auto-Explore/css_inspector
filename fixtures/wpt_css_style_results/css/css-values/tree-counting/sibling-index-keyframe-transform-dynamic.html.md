# css/css-values/tree-counting/sibling-index-keyframe-transform-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-transform-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      transform: translateX(calc(10px * sibling-index()));
    }
    to {
      transform: none;
    }
  }
  #target {
    animation: --anim 1000s step-end;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
