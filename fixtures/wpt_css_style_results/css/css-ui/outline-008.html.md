# css/css-ui/outline-008.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/outline-008.html"
}
```

## style[0]

```css

body > div {
  width: 100px;
  height: 100px;
  background: red;

  font-size: 10px;

  /* stacking  of outlines is undefined, so we cannot reliably to have an outline hide another outline.
   We are using a border instead,
   but still need to create an (invisible) outline to make sure that outline-width does not compute to 0. */
  outline: solid 1em transparent;
  border: solid 1em red;
  margin-left: -10px;
  margin-top: -10px;

}
div > div {
  width: 100px;
  height: 100px;
  background: green;

  font-size: 5px;
  outline-width: inherit;
  outline-style: solid;
  outline-color: white;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
