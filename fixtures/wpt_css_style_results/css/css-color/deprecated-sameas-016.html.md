# css/css-color/deprecated-sameas-016.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-016.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: ButtonBorder; width: 12em; height: 6em; }
    @supports (color: ThreeDDarkShadow) {
        .test { background-color: ThreeDDarkShadow; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
