# css/css-view-transitions/set-universal-specificity-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/set-universal-specificity-ref.html"
}
```

## style[0]

```css

body {
  background: pink;
}
div {
  contain: paint;
  width: 100px;
  height: 100px;
  background: blue;
  border: 10px solid black;
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
