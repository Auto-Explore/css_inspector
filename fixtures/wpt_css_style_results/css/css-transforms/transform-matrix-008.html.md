# css/css-transforms/transform-matrix-008.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-matrix-008.html"
}
```

## style[0]

```css

      div {
        height: 100px;
        width: 100px;
      }
      body > div {
        transform: matrix(1, 0, 0, 1, 50px, 0);
      }
      body > div > div {
        transform: matrix(1, 0, 0, 1, 50%, 0);
      }
      body > div > div > div {
        background: blue;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
