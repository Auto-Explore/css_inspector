# css/css-cascade/presentational-hints-rollback.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/presentational-hints-rollback.html"
}
```

## style[0]

```css

@layer {
  .revert-1 {
    width: revert;
    height: revert;
  }
  .revert-layer-1 {
    width: revert-layer;
    height: revert-layer;
  }
}

.revert-2 {
  width: revert;
  height: revert;
}
.revert-layer-2 {
  width: revert-layer;
  height: revert-layer;
}

.revert-3 {
  animation: revert-3 paused 2s -1s;
}
.revert-layer-3 {
  animation: revert-layer-3 paused 2s -1s;
}
@keyframes revert-3 {
  from, to {
    width: revert;
    height: revert;
  }
}
@keyframes revert-layer-3 {
  from, to {
    width: revert-layer;
    height: revert-layer;
  }
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
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
