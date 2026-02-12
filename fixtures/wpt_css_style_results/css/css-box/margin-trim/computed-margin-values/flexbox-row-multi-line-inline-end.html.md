# css/css-box/margin-trim/computed-margin-values/flexbox-row-multi-line-inline-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-row-multi-line-inline-end.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: 110px;
    flex-wrap: wrap;
    margin-trim: inline-end;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(1) {
    margin-inline-end: 10px;
}
item:nth-child(2) {
    margin-inline-end: -10px;
}
item:nth-child(3) {
    margin-inline-end: 10px;
}
item:nth-child(4) {
    margin-inline-end: 50%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
