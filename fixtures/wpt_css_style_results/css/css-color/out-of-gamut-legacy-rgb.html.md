# css/css-color/out-of-gamut-legacy-rgb.html

```json
{
  "format_version": 3,
  "file": "css/css-color/out-of-gamut-legacy-rgb.html"
}
```

## style[0]

```css

  /*  See crbug.com/1483736.
      Values above 255 were being stored for legacy colors which was causing
      them to be rendered a if their alphas were greater.
  */
  body {
    background-color: black;
  }
  div {
    height: 100px;
    background-color: rgba(500, 500, 500, 0.5);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
