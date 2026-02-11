# css/css-contain/content-visibility/content-visibility-037.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-037.html"
}
```

## style[0]

```css

#grid {
  display: grid;
  width: 150px;
  height: 150px;
  background: lightblue;
}
#positioned {
  position: absolute;
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
