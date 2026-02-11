# css/css-position/position-absolute-semi-replaced-stretch-other.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-semi-replaced-stretch-other.html"
}
```

## style[0]

```css

.cb {
  position: relative;
  border: 3px solid black;
  height: 100px;
  width: 150px;
  display: inline-block;
  vertical-align: top;
  margin: 5px;
}
.wide {
  /* Some of the form controls are wider than others
     and need a wider CB in order to see if they're
     stretching or shrinking. */
  width: 350px;
}
.abs {
  margin: 0;
  position: absolute;
  box-sizing: border-box;
  top: 3px;
  right: 3px;
  bottom: 3px;
  left: 3px;
  outline: 2px solid lime;
  width: auto;
  height: auto;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
