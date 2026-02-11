# css/css-flexbox/table-as-item-cross-size.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/table-as-item-cross-size.html"
}
```

## style[0]

```css

.container {
    display: flex;
    flex-direction: column;
    height: 100px;
    width: 50px;
}
.first {
    flex: 1 1 auto;
    background-color: blue;
}
.test {
    flex: 0 0 auto;
    background-color: green;
    display: flex;
}
td {
    padding: 23px;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “flex”.",
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
