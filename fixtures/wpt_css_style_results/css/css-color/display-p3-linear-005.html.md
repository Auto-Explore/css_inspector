# css/css-color/display-p3-linear-005.html

```json
{
  "format_version": 3,
  "file": "css/css-color/display-p3-linear-005.html"
}
```

## style[0]

```css

    body { background-color: grey; }
    .test { background-color: red; width: 12em; height: 6em; margin-top: 0; }
    .ref { background-color: yellow; width: 12em; height: 6em; margin-bottom: 0; }
    /* sRGB yellow converted to display-p3-linear */
    .test { background-color: color(display-p3-linear 1 1 0.0895); }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
