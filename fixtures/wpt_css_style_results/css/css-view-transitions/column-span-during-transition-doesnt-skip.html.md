# css/css-view-transitions/column-span-during-transition-doesnt-skip.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/column-span-during-transition-doesnt-skip.html"
}
```

## style[0]

```css

#container {
  width: 500px;
  height: 500px;
}
.fragment {
  columns: 2;
}
#target {
  height: 200px;
  background: green;
  view-transition-name: target;
  column-span: all;
}

::view-transition {
  background: pink;
}
::view-transition-group(root) {
  animation-duration: 500s;
  visibility: hidden;
}
```

```json
{
  "errors": 4,
  "messages": [
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
    }
  ],
  "warnings": 0
}
```
