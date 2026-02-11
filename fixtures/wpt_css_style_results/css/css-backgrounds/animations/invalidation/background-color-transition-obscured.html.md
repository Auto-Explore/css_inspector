# css/css-backgrounds/animations/invalidation/background-color-transition-obscured.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/invalidation/background-color-transition-obscured.html"
}
```

## style[0]

```css

.parent {
  width: 100px;
  height: 100px;
  background-color: rgb(0, 200, 0);
  transition: background-color 200000ms steps(2) -99999.9ms;
}
.container {
  width: 100px;
  height: 100px;
  background-color: rgb(0, 0, 200);
  transition: background-color 200000ms steps(2) -99999.9ms;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
