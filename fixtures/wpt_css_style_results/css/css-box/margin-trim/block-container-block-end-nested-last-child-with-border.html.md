# css/css-box/margin-trim/block-container-block-end-nested-last-child-with-border.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-end-nested-last-child-with-border.html"
}
```

## style[0]

```css

.trim {
    margin-trim: block;
    outline: 1px solid blue;
}
container {
    display: block;
    width: min-content;
}
item {
    display: block;
    inline-size: 50px;
    block-size: 10px;
    background-color: green;
}
.collapsed {
    block-size: 0px;
    margin-block: 10px;
}
.border {
    block-size: auto;
    border: 10px solid black;
    margin-block-end: 25px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
