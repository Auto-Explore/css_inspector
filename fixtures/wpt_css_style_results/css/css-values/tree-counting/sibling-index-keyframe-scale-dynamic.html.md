# css/css-values/tree-counting/sibling-index-keyframe-scale-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-scale-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      scale: 0.7 calc(0.2 * sibling-index());
    }
    to {
      scale: 0.3 2;
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
      "message": "Invalid value for property “scale”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scale”.",
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
