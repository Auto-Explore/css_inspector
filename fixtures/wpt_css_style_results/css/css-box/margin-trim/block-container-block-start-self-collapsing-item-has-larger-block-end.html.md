# css/css-box/margin-trim/block-container-block-start-self-collapsing-item-has-larger-block-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-start-self-collapsing-item-has-larger-block-end.html"
}
```

## style[0]

```css

container {
    display: block;
    margin-trim: block;
    margin-block-start: 10px;
}
item {
    display: block;
    margin-block-start: 40px;
    width: 50px;
    height: 50px;
    background-color: green;
}
.collapsed {
    margin-block-end: 100px;
    height: 0px;
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
