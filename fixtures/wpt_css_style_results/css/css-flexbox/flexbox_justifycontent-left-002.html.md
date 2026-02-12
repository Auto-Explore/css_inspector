# css/css-flexbox/flexbox_justifycontent-left-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-left-002.html"
}
```

## style[0]

```css

div {
  background: blue;
  margin: 1em 0;
  display: flex;
  width: 12em;
  height: 10em;
  justify-content: left;
}
div#column {
  flex-direction: column;
}
div#column-reverse {
  flex-direction: column-reverse;
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
