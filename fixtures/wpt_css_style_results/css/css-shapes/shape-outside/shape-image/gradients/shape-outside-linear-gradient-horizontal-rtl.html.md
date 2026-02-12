# css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-horizontal-rtl.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside/shape-image/gradients/shape-outside-linear-gradient-horizontal-rtl.html"
}
```

## style[0]

```css

    .container {
      inline-size: 100px;
      block-size: 200px;
      background-color: red;
      font-family: Ahem;
      font-size: 50px;
      line-height: 1;
      direction: rtl;
      color: green;
    }
    #float-left {
      float: left;
      inline-size: 100px;
      block-size: 200px;
      background: linear-gradient(to right, green 50%, transparent 50%);
      shape-outside: linear-gradient(to right, green 50%, transparent 50%);
    }
    #float-right {
      float: right;
      inline-size: 100px;
      block-size: 200px;
      background: linear-gradient(to left, green 50%, transparent 50%);
      shape-outside: linear-gradient(to left, green 50%, transparent 50%);
    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
