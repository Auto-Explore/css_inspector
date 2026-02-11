# css/css-view-transitions/pseudo-computed-style-stays-in-sync-with-new-element.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/pseudo-computed-style-stays-in-sync-with-new-element.html"
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
