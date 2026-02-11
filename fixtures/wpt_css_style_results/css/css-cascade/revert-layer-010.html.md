# css/css-cascade/revert-layer-010.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-010.html"
}
```

## style[0]

```css

#target {
  width: 150px;
  height: 100px;
  background-color: green;
  animation: anim linear 2s -1s paused;
}

@keyframes anim {
  from { width: 50px; }
  to { width: revert-layer; }
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
