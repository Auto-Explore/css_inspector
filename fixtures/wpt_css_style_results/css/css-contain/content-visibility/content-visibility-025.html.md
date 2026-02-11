# css/css-contain/content-visibility/content-visibility-025.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-025.html"
}
```

## style[0]

```css

div {
  background: blue;
  color: white;
}
svg {
  border: 1px solid black;
  background: lightblue;
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
