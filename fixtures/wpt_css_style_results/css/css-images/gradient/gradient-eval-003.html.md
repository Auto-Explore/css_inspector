# css/css-images/gradient/gradient-eval-003.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-eval-003.html"
}
```

## style[0]

```css

   :root {
       --start: rgb(34.04% 57.84% 91.39%);      /* lab(60% 0 -50) */
       --end: rgb(64.07% 56.14% 19.72%);        /* lab(60% 0 50) */
       --t: 0.5;
       --big: 131070000px;
   }
   .test {
       width: 100px;
       height: 100px;
       background: linear-gradient(in lab, var(--start) calc(var(--big) * (0 - var(--t))), var(--end) calc(var(--big) * (1 - var(--t))));
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
