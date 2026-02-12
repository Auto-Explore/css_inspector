# css/css-overflow/overflow-body-propagation-016.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-body-propagation-016.html"
}
```

## style[0]

```css

html {
  scrollbar-color: red red;
}
body {
  overflow: scroll;
  width: 0px;
  height: 0px;
  border: solid red;
  border-width: 0 400px 400px 0;
}
body > div {
  overflow: hidden;
  background: green;
  width: 400px;
  height: 400px;
}
#clone {
  border-color: green;
}
#clone > div {
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
