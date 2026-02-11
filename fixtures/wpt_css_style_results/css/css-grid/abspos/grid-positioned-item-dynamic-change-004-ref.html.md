# css/css-grid/abspos/grid-positioned-item-dynamic-change-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/abspos/grid-positioned-item-dynamic-change-004-ref.html"
}
```

## style[0]

```css

#grid {
  display: grid;
  width: 100px;
  height: 100px;
  position: relative;
  border: solid;
  grid-template: 50px 50px / 50px 50px;
}

#abspos {
  position: absolute;
  top: 15px;
  left: 15px;
  grid-area: 2 / 2 / 2 / 2;
  width: 20px;
  height: 20px;
  background: lime;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “grid-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
