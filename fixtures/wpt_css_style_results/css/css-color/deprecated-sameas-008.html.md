# css/css-color/deprecated-sameas-008.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-008.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: ButtonBorder; width: 12em; height: 6em; }
    @supports (color: InactiveBorder) {
        .test { background-color: InactiveBorder; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
