# css/css-contain/content-visibility/content-visibility-083.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-083.html"
}
```

## style[0]

```css

.spacer {
  width: 150px;
  height: 3000px;
  background: lightblue;
}
#container {
  width: 150px;
  height: 150px;
  background: red;
  content-visibility: hidden;
}

#target {
  width: 100px;
  height: 100px;
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
