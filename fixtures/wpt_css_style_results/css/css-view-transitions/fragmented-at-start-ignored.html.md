# css/css-view-transitions/fragmented-at-start-ignored.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/fragmented-at-start-ignored.html"
}
```

## style[0]

```css

:root {
  scrollbar-width: none;
}
#spacer {
  width: 100px;
  height: 950px;
  background: lightgreen;
}
#container {
  width: 500px;
  columns: 2;
  height: 500px;
}
#target {
  width: 200px;
  height: 200px;
  background: green;
  view-transition-name: target;
}
#unrelated {
  width: 100px;
  height: 100px;
  background: lightblue;
  view-transition-name: unrelated;
}

::view-transition {
  background: pink;
}
::view-transition-group(root) {
  animation-duration: 500s;
  visibility: hidden;
}
::view-transition-group(target) {
  border: 1px solid black;
}
```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
