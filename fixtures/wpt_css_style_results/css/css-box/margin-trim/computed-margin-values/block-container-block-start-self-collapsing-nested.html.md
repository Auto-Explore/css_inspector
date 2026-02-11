# css/css-box/margin-trim/computed-margin-values/block-container-block-start-self-collapsing-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/block-container-block-start-self-collapsing-nested.html"
}
```

## style[0]

```css

container {
    display: block;
    margin-trim: block;
    border: 1px solid black;
    margin-block-start: 10px;
}
item {
    display: block;
    inline-size: 50px;
    background-color: green;
}
.collapsed {
    margin-block-start: 50px;
    block-size: 0px
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
