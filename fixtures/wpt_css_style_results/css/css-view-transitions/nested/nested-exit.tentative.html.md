# css/css-view-transitions/nested/nested-exit.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/nested-exit.tentative.html"
}
```

## style[0]

```css

        ::view-transition-old(*),
        ::view-transition-new(*),
        ::view-transition-group(*) {
            animation-play-state: paused;
        }

        ::view-transition-group-children(*) {
          background: inherit;
        }

        ::view-transition-group(parent) {
            background: green;
            visibility: hidden;
        }
        ::view-transition-group(child) {
            visibility: visible;
            background: inherit;
        }

        body {
            margin: 0;
        }

        div {
            top: 50px;
            left: 50px;
            width: 100px;
            height: 100px;
            position: absolute;
        }

        .parent {
            view-transition-name: parent;
        }

        .child {
            view-transition-name: child;
            view-transition-group: parent;
        }

        .vt-new .child {
            display: none;
        }
    
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
