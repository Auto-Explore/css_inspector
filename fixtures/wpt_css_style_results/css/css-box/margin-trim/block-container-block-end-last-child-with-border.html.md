# css/css-box/margin-trim/block-container-block-end-last-child-with-border.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/block-container-block-end-last-child-with-border.html"
}
```

## style[0]

```css

.trim {
    margin-trim: block;
}
container {
    display: block;
    width: min-content;
    outline: 1px solid blue;
}
item {
    display: block;
    inline-size: 50px;
    block-size: 25px;
}
.collapsed {
    block-size: 0px;
    margin-block: 14px;
}
.border {
    block-size: auto;
    border: 1px solid black;
    background-color: green;
    margin-block-end: 20px;
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
