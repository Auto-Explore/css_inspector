# css/css-images/gradient/gradient-eval-007.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-eval-007.html"
}
```

## style[0]

```css

   :root {
       --start: lch(60% 60 70);
       --end: lch(60% 60 290);
       --t: 0.5;
       --big: 131070000px;
   }
   .test {
       width: 100px;
       height: 100px;
       background: linear-gradient(in lch, var(--start) calc(var(--big) * (0 - var(--t))), var(--end) calc(var(--big) * (1 - var(--t))));
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
