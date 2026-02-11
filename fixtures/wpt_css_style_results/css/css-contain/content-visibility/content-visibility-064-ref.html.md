# css/css-contain/content-visibility/content-visibility-064-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-064-ref.html"
}
```

## style[0]

```css

.spacer {
  height: 1000px;
  background: lightblue;
}
#container {
  position: relative;
  width: 150px;
  background: lightblue;
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
