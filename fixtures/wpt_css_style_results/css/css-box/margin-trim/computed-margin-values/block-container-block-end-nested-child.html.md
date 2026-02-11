# css/css-box/margin-trim/computed-margin-values/block-container-block-end-nested-child.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/block-container-block-end-nested-child.html"
}
```

## style[0]

```css

container {
    display: block;
    inline-size: min-content;
    margin-trim: block-end;
}
item {
    display: block;
    background-color: green;
    inline-size: 50px;
    block-size: 10px;
    margin-block-end: 10px;
}
.border {
    border: 1px solid black;
}
.collapsed {
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
