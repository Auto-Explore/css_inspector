# css/css-view-transitions/web-animation-pseudo-incorrect-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/web-animation-pseudo-incorrect-name.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: blue;
  view-transition-name: target;
  contain: paint;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
