# css/css-view-transitions/content-with-inline-child-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-with-inline-child-ref.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 100px;
  overflow-clip-margin: 500px;
  contain: paint;
  view-transition-name: target;
  background-color: grey;
}

#child {
  position: relative;
  left: 100px;
  top: 100px;
  color: lightgreen;
  background-color: darkgreen;
}

#innerchild {
  position: relative;
  left: 100px;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
