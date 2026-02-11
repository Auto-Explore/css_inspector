# css/css-box/margin-trim/flex-column-style-change-triggers-layout-block-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/flex-column-style-change-triggers-layout-block-end.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    width: min-content;
    flex-wrap: wrap;
    border: 1px solid black;
}
.initial-margin-trim {
    margin-trim: block-end;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
    margin-block-end: 10px;
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
