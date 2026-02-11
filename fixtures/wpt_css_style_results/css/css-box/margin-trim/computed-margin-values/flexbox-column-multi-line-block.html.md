# css/css-box/margin-trim/computed-margin-values/flexbox-column-multi-line-block.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-column-multi-line-block.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    width: 100px;
    height: 120px;
    margin-trim: block;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-block: 10px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “margin-trim”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
