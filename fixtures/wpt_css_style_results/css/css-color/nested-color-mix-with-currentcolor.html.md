# css/css-color/nested-color-mix-with-currentcolor.html

```json
{
  "format_version": 3,
  "file": "css/css-color/nested-color-mix-with-currentcolor.html"
}
```

## style[0]

```css

  #parent {
    color: red;
    background-color: color-mix(in lch, color-mix(in lch, black, currentColor), black);
  }
  #child {
    color: black;
    background-color: inherit;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
