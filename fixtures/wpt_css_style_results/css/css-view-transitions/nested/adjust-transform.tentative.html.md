# css/css-view-transitions/nested/adjust-transform.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested/adjust-transform.tentative.html"
}
```

## style[0]

```css

    body {
        margin: 0;
    }
    div {
        position: absolute;
        top: 50px;
        left: 50px;
        width: 100px;
        height: 100px;
    }

    .parent {
        view-transition-name: parent;
    }

    .child {
        view-transition-name: child;
        view-transition-group: parent;
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
