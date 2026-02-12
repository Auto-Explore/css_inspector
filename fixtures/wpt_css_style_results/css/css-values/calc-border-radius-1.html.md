# css/css-values/calc-border-radius-1.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-border-radius-1.html"
}
```

## style[0]

```css


p {
  /* We use powers of two here to avoid floating-point issues.
     See bug 590181. */

  height: 256px;
  width: 512px;
  background: blue;
  border-radius: calc((1/32 * 100%) + 5px)
                 calc((1/64 * 100%) - 2px)
                 calc(10px + (1/256 * 100%))
                 calc((1/16 * 100%) - 3px) /
                 calc((1/32 * 100%) - (1px + (1/128 * 100%)))
                 calc(1/16 * 100%)
                 calc(10px)
                 3px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
