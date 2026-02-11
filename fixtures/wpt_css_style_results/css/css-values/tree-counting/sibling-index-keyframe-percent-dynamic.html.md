# css/css-values/tree-counting/sibling-index-keyframe-percent-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-percent-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      text-size-adjust: calc(50% * sibling-index());
    }
    to {
      text-size-adjust: 90%;
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
