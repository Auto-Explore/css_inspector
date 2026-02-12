# css/css-view-transitions/nested/render-element.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/render-element.tentative.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    div {
        position: absolute;
        top: 0;
        left: 0;
        width: 100px;
        height: 100px;
    }

    .parent {
        view-transition-name: parent;
    }

    .child {
        view-transition-name: child;
        view-transition-group: parent;
        top: 100px;
        left: 100px;
        background: green;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
