# css/css-overflow/overflow-body-propagation-014.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/overflow-body-propagation-014.html"
}
```

## style[0]

```css

body {
  overflow: hidden;
  width: 0px;
  height: 0px;
  border: solid red;
  border-width: 0 400px 200px 0;
  margin-bottom: 0;
}
body > div {
  background: green;
  width: 400px;
  height: 200px;
}
#clone {
  border-color: green;
  margin-top: 0;
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
