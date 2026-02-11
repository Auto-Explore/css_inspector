# css/css-view-transitions/inline-child-with-filter-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/inline-child-with-filter-ref.html"
}
```

## style[0]

```css

body { margin : 0; }
#target {
  width: 100px;
  height: 100px;
  background-color: grey;
  overflow-clip-margin: 40px;
  contain: paint;
  view-transition-name: target;
}

#child {
  position: relative;
  left: 100px;
  top: 100px;
  color: lightgreen;
  background-color: darkgreen;
  filter: blur(30px);
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "“overflow-clip-margin” is not supported by Safari.",
      "severity": "Warning"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 1
}
```
