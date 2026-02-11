# css/css-color/deprecated-sameas-022.html

```json
{
  "format_version": 3,
  "file": "css/css-color/deprecated-sameas-022.html"
}
```

## style[0]

```css

    .test { background-color: red; width: 12em; height: 6em; }
    .ref { background-color: ButtonBorder; width: 12em; height: 6em; }
    @supports (color: WindowFrame) {
        .test { background-color: WindowFrame; }
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
