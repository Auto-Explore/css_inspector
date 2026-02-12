# css/css-view-transitions/view-transition-waituntil-animation-manipulation.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-waituntil-animation-manipulation.html"
}
```

## style[0]

```css

:root { view-transition-name: none }

#target {
  width: 100px;
  height: 100px;
  background: green;
  view-transition-name: target;
}

#target.after {
  background: red;
}

::view-transition { background: lightpink; }
::view-transition-group(*) {
  animation-duration: 1ms;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
