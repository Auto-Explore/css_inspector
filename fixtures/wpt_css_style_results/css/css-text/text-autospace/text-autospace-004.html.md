# css/css-text/text-autospace/text-autospace-004.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-autospace/text-autospace-004.html"
}
```

## style[0]

```css

#container {
  writing-mode: sideways-lr;
}
.test {
  font-family: Ahem;
  font-size: 40px;
}
.normal {
  text-autospace: normal;
  /* These properties have no effect in sideways-* writing modes,
     so should not disable the auto-space insertion: */
  text-orientation: upright;
  text-combine-upright: all;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
