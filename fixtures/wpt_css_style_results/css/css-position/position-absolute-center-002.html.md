# css/css-position/position-absolute-center-002.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-center-002.html"
}
```

## style[0]

```css

#containing-block {
  position: relative;
  width: 100px;
  height: 100px;
  background: red;
  display: flex;
}

#containing-block > div {
  flex-grow: 1;
}

#inner-flex {
  display: flex;
  align-items: center;
}

span {
  display: inline-block;
  inline-size: 50px;
  block-size: 10px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
