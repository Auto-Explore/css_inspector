# css/css-view-transitions/named-element-with-fix-pos-child-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/named-element-with-fix-pos-child-ref.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  background: blue;
}
.child {
  width: 100px;
  height: 100px;
  position: fixed;
  top: 150px;
  left: 150px;
  background: grey;
}
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
