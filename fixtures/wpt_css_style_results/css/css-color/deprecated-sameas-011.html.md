# css/css-color/deprecated-sameas-011.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-011.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: Canvas; width: 12em; height: 6em; }
    @supports (color: InfoBackground) {
        .test { background-color: InfoBackground; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
