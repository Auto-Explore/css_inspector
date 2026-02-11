# css/css-view-transitions/animating-new-content-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/animating-new-content-ref.html"
}
```

## style[0]

```css

#target {
  width:100px;
  height:100px;
  background: grey;
  will-change: transform;
}

.child {
  width: 50px;
  height: 50px;
  will-change: transform;
  background: green;
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
