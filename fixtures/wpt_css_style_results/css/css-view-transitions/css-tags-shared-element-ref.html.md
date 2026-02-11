# css/css-view-transitions/css-tags-shared-element-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/css-tags-shared-element-ref.html"
}
```

## style[0]

```css

div { contain: paint; }
#left {
  background: blue;
  width: 100px;
  height: 100px;
  position: absolute;
  top: 50px;
  left: 50px;
}
#right {
  width: 50px;
  height: 50px;
  background: green;
  position: absolute;
  top: 50px;
  left: 250px;
}
body { background: lightpink; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
