# css/css-color/relative-currentcolor-rgb-02.html

```json
{
  "format_version": 3,
  "file": "css/css-color/relative-currentcolor-rgb-02.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: yellow;
  background-color: rgb(from currentColor g r b);
}
div div {
  color: #800000;
  background-color: inherit;
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
