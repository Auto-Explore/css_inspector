# css/mediaqueries/mq-calc-006.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-calc-006.html"
}
```

## style[0]

```css

p { font-size: 16px; }
#target {
  width: 100px;
  height: 100px;
  background-color: green;
}
@media (min-width: calc(100in)) {
  /* Should not be selected */
  #target { background-color: red }
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
