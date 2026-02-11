# css/css-anchor-position/anchor-size-in-flex-basis-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-size-in-flex-basis-crash.html"
}
```

## style[0]

```css

#dut1 {
  display: flex;
}
#dut2 {
  flex-basis: anchor-size(--a width, 10px);
}
#dut3 {
  flex-basis: auto;
  width: anchor-size(--a width, 10px);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
