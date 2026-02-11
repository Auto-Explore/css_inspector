# css/css-transforms/transform-stacking-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-stacking-001.html"
}
```

## style[0]

```css

      body > div:first-child {
        transform: scale(1);
      }
      body > div:first-child > div {
        height: 100px;
        width: 100px;
        background: red;
        z-index: 1;
        position: relative;
      }
      body > div:last-child {
        height: 100px;
        width: 100px;
        background: lime;
        position: relative;
        bottom: 100px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
