# css/css-animations/animation-css-variable-in-keyframe-adjusted.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/animation-css-variable-in-keyframe-adjusted.html"
}
```

## style[0]

```css


@keyframes anim {
    from { margin-left: var(--margin-left) }
    to   { margin-left: calc(var(--margin-left) * 2) }
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
