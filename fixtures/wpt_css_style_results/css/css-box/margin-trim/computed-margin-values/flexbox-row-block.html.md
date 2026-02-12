# css/css-box/margin-trim/computed-margin-values/flexbox-row-block.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-row-block.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    margin-trim: block;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-inline: 10px;
}
item:nth-child(1) {
    margin-block: 10px;
}
item:nth-child(2) {
    margin-block: -10px;
}
item:nth-child(3) {
    margin-block: 30%;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
