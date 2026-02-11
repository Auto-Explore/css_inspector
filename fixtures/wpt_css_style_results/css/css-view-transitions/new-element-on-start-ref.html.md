# css/css-view-transitions/new-element-on-start-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/new-element-on-start-ref.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  contain: paint;
  position: absolute;
  top: 50px;
  background: black;
  left: 200px;
  filter: invert(1);
}
body { background: lightpink; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
