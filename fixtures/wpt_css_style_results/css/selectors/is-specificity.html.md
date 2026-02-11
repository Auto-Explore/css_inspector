# css/selectors/is-specificity.html

```json
{
  "format_version": 3,
  "file": "css/selectors/is-specificity.html"
}
```

## style[0]

```css

      .b.c + .d + .q.r + .s + #target {
        font-size: 10px;
        height: 10px;
        width: 10px;
      }
      :is(.a, .b.c + .d, .q) + :is(* + .p, .q.r + .s, * + .t) + #target {
        height: 20px;
        width: 20px;
      }
      .b.c + .d + .q.r + .s + #target {
        width: 30px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
