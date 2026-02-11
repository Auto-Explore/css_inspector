# css/css-images/gradient/gradient-eval-010.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-eval-010.html"
}
```

## style[0]

```css

   .test {
       width: 100px;
       height: 100px;
       /* none should not resolve to 40% */
       /* hsl(180 50% 40%) in rgb is color(srgb 0.2 0.6 0.6) which is 60% of b component */
       background: linear-gradient(90deg in srgb, color(srgb 0 0 none), hsl(180 50% 40%));
   }
 
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
