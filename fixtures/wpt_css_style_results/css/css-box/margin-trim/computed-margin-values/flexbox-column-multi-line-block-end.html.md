# css/css-box/margin-trim/computed-margin-values/flexbox-column-multi-line-block-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-column-multi-line-block-end.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    width: 100px;
    height: 110px;
    margin-trim: block-end;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-block-end: 10px;
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
