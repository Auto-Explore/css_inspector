# css/css-transforms/transform-input-014.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-input-014.html"
}
```

## style[0]

```css

      input {
        /* Margin to avoid overlap of translated inputs */
        margin: 10px;
      }
      p + input {
        transform: rotate(360deg);
      }
      p + input + input {
        transform: translateX(-10px);
      }
      p + input + input + input {
        transform: translateX(10px);
      }
      p + input + input + input + input {
        transform: translateY(-10px);
      }
      p + input + input + input + input + input {
        transform: translateY(10px);
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
