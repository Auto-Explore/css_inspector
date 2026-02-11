# css/css-contain/content-visibility/content-visibility-055.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-055.html"
}
```

## style[0]

```css

.spacer {
  width: 150px;
  height: 300vh;
  background: lightblue;
}
#container {
  width: 150px;
  height: 150px;
  background: green;
  content-visibility: auto;
}

#target {
  width: 150px;
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
