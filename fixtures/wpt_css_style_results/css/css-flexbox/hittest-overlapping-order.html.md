# css/css-flexbox/hittest-overlapping-order.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/hittest-overlapping-order.html"
}
```

## style[0]

```css

#container {
  width: 600px;
  display: flex;
}

#left {
  width: 300px;
  overflow: hidden;
  white-space: nowrap;
  background: rgba(200, 200, 200, 0.8);
  order: 0;
}

#right {
  width: 300px;
  background-color: rgba(0, 128, 0, 0.8);
  margin-left: -100px;
  order: 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
