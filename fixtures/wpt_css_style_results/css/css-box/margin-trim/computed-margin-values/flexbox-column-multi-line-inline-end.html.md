# css/css-box/margin-trim/computed-margin-values/flexbox-column-multi-line-inline-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-column-multi-line-inline-end.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    width: min-content;
    height: 100px;
    margin-trim: inline-end;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-inline-end: 10px;
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
