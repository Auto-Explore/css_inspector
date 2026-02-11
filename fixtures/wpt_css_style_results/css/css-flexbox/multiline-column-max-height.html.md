# css/css-flexbox/multiline-column-max-height.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/multiline-column-max-height.html"
}
```

## style[0]

```css

.flex {
  display: flex;
  flex-flow: column wrap;
  max-height: 200px;
  background: blue;
  color: blue;
}

.item {
  flex: 0 0 auto;
  line-height: 20px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “flex”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
