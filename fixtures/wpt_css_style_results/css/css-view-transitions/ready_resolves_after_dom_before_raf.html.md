# css/css-view-transitions/ready_resolves_after_dom_before_raf.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/ready_resolves_after_dom_before_raf.html"
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
.green {
  background: green;
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
