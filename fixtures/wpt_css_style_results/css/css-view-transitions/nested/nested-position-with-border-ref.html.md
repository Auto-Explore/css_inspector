# css/css-view-transitions/nested/nested-position-with-border-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/nested-position-with-border-ref.html"
}
```

## style[0]

```css

body {
  background: pink;
}
#clipper {
  view-transition-group: contain;
  view-transition-name: clipper;

  border-width: 6px 10px 16px 20px;
  border-style: solid;
  border-color: green;

  height: 200px;
  width: 200px;
}

.item {
  view-transition-name: item;
  background: blue;
  position: relative;

  width: 100px;
  height: 100px;
  border: 1px solid black;
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-group”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
