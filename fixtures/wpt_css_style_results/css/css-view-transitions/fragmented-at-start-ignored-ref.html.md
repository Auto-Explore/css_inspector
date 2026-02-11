# css/css-view-transitions/fragmented-at-start-ignored-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fragmented-at-start-ignored-ref.html"
}
```

## style[0]

```css

:root {
  scrollbar-width: none;
}
body { background: pink }
#spacer {
  width: 100px;
  height: 950px;
  background: lightgreen;
}
#container {
  width: 500px;
  columns: 2;
  height: 500px;
  visibility: hidden;
}
#target {
  width: 200px;
  height: 200px;
  background: green;
}
#unrelated {
  width: 100px;
  height: 100px;
  background: lightblue;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
