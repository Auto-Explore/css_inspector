# css/css-box/margin-trim/block-container-block-end-self-collapsing-item-has-larger-block-end-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-end-self-collapsing-item-has-larger-block-end-ref.html"
}
```

## style[0]

```css

container {
    display: block;
    margin-trim: block;
    margin-block-end: 10px;
}
item {
    display: block;
    width: 50px;
    height: 50px;
    background-color: green;
}
.collapsed {
    margin-block-end: 0px;
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
