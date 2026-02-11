# css/css-cascade/revert-layer-011.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-layer-011.html"
}
```

## style[0]

```css

#target {
  width: var(--x);
  --x: 150px;
  height: 100px;
  background-color: green;
  animation: anim linear 2s -1s paused;
}

@property --x {
  syntax: '<length>';
  initial-value: 0px;
  inherits: false;
}

@keyframes anim {
  from { --x: 50px; }
  to { --x: revert-layer; }
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
