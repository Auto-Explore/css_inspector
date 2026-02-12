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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
