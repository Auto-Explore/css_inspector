# css/css-color/deprecated-sameas-017.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-017.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: ButtonFace; width: 12em; height: 6em; }
    @supports (color: ThreeDFace) {
        .test { background-color: ThreeDFace; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
