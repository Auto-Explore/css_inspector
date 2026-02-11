# css/css-contain/content-visibility/content-visibility-051.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-051.html"
}
```

## style[0]

```css

#container {
  content-visibility: hidden;
  width: 150px;
  height: 150px;
  background: lightblue;
}
.inline {
  display: inline;
}
#child {
  width: 50px;
  height: 50px;
  background: red;
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
