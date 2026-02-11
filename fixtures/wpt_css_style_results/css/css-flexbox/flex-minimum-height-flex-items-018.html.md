# css/css-flexbox/flex-minimum-height-flex-items-018.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-minimum-height-flex-items-018.html"
}
```

## style[0]

```css

.flexbox {
  display: flex;
  width: 100px;
  height: 0;
  flex-direction: column;
}
.item {
  /* Because the flexbox has height:0, this would flex-shrink to 0. So we rely
   * on min-height: auto to get a nonzero height.  */
  height: 100px;
  background: green;
}
.percentage {
  height: 100%;
}
.fixed {
  height: 100px;
}

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
