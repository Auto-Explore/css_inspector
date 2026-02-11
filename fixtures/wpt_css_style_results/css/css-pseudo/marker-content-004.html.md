# css/css-pseudo/marker-content-004.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-004.html"
}
```

## style[0]

```css

body { margin-left: 100px; }
li { background: grey; }
li::marker {
  content: "X";
  display: flex;
  width: 100px;
  align-content: start;
  background: lime;
}
span { font-size: 32pt; }
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
