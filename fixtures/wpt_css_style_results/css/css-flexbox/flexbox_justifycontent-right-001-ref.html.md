# css/css-flexbox/flexbox_justifycontent-right-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox_justifycontent-right-001-ref.html"
}
```

## style[0]

```css

div {
  background: blue;
  margin: 1em 0;
  height: 4em;
  width: 40em;
}
span {
  display: inline-block;
  background: white;
  padding: 0 2em;
  width: 4em;
  height: 2em;
}
span:nth-child(1) {background: yellow; margin-left: 16em;}
span:nth-child(2) {background: pink;}
span:nth-child(3) {background: lightblue;}
#row-reverse span:nth-child(3) {background: yellow;}
#row-reverse span:nth-child(1) {background: lightblue;}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
