# css/css-box/margin-trim/block-container-block-end-self-collapsing-children-offsets.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-end-self-collapsing-children-offsets.html"
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
