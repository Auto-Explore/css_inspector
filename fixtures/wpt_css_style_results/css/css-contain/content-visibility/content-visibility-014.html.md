# css/css-contain/content-visibility/content-visibility-014.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-014.html"
}
```

## style[0]

```css

#container {
  width: 150px;
  height: 150px;
  background: lightblue;
  will-change: transform;
}
.hidden {
  content-visibility: hidden;
}
#child {
  width: 50px;
  height: 50px;
  background: green;
  will-change: transform;
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
