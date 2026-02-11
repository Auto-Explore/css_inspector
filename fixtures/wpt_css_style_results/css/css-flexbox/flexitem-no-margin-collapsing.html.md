# css/css-flexbox/flexitem-no-margin-collapsing.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexitem-no-margin-collapsing.html"
}
```

## style[0]

```css

.flexbox {
    background-color: lightgrey;
}
.flexbox p {
    height: 100px;
    width: 100px;
    margin: 10px;
    background-color: blue;
}
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
