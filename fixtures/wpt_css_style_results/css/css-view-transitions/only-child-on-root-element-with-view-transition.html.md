# css/css-view-transitions/only-child-on-root-element-with-view-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/only-child-on-root-element-with-view-transition.html"
}
```

## style[0]

```css

::view-transition {
  background-color: red;
}

html:only-child {
  background-color: blue;
}

:root:only-child {
  background-color: blue;
}

:only-child {
  background-color: blue;
}

.foo:only-child {
  background-color: blue;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
