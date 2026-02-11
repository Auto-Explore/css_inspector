# css/css-overflow/overflow-body-propagation-015.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-body-propagation-015.html"
}
```

## style[0]

```css

body {
  overflow: hidden;
  width: 0px;
  height: 0px;
  border: solid green;
  border-width: 0 400px 200px 0;
  margin-bottom: 0;
}
body:not(#clone) {
  margin-top: 0;
}
body > div {
  background: red;
  width: 400px;
  height: 200px;
}
#clone {
  border-color: red;
  margin-top: revert;
}
#clone > div {
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
