# css/css-box/margin-trim/flex-block-end-trimmed-only.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-block-end-trimmed-only.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    width: min-content;
    border: 1px black solid;
    margin-trim: block-end;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-inline: 10px;
    margin-block: 10px;
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
