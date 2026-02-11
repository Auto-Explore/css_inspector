# css/css-pseudo/highlight-cascade/highlight-currentcolor-painting-text-shadow-002.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-cascade/highlight-currentcolor-painting-text-shadow-002.html"
}
```

## style[0]

```css

div {
  color: lime;
  background: green;
  margin: 10px;
}
::highlight(below) {
  color: yellow;
  background: maroon;
}
::highlight(textshadow-currentcolor) {
  text-shadow: currentcolor 2px 2px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
