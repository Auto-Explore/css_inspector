# css/css-view-transitions/writing-mode-container-resize-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/writing-mode-container-resize-ref.html"
}
```

## style[0]

```css

#target {
  background: lightblue;
  height: 100%;
  aspect-ratio: 1 / 1;
}
#container {
  width: 100px;
  height: 50px;
  border: 1px solid black;
}
body {
  writing-mode: vertical-lr;
  background: pink;
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
