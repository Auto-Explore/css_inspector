# css/css-contain/content-visibility/content-visibility-024.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-024.html"
}
```

## style[0]

```css

div {
  background: blue;
  color: white;
}
img {
  width: 400px;
  height: 200px;
  background: lightblue;
  border: 1px solid black;
}
.hidden {
  content-visibility: hidden;
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
