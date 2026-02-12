# css/css-box/margin-trim/flex-row-inline-multiline.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-row-inline-multiline.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: 100px;
    flex-wrap: wrap;
    margin-trim: inline;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(odd) {
    margin-inline-start: 25px;
}
item:nth-child(even) {
    margin-inline-end: 25px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
