# css/css-view-transitions/no-css-animation-while-render-blocked.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/no-css-animation-while-render-blocked.html"
}
```

## style[0]

```css

@keyframes fade {
  from {
    opacity: 0;
  }
}

div {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: target;
}

.animated {
  animation: fade 0.5s;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
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
