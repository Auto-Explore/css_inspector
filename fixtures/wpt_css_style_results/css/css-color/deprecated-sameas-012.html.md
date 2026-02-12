# css/css-color/deprecated-sameas-012.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-012.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: CanvasText; width: 12em; height: 6em; }
    @supports (color: InfoText) {
        .test { background-color: InfoText; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
