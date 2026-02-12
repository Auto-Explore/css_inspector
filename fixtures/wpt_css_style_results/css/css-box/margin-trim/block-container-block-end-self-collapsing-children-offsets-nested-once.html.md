# css/css-box/margin-trim/block-container-block-end-self-collapsing-children-offsets-nested-once.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-end-self-collapsing-children-offsets-nested-once.html"
}
```

## style[0]

```css

.outer {
    margin-trim: block;
}
container {
    display: block;
}
item {
    display: block;
    margin-block-end: 40px;
    inline-size: 50px;
    block-size: 50px;
    background-color: green;
}
.collapsed {
    margin-block-start: 50px;
    block-size: 0px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
