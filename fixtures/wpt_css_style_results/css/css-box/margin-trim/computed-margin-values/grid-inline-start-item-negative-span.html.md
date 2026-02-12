# css/css-box/margin-trim/computed-margin-values/grid-inline-start-item-negative-span.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/grid-inline-start-item-negative-span.html"
}
```

## style[0]

```css

grid {
    display: grid;
    width: min-content;
    outline: 1px solid black;
    grid-template-columns: repeat(2, auto);
    margin-trim: inline-start;
}
item {
    display: block;
    width: 50px;
    height: 50px;
    margin-inline-start: 10px;
    background-color: green;
}
.negative-line-number {
    width: 50px;
    grid-row: 2;
    grid-column: -3;
    background-color: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
