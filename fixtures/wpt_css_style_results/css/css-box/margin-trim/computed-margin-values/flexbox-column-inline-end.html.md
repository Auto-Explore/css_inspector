# css/css-box/margin-trim/computed-margin-values/flexbox-column-inline-end.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/flexbox-column-inline-end.html"
}
```

## style[0]

```css

flexbox {
    display: flex;
    flex-direction: column;
    width: min-content;
    margin-trim: inline-end;
}
item {
    display: block;
    background-color: green;
    width: 50px;
    height: 50px;
}
item:nth-child(1) {
    margin-inline-end: 10px;
}
item:nth-child(2) {
    margin-inline-end: -10px;
}
item:nth-child(3) {
    margin-inline-end: 30%;
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
