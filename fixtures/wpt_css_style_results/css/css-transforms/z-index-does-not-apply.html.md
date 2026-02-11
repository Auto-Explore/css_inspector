# css/css-transforms/z-index-does-not-apply.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/z-index-does-not-apply.html"
}
```

## style[0]

```css

  #a, #b {
    background: green;
    width: 100px;
    height: 100px;
  }
  #a {
    background: red;
    z-index: 2;
    transform: translateX(0);
  }
  #b {
    z-index: 1;
    transform: translateY(-100px);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
