# css/css-box/margin-trim/computed-margin-values/flexbox-row-multi-line-block-start.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-row-multi-line-block-start.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: 100px;
    flex-wrap: wrap;
    margin-trim: block-start;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(1) {
    margin-block-start: 10px;
}
item:nth-child(2) {
    margin-block-start: -10px;
}
item:nth-child(3) {
    margin-block-start: 10px;
}
item:nth-child(4) {
    margin-block-start: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
