# css/css-box/margin-trim/flex-column-inline-multiline-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-column-inline-multiline-ref.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    height: 100px;
    flex-direction: column;
    flex-wrap: wrap;
    gap: 0px 30px;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “gap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
