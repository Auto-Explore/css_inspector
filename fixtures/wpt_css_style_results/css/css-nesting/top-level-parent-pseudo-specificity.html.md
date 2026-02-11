# css/css-nesting/top-level-parent-pseudo-specificity.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/top-level-parent-pseudo-specificity.html"
}
```

## style[0]

```css

  /* Note: at the top level, '&' matches like ':root'. */

  /* Should have zero specificity: */
  & { color: red; }
  /* Should also have zero specificity: */
  :where(&) { color: green; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
