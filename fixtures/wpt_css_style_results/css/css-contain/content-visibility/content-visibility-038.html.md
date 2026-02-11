# css/css-contain/content-visibility/content-visibility-038.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-038.html"
}
```

## style[0]

```css

#container {
  background: lightgreen;
  contain: layout;
}
.hidden {
  content-visibility: hidden;
}
#sizer {
  width: 100px;
  height: 100px;
}
.child {
  width: 20px;
  height: 20%;
  background: cyan;
}
#spacer {
  width: 150px;
  height: 150px;
  background: green;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
