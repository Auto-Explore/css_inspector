# css/css-contain/content-visibility/content-visibility-064.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-064.html"
}
```

## style[0]

```css

.spacer {
  height: 1000px;
  background: lightblue;
}
#container {
  width: 150px;
  background: lightblue;
  contain-intrinsic-size: 50px 150px;
  contain: paint;
}
#child {
  width: 150px;
  height: 300px;
}
#target {
  position: absolute;
  bottom: 0;

  width: 50px;
  height: 50px;
  background: green;
}
.auto {
  content-visibility: auto;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
