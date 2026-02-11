# css/CSS2/syntax/escaped-url-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/escaped-url-001.xht"
}
```

## style[0]

```css

    div {background: red; width: 5em; height: 1em}
    #div0 {background: url(support/1x1-green.png)}
    #div1 {background: \url(support/1x1-green.png)}
    #div2 {background: U\Rl(support/1x1-green.png)}
    #div3 {background: U\R\l(support/1x1-green.png)}
    #div4 {background: \55Rl(support/1x1-green.png)}
    #div5 {background: \000075 rl(support/1x1-green.png)}
  
```

```json
{
  "errors": 5,
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
