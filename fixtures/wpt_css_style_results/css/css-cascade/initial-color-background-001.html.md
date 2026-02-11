# css/css-cascade/initial-color-background-001.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/initial-color-background-001.html"
}
```

## style[0]

```css

body {
  background-color: white;
}
div {
  font-size: 100px;
}
.outer {
  color: red;
}
.inner {
  background-color: red;
}
.inner {
  color: initial;/* normally black, almost certainly not red */
  background-color: initial;/* transparent, so the body's white will show thru */
}
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
