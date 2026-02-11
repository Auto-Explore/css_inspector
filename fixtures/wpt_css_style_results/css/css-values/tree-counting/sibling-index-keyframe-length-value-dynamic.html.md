# css/css-values/tree-counting/sibling-index-keyframe-length-value-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-length-value-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      top: calc(100px * sibling-index());
    }
    to {
      top: 0px;
    }
  }
  #target {
    animation: --anim 1000s step-end;
    position: absolute;
    top: 13px;
    width: 100px;
    height: 100px;
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
