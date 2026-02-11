# css/css-properties-values-api/animation/registered-neutral-keyframe.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/animation/registered-neutral-keyframe.html"
}
```

## style[0]

```css

@keyframes test {
  to { --x: to; }
}
#target {
  --x: underlying;
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
