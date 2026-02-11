# css/css-values/tree-counting/sibling-index-keyframe-font-weight-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-font-weight-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      font-weight: calc(100 * sibling-index());
    }
    to {
      font-weight: 600;
    }
  }
  #target {
    animation: --anim 1000s step-end;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
