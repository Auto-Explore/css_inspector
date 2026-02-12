# css/css-color/deprecated-sameas-014.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-014.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: CanvasText; width: 12em; height: 6em; }
    @supports (color: MenuText) {
        .test { background-color: MenuText; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
