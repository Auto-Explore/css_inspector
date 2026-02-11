# css/css-values/lh-unit-same-element-font-size-dependency.html

```json
{
  "format_version": 3,
  "file": "css/css-values/lh-unit-same-element-font-size-dependency.html"
}
```

## style[0]

```css

div {
    background-image: linear-gradient(green, green);
    background-repeat: no-repeat;
    background-size: 100px 1lh;
    line-height: 2em;
    font-size: 50px;
    width: 100px;
    color: transparent;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
