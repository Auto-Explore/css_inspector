# css/css-view-transitions/modify-style-via-cssom-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/modify-style-via-cssom-ref.html"
}
```

## style[0]

```css

#box {
  width: 100px;
  height: 100px;
  background: limegreen;
}
#box {
  transform: translateY(100px);
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
