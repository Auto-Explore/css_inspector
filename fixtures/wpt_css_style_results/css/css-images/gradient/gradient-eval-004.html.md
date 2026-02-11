# css/css-images/gradient/gradient-eval-004.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-eval-004.html"
}
```

## style[0]

```css

   :root {
       --start: rgb(0% 0% 100%);
       --end: rgb(0% 0% none); /* none should resolve to 0  as rgb don't have analogous components with oklab */
       --t: 0.5;
       --big: 131070000px;
   }
   .test {
       width: 100px;
       height: 100px;
       background: linear-gradient(var(--start) calc(var(--big) * (0 - var(--t))), var(--end) calc(var(--big) * (1 - var(--t))));
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
