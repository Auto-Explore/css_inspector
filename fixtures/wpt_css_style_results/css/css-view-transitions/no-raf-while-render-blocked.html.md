# css/css-view-transitions/no-raf-while-render-blocked.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/no-raf-while-render-blocked.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
  view-transition-name: target;
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
