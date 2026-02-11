# css/CSS2/normal-flow/blocks-026.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/blocks-026.xht"
}
```

## style[0]

```css

   div, div p, div input { margin: 0; border: 0; padding: 0; }
   div { height: 0; }
   div p, div input { width: 50%; height: 300px; }
   div p { background: red; }
   div input { border: 100px solid green; background: green; display: block; }
   /* input should be at LEAST 50% wide, it may even be 50% + 200px if
      the browser doesn't assume box-sizing: border-box. However,
      there is no way the input can be narrower than the p. */
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
