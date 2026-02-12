# css/css-color/deprecated-sameas-003.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-003.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: Canvas; width: 12em; height: 6em; }
    @supports (color: AppWorkspace) {
        .test { background-color: AppWorkspace; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
