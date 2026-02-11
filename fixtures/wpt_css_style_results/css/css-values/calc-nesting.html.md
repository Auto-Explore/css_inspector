# css/css-values/calc-nesting.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-nesting.html"
}
```

## style[0]

```css

#parent { width: 200px; }
#div1 { width: calc(calc(50px)); }
#div2 { width: calc(calc(60%) - 20px); }
#div3 { width: calc(calc(3 * 25%)); }
#div4 {
    --width: calc(10% + 30px);
    width: calc(2 * var(--width));
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
