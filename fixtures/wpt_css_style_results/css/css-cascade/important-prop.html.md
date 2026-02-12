# css/css-cascade/important-prop.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/important-prop.html"
}
```

## style[0]

```css

  @keyframes override {
    from, to {
      background: #f00; color: green;
      border-color: green; border-color: red !important;
    }
  }

  .square {
    color:#00f;
    animation: override 1s infinite;
    width: 80px;
    height: 80px;
    border: 10px solid red;
    text-align: center;
  }
  div {
    background-color:green !important;
    color: red;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
