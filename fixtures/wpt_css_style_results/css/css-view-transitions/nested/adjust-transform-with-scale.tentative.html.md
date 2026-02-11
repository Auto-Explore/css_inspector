# css/css-view-transitions/nested/adjust-transform-with-scale.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/adjust-transform-with-scale.tentative.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    div {
        position: absolute;
        width: 25px;
        height: 25px;
        transform-origin: 0 0;
        scale: 2;
    }

    .parent {
        view-transition-name: parent;
        top: 50px;
        left: 50px;
    }

    .child {
        top: 25px;
        left: 25px;
        view-transition-name: child;
        view-transition-group: parent;
        background: green;
    }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-group”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
