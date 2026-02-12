# css/css-flexbox/flexbox_justifycontent-right-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-right-001.html"
}
```

## style[0]

```css

div {
  background: blue;
  margin: 1em 0;
  height: 4em;
  width: 40em;
  display: flex;
  flex-direction: row;
  justify-content: right;
}
div + div {
  flex-direction: row-reverse;
}
span {
  background: white;
  padding: 0 2em;
  width: 4em;
  height: 2em;
}
span:nth-child(1) {background: yellow;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
