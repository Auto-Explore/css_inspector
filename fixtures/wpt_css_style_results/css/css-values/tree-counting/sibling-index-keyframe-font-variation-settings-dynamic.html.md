# css/css-values/tree-counting/sibling-index-keyframe-font-variation-settings-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-values/tree-counting/sibling-index-keyframe-font-variation-settings-dynamic.html"
}
```

## style[0]

```css

  @keyframes --anim {
    from {
      font-variation-settings: "wght" sibling-index();
    }
    to {
      font-variation-settings: "wght" 10;
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
      "message": "Invalid value for property “font-variation-settings”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “font-variation-settings”.",
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
