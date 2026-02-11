# css/css-contain/content-visibility/spacer-and-container-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/spacer-and-container-ref.html"
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
  contain: style layout;
  width: 150px;
  height: 150px;
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
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
