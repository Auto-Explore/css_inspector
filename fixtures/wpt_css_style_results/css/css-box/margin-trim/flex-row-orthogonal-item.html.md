# css/css-box/margin-trim/flex-row-orthogonal-item.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-row-orthogonal-item.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    margin-trim: inline
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 100px;
}
.orthogonal {
    writing-mode: vertical-rl;
}
item:first-child {
    margin-block-end: 10px;
}
item:last-child {
    margin-inline-end: 10px;
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
