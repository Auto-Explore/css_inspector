# css/css-pseudo/before-dynamic-display-none.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/before-dynamic-display-none.html"
}
```

## style[0]

```css

  #id::before {
    content: "FAIL";
    position: absolute;
    width: 100px;
    height: 100px;
    background-color: red;
  }
  #id.none::before {
    display: none;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
