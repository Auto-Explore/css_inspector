# css/css-view-transitions/view-transition-types-removed.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-types-removed.html"
}
```

## style[0]

```css


body { background: lightpink; }
html:active-view-transition-type(type-name) #target {
  background: red;
}

#target {
  view-transition-name: target;
  background: green;
  width: 100px;
  height: 100px;
}

html::view-transition-group(root) {
  display: none;
}

html::view-transition { background: white; }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
